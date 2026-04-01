use crate::error::Error;
use crate::state::ServerState;
use crate::{auth, chat, time};
use elysium_rust::chat::v1::chat_service_server::ChatService;
use elysium_rust::chat::v1::{
    ChannelPermission, CreateChannelRequest, CreateChannelResponse, DeleteMessageRequest,
    DeleteMessageResponse, ReadMessagesRequest, ReadMessagesResponse, SendMessageRequest,
    SendMessageResponse, UpdateMessageRequest, UpdateMessageResponse, create_channel_response,
    send_message_response,
};
use elysium_rust::common::v1::ErrorCode;
use elysium_rust::{Channel, Message};
use tonic::{Request, Response, Status};

pub struct Service {
    state: ServerState,
}

impl Service {
    pub fn new(state: ServerState) -> Self {
        Self { state }
    }

    async fn _create_channel(
        &self,
        request: Request<CreateChannelRequest>,
    ) -> Result<CreateChannelResponse, Error> {
        let database = self.state.database();

        auth::verify(database, &request).await?;
        let channel_args = request.into_inner();
        let channel_id = chat::build_channel_id(database).await?;

        let channel = chat::create_channel(
            database,
            Channel {
                channel_id,
                name: channel_args.name,
                description: channel_args.description,
                members: channel_args.members,
            },
        )
        .await?;

        Ok(CreateChannelResponse {
            result: Some(create_channel_response::Result::Channel(channel.into())),
        })
    }

    async fn _send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<SendMessageResponse, Error> {
        let database = self.state.database();

        let user = auth::verify(database, &request).await?;
        let msg_args = request.into_inner();

        let mut content = msg_args.content.ok_or(Error::invalid_argument())?;

        content.created_at = Some(time::get_timestamp());

        let perm =
            chat::get_channel_member_perm(database, &msg_args.channel_id, &user.user_id).await?;

        if perm == ChannelPermission::ReadWrite || perm == ChannelPermission::Manager {
            let id = chat::build_message_id(database).await?;
            let msg = chat::send(
                database,
                Message {
                    message_id: id,
                    user_id: user.user_id,
                    channel_id: msg_args.channel_id,
                    content: content.try_into()?,
                },
            )
            .await?;

            Ok(SendMessageResponse {
                result: Some(send_message_response::Result::Message(msg.into())),
            })
        } else {
            Err(Error::new(
                ErrorCode::Unauthorized,
                "User has no permission to send messages",
            ))
        }
    }
}

#[tonic::async_trait]
impl ChatService for Service {
    async fn create_channel(
        &self,
        request: Request<CreateChannelRequest>,
    ) -> Result<Response<CreateChannelResponse>, Status> {
        let resp =
            self._create_channel(request)
                .await
                .unwrap_or_else(|err| CreateChannelResponse {
                    result: Some(create_channel_response::Result::Error(err.into())),
                });

        Ok(Response::new(resp))
    }

    async fn read_messages(
        &self,
        _request: Request<ReadMessagesRequest>,
    ) -> Result<Response<ReadMessagesResponse>, Status> {
        todo!()
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let resp = self
            ._send_message(request)
            .await
            .unwrap_or_else(|err| SendMessageResponse {
                result: Some(send_message_response::Result::Error(err.into())),
            });

        Ok(Response::new(resp))
    }

    async fn delete_message(
        &self,
        _request: Request<DeleteMessageRequest>,
    ) -> Result<Response<DeleteMessageResponse>, Status> {
        todo!()
    }

    async fn update_message(
        &self,
        _request: Request<UpdateMessageRequest>,
    ) -> Result<Response<UpdateMessageResponse>, Status> {
        todo!()
    }
}

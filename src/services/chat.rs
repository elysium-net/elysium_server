use crate::error::Error;
use crate::state::ServerState;
use crate::{auth, chat, time};
use elysium_rust::chat::v1::chat_service_server::ChatService;
use elysium_rust::chat::v1::{
    Channel, ChannelPermission, CreateChannelRequest, CreateChannelResponse, DeleteMessageRequest,
    DeleteMessageResponse, Message, ReadMessagesRequest, ReadMessagesResponse, SendMessageRequest,
    SendMessageResponse, UpdateMessageRequest, UpdateMessageResponse, create_channel_response,
    send_message_response,
};
use elysium_rust::common::v1::ErrorCode;
use tonic::{Request, Response, Status};

pub struct Service {
    state: ServerState,
}

impl Service {
    pub fn new(state: ServerState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl ChatService for Service {
    async fn create_channel(
        &self,
        request: Request<CreateChannelRequest>,
    ) -> Result<Response<CreateChannelResponse>, Status> {
        let auth = auth::verify(self.state.database(), &request).await;
        let channel_args = request.into_inner();

        let resp = if let Err(err) = auth {
            CreateChannelResponse {
                result: Some(create_channel_response::Result::Error(err.into())),
            }
        } else {
            match chat::build_channel_id(self.state.database()).await {
                Ok(channel_id) => match chat::create_channel(
                    self.state.database(),
                    Channel {
                        channel_id,
                        name: channel_args.name,
                        description: channel_args.description,
                        members: channel_args.members,
                    },
                )
                .await
                {
                    Ok(channel) => CreateChannelResponse {
                        result: Some(create_channel_response::Result::Channel(channel)),
                    },
                    Err(err) => CreateChannelResponse {
                        result: Some(create_channel_response::Result::Error(err.into())),
                    },
                },

                Err(err) => CreateChannelResponse {
                    result: Some(create_channel_response::Result::Error(err.into())),
                },
            }
        };

        Ok(Response::new(resp))
    }

    async fn read_messages(
        &self,
        request: Request<ReadMessagesRequest>,
    ) -> Result<Response<ReadMessagesResponse>, Status> {
        todo!()
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let auth = auth::verify(self.state.database(), &request).await;
        let msg_args = request.into_inner();

        let mut content = msg_args
            .content
            .ok_or(Status::invalid_argument("Request must contain content"))?;

        content.created_at = Some(time::get_timestamp());

        let resp = match auth {
            Ok(user) => {
                match chat::get_channel_member_perm(
                    self.state.database(),
                    &msg_args.channel_id,
                    &user.user_id,
                )
                .await
                {
                    Ok(perm) => match perm {
                        perm => {
                            if perm == ChannelPermission::Manager
                                || perm == ChannelPermission::ReadWrite
                            {
                                match chat::build_message_id(self.state.database()).await {
                                    Ok(id) => {
                                        match chat::send(
                                            self.state.database(),
                                            Message {
                                                message_id: id,
                                                user_id: user.user_id,
                                                channel_id: msg_args.channel_id,
                                                content: Some(content),
                                            },
                                        )
                                        .await
                                        {
                                            Ok(msg) => SendMessageResponse {
                                                result: Some(
                                                    send_message_response::Result::Message(msg),
                                                ),
                                            },
                                            Err(err) => SendMessageResponse {
                                                result: Some(send_message_response::Result::Error(
                                                    err.into(),
                                                )),
                                            },
                                        }
                                    }
                                    Err(err) => SendMessageResponse {
                                        result: Some(send_message_response::Result::Error(
                                            err.into(),
                                        )),
                                    },
                                }
                            } else {
                                SendMessageResponse {
                                    result: Some(send_message_response::Result::Error(
                                        Error::new(
                                            ErrorCode::Unauthorized,
                                            "User has no permission to send messages",
                                        )
                                        .into(),
                                    )),
                                }
                            }
                        }
                    },
                    Err(err) => SendMessageResponse {
                        result: Some(send_message_response::Result::Error(err.into())),
                    },
                }
            }
            Err(err) => SendMessageResponse {
                result: Some(send_message_response::Result::Error(err.into())),
            },
        };

        Ok(Response::new(resp))
    }

    async fn delete_message(
        &self,
        request: Request<DeleteMessageRequest>,
    ) -> Result<Response<DeleteMessageResponse>, Status> {
        todo!()
    }

    async fn update_message(
        &self,
        request: Request<UpdateMessageRequest>,
    ) -> Result<Response<UpdateMessageResponse>, Status> {
        todo!()
    }
}

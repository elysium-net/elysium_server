use crate::state::ServerState;
use elysium_rust::chat::v1::chat_service_server::ChatService;
use elysium_rust::chat::v1::{
    DeleteMessageRequest, DeleteMessageResponse, SendMessageRequest, SendMessageResponse,
    UpdateMessageRequest, UpdateMessageResponse,
};
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
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        todo!()
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

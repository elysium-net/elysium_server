use crate::state::ServerState;
use elysium_rust::resource::v1::resource_service_server::ResourceService;
use elysium_rust::resource::v1::{
    DownloadRequest, DownloadResponse, UploadRequest, UploadResponse,
};
use tonic::codegen::BoxStream;
use tonic::{Request, Response, Status, Streaming};

pub struct Service {
    state: ServerState,
}

impl Service {
    pub fn new(state: ServerState) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl ResourceService for Service {
    async fn upload(
        &self,
        _request: Request<Streaming<UploadRequest>>,
    ) -> Result<Response<UploadResponse>, Status> {
        todo!()
    }

    type DownloadStream = BoxStream<DownloadResponse>;

    async fn download(
        &self,
        _request: Request<DownloadRequest>,
    ) -> Result<Response<Self::DownloadStream>, Status> {
        todo!()
    }
}

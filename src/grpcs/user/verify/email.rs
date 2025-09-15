use crate::state::ServerState;
use elysium_api::proto::{VerifyUserEmailRequest, VerifyUserEmailResponse};
use tonic::{Request, Response, Status};

pub async fn verify_email(
    state: ServerState,
    req: Request<VerifyUserEmailRequest>,
) -> Result<Response<VerifyUserEmailResponse>, Status> {
    state
        .email_verifier()
        .start_verify(req.into_inner().email)
        .await?;

    Ok(Response::new(VerifyUserEmailResponse {}))
}

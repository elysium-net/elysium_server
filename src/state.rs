use crate::database::Database;
use crate::email::EmailVerifier;
use crate::error::ElyError;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ServerState {
    database: Arc<Database>,
    email_verifier: Arc<EmailVerifier>,
}

impl ServerState {
    pub async fn new() -> Result<Self, ElyError> {
        Ok(Self {
            database: Arc::new(Database::new().await?),
            email_verifier: Arc::new(EmailVerifier::new()),
        })
    }

    pub fn database(&self) -> Arc<Database> {
        self.database.clone()
    }

    pub fn email_verifier(&self) -> Arc<EmailVerifier> {
        self.email_verifier.clone()
    }
}

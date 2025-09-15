use crate::config;
use crate::error::ElyError;
use crate::templates::VerifyEmailTemplate;
use ahash::RandomState;
use askama::Template;
use elysium_api::id;
use elysium_api::id::RandId;
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MessageBuilder};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::{Address, SmtpTransport, Transport};
use scc::HashMap;
use std::str::FromStr;
use std::time::{Duration, Instant};

const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Debug, Clone)]
struct EmailVerifyEntry {
    pub token: RandId,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct EmailVerifier {
    tokens: HashMap<String, EmailVerifyEntry, RandomState>,
    transport: SmtpTransport,
}

impl EmailVerifier {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::default(),
            transport: if cfg!(debug_assertions) {
                SmtpTransport::unencrypted_localhost()
            } else {
                SmtpTransport::relay(config::email::EMAIL_SMTP_HOST.as_str())
                    .unwrap()
                    .port(*config::email::EMAIL_SMTP_PORT)
                    .tls(Tls::None)
                    .credentials(Credentials::new(
                        config::email::EMAIL_SMTP_USER.to_string(),
                        config::email::EMAIL_SMTP_PASSWORD.to_string(),
                    ))
                    .build()
            },
        }
    }

    pub async fn start_verify(&self, email: String) -> Result<(), ElyError> {
        let token = self.generate_token();

        self.send_email(&email, token.to_string())?;

        let entry = self.tokens.entry_async(email).await;
        entry.insert_entry(EmailVerifyEntry {
            token,
            timestamp: Instant::now(),
        });

        Ok(())
    }

    pub async fn end_verify(&self, email: String, token: RandId) -> Result<String, ElyError> {
        if let Some(entry) = self.tokens.get_async(&email).await {
            let (_, entry) = entry.remove_entry();

            if entry.token == token {
                Ok(email)
            } else {
                Err(ElyError::EmailNotVerified)
            }
        } else {
            Err(ElyError::EmailNotVerified)
        }
    }

    pub async fn cleanup(&self) {
        self.tokens
            .retain_async(|_, entry| {
                entry.timestamp.elapsed()
                    < Duration::from_secs(*config::email::EMAIL_VERIFY_EXPIRY_SECS)
            })
            .await
    }

    fn send_email(&self, email: &String, token: RandId) -> Result<(), ElyError> {
        let template = VerifyEmailTemplate {
            token: token.as_str(),
        };

        let msg = MessageBuilder::new()
            .to(Mailbox::new(
                None,
                Address::from_str(email).map_err(|_| ElyError::InvalidToken)?,
            ))
            .from(Mailbox::new(
                None,
                Address::new("no-reply", config::app::DOMAIN.as_str()).unwrap(),
            ))
            .header(ContentType::TEXT_HTML)
            .subject("Elysium Email Verification")
            .body(template.render().unwrap())
            .expect("failed to build email");

        self.transport.send(&msg).unwrap();

        Ok(())
    }

    fn generate_token(&self) -> RandId {
        id::generate_custom(*config::email::EMAIL_VERIFY_TOKEN_LEN, &NUMBERS)
    }
}

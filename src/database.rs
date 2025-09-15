use crate::config;
use std::ops::Deref;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

#[derive(Clone, Debug)]
pub struct Database {
    surreal: Surreal<Client>,
}

impl Database {
    pub async fn new() -> Result<Self, surrealdb::Error> {
        tracing::info!("Connecting to database...");

        let surreal = Surreal::new::<Ws>(config::database::DB_URL.as_str()).await?;

        surreal
            .signin(Root {
                username: config::database::DB_USER.as_str(),
                password: config::database::DB_PASS.as_str(),
            })
            .await?;

        surreal
            .use_ns(config::database::DB_NAMESPACE.as_str())
            .use_db(config::database::DB_DATABASE.as_str())
            .await?;

        Ok(Self { surreal })
    }
}

impl Deref for Database {
    type Target = Surreal<Client>;

    fn deref(&self) -> &Self::Target {
        &self.surreal
    }
}

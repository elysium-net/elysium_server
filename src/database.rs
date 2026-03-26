use crate::{cfg, user};
use std::ops::Deref;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;

#[derive(Clone, Debug)]
pub struct Database {
    surreal: Surreal<Client>,
}

impl Database {
    pub async fn new() -> Self {
        let surreal = Surreal::new::<Ws>(cfg::DATABASE_ADDRESS.as_str())
            .await
            .expect("Failed to connect to database");

        surreal
            .signin(Root {
                username: cfg::DATABASE_USER.to_string(),
                password: cfg::DATABASE_PASSWORD.to_string(),
            })
            .await
            .expect("Failed to login to database");

        surreal
            .use_ns(cfg::DATABASE_NAMESPACE.as_str())
            .use_db(cfg::DATABASE_NAME.as_str())
            .await
            .expect("Failed to get into database");

        let this = Self { surreal };

        this.setup().await;

        this
    }

    async fn setup(&self) {
        self.query("DEFINE TABLE IF NOT EXISTS user SCHEMALESS;")
            .await
            .expect("Failed to setup user table");
    }
}

impl Deref for Database {
    type Target = Surreal<Client>;

    fn deref(&self) -> &Self::Target {
        &self.surreal
    }
}

use crate::cfg;
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

        #[cfg(test)]
        {
            tracing::info!("Detected test environment. Clearing database...");

            surreal
                .query(
                    r#"
REMOVE TABLE user;
REMOVE TABLE channel;
REMOVE TABLE message;"#,
                )
                .await
                .expect("Failed to drop user table");
        }

        let this = Self { surreal };

        this.setup().await;

        this
    }

    async fn setup(&self) {
        self.query(
            r#"
DEFINE TABLE IF NOT EXISTS user SCHEMALESS;
DEFINE TABLE IF NOT EXISTS channel SCHEMALESS;
DEFINE TABLE IF NOT EXISTS message SCHEMALESS;
"#,
        )
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

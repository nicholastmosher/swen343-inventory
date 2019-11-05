use reqwest::Client;
use std::sync::Arc;
use actix::{Actor, SyncContext};

pub mod v2;

/// Configurations for the HttpExecutor such as hostnames for other services.
pub struct HttpConfig {
    pub accounting_url: Option<String>,
    pub manufacturing_url: Option<String>,
    pub sales_url: Option<String>,
}

impl HttpConfig {
    /// Constructs an HttpConfig from environment variables
    pub fn from_env() -> Result<Self, String> {
        let accounting_url = std::env::var("ACCOUNTING_URL").ok();
        let manufacturing_url = std::env::var("MANUFACTURING_URL").ok();
        let sales_url = std::env::var("SALES_URL").ok();
        Ok(HttpConfig {
            accounting_url,
            manufacturing_url,
            sales_url,
        })
    }
}

/// An actor which can send Http requests
#[derive(Clone)]
pub struct HttpExecutor {
    /// A shared reference to the reqwest::Client object. The Client internally
    /// manages a connection pool, so we should not duplicate it when we scale
    /// up the number of HttpExecutor actor instances, therefore we keep one
    /// instance inside of an Arc.
    client: Arc<Client>,
    /// A shared reference to the HttpConfig. The config is unchanging after
    /// initialization, so we put it in an Arc and share it between instances.
    config: Arc<HttpConfig>,
}

impl Actor for HttpExecutor {
    type Context = SyncContext<Self>;
}

impl HttpExecutor {
    /// Creates a new instance of an HttpExecutor, which contains a reference
    /// to a shared reqwest::Client.
    pub fn new(config: HttpConfig) -> Self {
        HttpExecutor {
            client: Arc::new(Client::new()),
            config: Arc::new(config),
        }
    }
}

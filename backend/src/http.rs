use reqwest::Client;
use std::sync::Arc;
use actix::{Actor, SyncContext};
use reqwest::header::{HeaderMap, HeaderValue, HeaderName};

pub mod v2;

/// Configurations for the HttpExecutor such as hostnames for other services.
pub struct HttpConfig {
    pub accounting_url: Option<String>,
    pub manufacturing_url: Option<String>,
    pub sales_url: Option<String>,
    pub support_url: Option<String>,
    pub inventory_email: Option<String>,
    pub inventory_token: Option<String>,
}

impl HttpConfig {
    /// Constructs an HttpConfig from environment variables
    pub fn from_env() -> Result<Self, String> {
        let accounting_url = std::env::var("ACCOUNTING_URL").ok();
        let manufacturing_url = std::env::var("MANUFACTURING_URL").ok();
        let sales_url = std::env::var("SALES_URL").ok();
        let support_url = std::env::var("SUPPORT_URL").ok();
        let inventory_email = std::env::var("INVENTORY_EMAIL").ok();
        let inventory_token = std::env::var("INVENTORY_TOKEN").ok();
        Ok(HttpConfig {
            accounting_url,
            manufacturing_url,
            sales_url,
            support_url,
            inventory_email,
            inventory_token,
        })
    }

    pub fn auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let email = &self.inventory_email;
        let token = &self.inventory_token;
        if let (Some(email), Some(token)) = (email, token) {
            let email_hv = HeaderValue::from_str(&email).expect("email should be a valid header");
            let token_hv = HeaderValue::from_str(&token).expect("token should be a valid header");
            headers.insert(HeaderName::from_static("email"), email_hv);
            headers.insert(HeaderName::from_static("authorization"), token_hv);
        }
        headers
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

//! Defines the web application configuration and state.
//!
//! The Inventory Management backend is stateless except for the reference to
//! the database actor. This means that every request can be served without
//! any context of previous calls, allowing the server to be deployed many
//! times horizontally without needing to coordinate any state between different
//! instances of processes.

use actix::{Addr, SyncArbiter};
use actix_web::{web, HttpServer, App};
use crate::db::DbExecutor;
use actix_cors::Cors;
use actix_web::middleware::Logger;

pub mod v1;
pub mod v2;

pub struct AppConfig {
    pub database_url: String,
    pub bind_address: String,
    pub frontend_address: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, String> {
        use std::env;
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "failed to read DATABASE_URL environment variable")?;
        let bind_address = env::var("BIND_ADDRESS")
            .map_err(|_| "failed to read BIND_ADDRESS environment variable")?;
        let frontend_address = env::var("FRONTEND_ADDRESS").ok();
        Ok(AppConfig { database_url, bind_address, frontend_address })
    }
}

/// The only application state used is a reference to the database Actor inbox.
pub struct AppState {
    db: Addr<DbExecutor>,
}

/// Given a URL to the database and a web address, launches the web server.
///
/// Notice that the configuration of the web server is declarative. Rather
/// than creating a web server and launching it, we tell the system _how_ to
/// create a web server and let it take care of launching it. This is important
/// because the runtime has the freedom to launch one or many instances of the
/// server on multiple threads or even on multiple processes.
///
/// The web application is configured using the `AppConfig` struct. You can use
/// the `from_env` function to create an instance of `AppConfig` using values
/// loaded from environment variables.
///
/// # Example
///
/// ```no_run
/// use erp::app::{launch, AppConfig};
///
/// let config = AppConfig::from_env().expect("should load AppConfig from env");
/// launch(&config).expect("should launch web server");
/// ```
///
/// You can also use the struct initialization syntax in order to manually
/// override any values that you want.
///
/// # Example
///
/// ```no_run
/// use erp::app::{launch, AppConfig};
///
/// let env_config = AppConfig::from_env().expect("should load AppConfig from env");
/// launch(&AppConfig {
///     database_url: "postgres://postgres:test-db-password@localhost/test-db".to_string(),
///     ..env_config
/// }).expect("should launch web server");
/// ```
pub fn launch(config: &AppConfig) -> std::io::Result<()>
{
    let database_url = config.database_url.clone();
    let frontend_address = config.frontend_address.clone();
    let mut listenfd = listenfd::ListenFd::from_env();

    let database_addr = SyncArbiter::start(
        num_cpus::get(),
        move || DbExecutor::new(database_url.clone()),
    );

    let mut server = HttpServer::new(move || {
        let cors = match &frontend_address {
            Some(origin) => {
                Cors::new()
                    .allowed_origin(origin)
                    .max_age(3600)
            },
            None => {
                Cors::new()
                    .allowed_methods(vec!["POST", "GET", "PUT", "DELETE"])
                    .send_wildcard()
                    .max_age(3600)
            },
        };

        App::new()
            .data(AppState { db: database_addr.clone() })
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes)
    });

    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(&config.bind_address)?
    };

    server.start();
    Ok(())
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(v1::routes(web::scope("/api/v1")))
        .service(v2::routes(web::scope("/api/v2")));
}

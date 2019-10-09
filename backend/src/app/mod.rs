//! Defines the web application configuration and state.
//!
//! The Inventory Management backend is stateless except for the reference to
//! the database actor. This means that every request can be served without
//! any context of previous calls, allowing the server to be deployed many
//! times horizontally without needing to coordinate any state between different
//! instances of processes.

use std::net::ToSocketAddrs;
use actix::{Addr, SyncArbiter};
use actix_web::{web, HttpServer, App};
use crate::db::DbExecutor;

pub mod products;

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
/// # Example
///
/// ```no_run
/// use erp::app;
///
/// app::launch(
///     "postgres://postgres:password@localhost/dbname",
///     "127.0.0.1:8000",
/// ).expect("should launch web server");
/// ```
pub fn launch<S, A>(database_url: S, bind_address: A) -> std::io::Result<()>
    where S: Into<String>, A: ToSocketAddrs,
{
    let database_url = database_url.into();
    let mut listenfd = listenfd::ListenFd::from_env();

    let database_addr = SyncArbiter::start(
        num_cpus::get(),
        move || DbExecutor::new(database_url.clone()),
    );

    let mut server = HttpServer::new(move ||
        App::new()
            .data(AppState { db: database_addr.clone() })
            .service(web::scope("/api/v1")
                .route("products", web::post().to_async(products::create))
                .route("products", web::get().to_async(products::read))
                .route("products", web::put().to_async(products::update))
                .route("products", web::delete().to_async(products::delete))
            )
    );

    server = if let Some(listener) = listenfd.take_tcp_listener(0)? {
        server.listen(listener)?
    } else {
        server.bind(bind_address)?
    };

    server.run()
}

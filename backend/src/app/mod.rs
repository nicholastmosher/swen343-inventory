use std::net::ToSocketAddrs;
use actix::{Addr, SyncArbiter};
use actix_web::{web, HttpServer, App};
use crate::db::DbExecutor;

pub mod products;

pub struct AppState {
    db: Addr<DbExecutor>,
}

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

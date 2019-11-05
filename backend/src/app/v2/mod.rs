//! Route definitions for the v2 API.

use actix_web::{web, Scope};

pub mod items;
pub mod stock;

pub fn routes(app: Scope) -> Scope {
    app
        .service(web::resource("rest/stock")
            .route(web::get().to_async(stock::read))
        )
        .service(web::resource("rest/receiveItems")
            .route(web::post().to_async(items::add))
        )
}

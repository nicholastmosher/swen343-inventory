//! Route definitions for the v2 API.

use actix_web::{web, Scope};

pub mod items;

pub fn routes(app: Scope) -> Scope {
    app
        .service(web::resource("warehouses")
        )
        .service(web::resource("products")
        )
        .service(web::resource("rest/receiveItems")
            .route(web::post().to_async(items::add))
        )
}

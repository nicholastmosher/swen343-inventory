//! Route definitions for the v2 API.

use actix_web::{web, Scope};
use futures::{FutureExt, TryFutureExt};

pub mod items;
pub mod stock;
pub mod order;
pub mod returns;

pub fn routes(app: Scope) -> Scope {
    app
        .service(web::resource("rest/stock")
            .route(web::get().to_async(|state| {
                stock::read(state).boxed().compat()
            }))
        )
        .service(web::resource("rest/receiveItems")
            .route(web::post().to_async(|state, json| {
                items::add(state, json).boxed().compat()
            }))
        )
        .service(web::resource("rest/order")
            .route(web::post().to_async(|state, json| {
                order::place_order(state, json).boxed().compat()
            }))
        )
        .service(web::resource("rest/returns")
            .route(web::post().to_async(|state, json| {
                returns::return_product(state, json).boxed().compat()
            }))
        )
}

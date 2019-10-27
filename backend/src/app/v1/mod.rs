//! Route definitions for the v1 API.

use actix_web::{web, Scope};

pub mod items;
pub mod boxes;
pub mod pallets;
pub mod warehouses;
pub mod rules;

pub fn routes(app: Scope) -> Scope {
    app
        .service(web::resource("warehouses")
            .route(web::post().to_async(warehouses::create))
            .route(web::get().to_async(warehouses::read))
            .route(web::put().to_async(warehouses::update))
            .route(web::delete().to_async(warehouses::delete)))
        .service(web::resource("items")
            .route(web::post().to_async(items::create))
            .route(web::get().to_async(items::read))
            .route(web::put().to_async(items::update))
            .route(web::delete().to_async(items::delete)))
        .service(web::resource("boxes")
             .route(web::post().to_async(boxes::create))
             .route(web::get().to_async(boxes::read))
             .route(web::put().to_async(boxes::update))
             .route(web::delete().to_async(boxes::delete)))
        .service(web::resource("pallets")
            .route(web::post().to_async(pallets::create))
            .route(web::get().to_async(pallets::read))
            .route(web::put().to_async(pallets::update))
            .route(web::delete().to_async(pallets::delete)))
        .service(web::resource("rules")
            .route(web::post().to_async(rules::create))
            .route(web::get().to_async(rules::read))
            .route(web::put().to_async(rules::update))
            .route(web::delete().to_async(rules::delete))
        )
}

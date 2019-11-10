//! Route definitions for the v1 API.

use actix_web::{web, Scope};
use futures::{FutureExt, TryFutureExt};

pub mod items;
pub mod boxes;
pub mod pallets;
pub mod warehouses;
pub mod rules;

pub fn routes(app: Scope) -> Scope {
    app
        .service(web::resource("warehouses")
            .route(web::post().to_async(|state, json| {
                warehouses::create(state, json).boxed().compat()
            }))
            .route(web::get().to_async(|state| {
                warehouses::read(state).boxed().compat()
            }))
            .route(web::put().to_async(|state, json| {
                warehouses::update(state, json).boxed().compat()
            }))
            .route(web::delete().to_async(|state, json| {
                warehouses::delete(state, json).boxed().compat()
            })))
        .service(web::resource("items")
            .route(web::post().to_async(|state, json| {
                items::create(state, json).boxed().compat()
            }))
            .route(web::get().to_async(|state| {
                items::read(state).boxed().compat()
            }))
            .route(web::put().to_async(|state, json| {
                items::update(state, json).boxed().compat()
            }))
            .route(web::delete().to_async(|state, json| {
                items::delete(state, json).boxed().compat()
            })))
        .service(web::resource("boxes")
             .route(web::post().to_async(|state, json| {
                 boxes::create(state, json).boxed().compat()
             }))
             .route(web::get().to_async(|state| {
                 boxes::read(state).boxed().compat()
             }))
             .route(web::put().to_async(|state, json| {
                 boxes::update(state, json).boxed().compat()
             }))
             .route(web::delete().to_async(|state, json| {
                 boxes::delete(state, json).boxed().compat()
             })))
        .service(web::resource("pallets")
            .route(web::post().to_async(|state, json| {
                pallets::create(state, json).boxed().compat()
            }))
            .route(web::get().to_async(|state| {
                pallets::read(state).boxed().compat()
            }))
            .route(web::put().to_async(|state, json| {
                pallets::update(state, json).boxed().compat()
            }))
            .route(web::delete().to_async(|state, json| {
                pallets::delete(state, json).boxed().compat()
            })))
        .service(web::resource("rules")
            .route(web::post().to_async(|state, json| {
                rules::create(state, json).boxed().compat()
            }))
            .route(web::get().to_async(|state| {
                rules::read(state).boxed().compat()
            }))
            .route(web::put().to_async(|state, json| {
                rules::update(state, json).boxed().compat()
            }))
            .route(web::delete().to_async(|state, json| {
                rules::delete(state, json).boxed().compat()
            }))
        )
}

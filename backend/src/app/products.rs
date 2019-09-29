use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use futures::Future;
use crate::app::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>
}

pub fn create(
    state: web::Data<AppState>,
    post_product: web::Json<CreateProduct>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.get_ref().db;
    let create_product = post_product.into_inner();

    db.send(create_product)
        .and_then(|res| match res {
            Ok(product) => Ok(HttpResponse::Ok().json(product)),
            Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
        })
        .map_err(|_| ())
}

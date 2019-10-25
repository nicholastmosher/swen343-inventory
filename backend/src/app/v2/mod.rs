//! Route definitions for the v2 API.

use actix_web::{web, Scope};

pub fn routes(app: Scope) -> Scope {
    app
        .service(web::resource("warehouses")
        )
        .service(web::resource("products")
        )
}

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};
use futures::{FutureExt, TryFutureExt, compat::Future01CompatExt};
use crate::app::AppState;
use crate::app::v2::stock::{ReadStock, StockInResponse, StockToRemove, RemoveStock};

#[derive(Debug, Deserialize)]
pub struct OrderRequest {
    order_id: i32,
    products: Vec<ProductInOrder>,
}

#[derive(Debug, Deserialize)]
pub struct ProductInOrder {
    product: String,
    quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    status: String,
}

pub struct OrderToManufacturing {

}

pub async fn place_order(
    state: web::Data<AppState>,
    web::Json(order): web::Json<OrderRequest>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;
    let http = &state.http;

    // Check if we have enough products to create the order

    let result = db.send(ReadStock).compat().await;
    let stock_response = match result {
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
        Ok(Err(e)) => return Ok(HttpResponse::InternalServerError().body(e)),
        Ok(Ok(stock_response)) => stock_response,
    };

    let stocks: HashMap<String, StockInResponse> = stock_response.stock.into_iter()
        .map(|item| (item.product.clone(), item))
        .collect();

    let mut should_order = false;
    for product in &order.products {
        match stocks.get(&product.product) {
            None => should_order = true,
            Some(item) => {
                if item.quantity < product.quantity as u32 {
                    should_order = true;
                }
            }
        }
    }

    if !should_order {
        // If we have enough products to create the order, we remove
        // the stock from inventory and return success
        let removal_request = RemoveStock {
            stock: order.products.into_iter()
                .map(|product| StockToRemove {
                    item: product.product,
                    quantity: product.quantity as u32,
                }).collect()
        };

        // If we have all of the items in stock, return those items to Sales
        let result = db.send(removal_request).compat().await;
        let response = match result {
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            Ok(Err(e)) => Ok(HttpResponse::InternalServerError().body(e)),
            Ok(Ok(_)) => Ok(HttpResponse::Ok().json(OrderResponse { status: "success".to_string() }))
        };
        return response;
    }

    // If we don't have enough products to create the order,
    // kick off a request to Manufacturing and return a failure to Sales
    actix::spawn(manufacturing_order(state, web::Json(order)).boxed().compat());
    return Ok(HttpResponse::Accepted().json(OrderResponse {
        status: "order accepted; sending to manufacturing".to_string() ,
    }))
}

pub async fn manufacturing_order(
    state: web::Data<AppState>,
    web::Json(order): web::Json<OrderRequest>,
) -> Result<(), ()> {

    // MANUFACTURING FLOW //////////////////////////////////////////////////////

    // Send a recipeInfo request to Manufacturing to get raw parts requirements

    // Check if we have enough raw parts to fulfill the production order

    //     If no, make a budget request to Accounting

    //         If budget is approved, continue to order raw parts

    //         If budget is not approved, make a budget increase request to Accounting

    //             Continue polling Accounting until budget increase is approved

    //             Continue to order raw parts

    // Order raw parts required for production order

    // Send a "make" request to Manufacturing to create Products from raw parts

    Ok(())
}

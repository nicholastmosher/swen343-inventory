use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse};
use futures::{FutureExt, TryFutureExt, compat::Future01CompatExt};
use crate::app::AppState;
use crate::app::v2::stock::{ReadStock, StockInResponse, StockToRemove, RemoveStock};
use crate::http::v2::manufacturing::{RecipeRequest, RecipeResponse, SendPartsRequest, ProductRequest, PartRequest};
use crate::app::v1::items::{ReadItems, ItemResponse};
use crate::http::v2::accounting::{ExpenseRequest, ExpenseResponse};
use crate::app::v2::items::{ReceiveItemsRequest, ItemInRequest};

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

pub async fn place_order(
    state: web::Data<AppState>,
    web::Json(order): web::Json<OrderRequest>,
) -> Result<HttpResponse, ()> {
    let db = &state.db;

    // Check if we have enough products to create the order

    let result = db.send(ReadStock).compat().await;
    let stock_response = match result {
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
        Ok(Err(e)) => return Ok(HttpResponse::InternalServerError().body(e)),
        Ok(Ok(stock_response)) => stock_response,
    };

    let stock: HashMap<String, StockInResponse> = stock_response.stock.into_iter()
        .map(|item| (item.product.clone(), item))
        .collect();

    let mut should_order = false;
    for product in &order.products {
        match stock.get(&product.product) {
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
    actix::spawn(manufacturing_order(state, order, stock).boxed().compat());
    return Ok(HttpResponse::Accepted().json(OrderResponse {
        status: "order accepted; sending to manufacturing".to_string() ,
    }))
}

pub async fn manufacturing_order(
    state: web::Data<AppState>,
    order: OrderRequest,
    stock: HashMap<String, StockInResponse>,
) -> Result<(), ()> {
    let db = &state.db;
    let http = &state.http;

    // MANUFACTURING FLOW //////////////////////////////////////////////////////

    // Send a recipeInfo request to Manufacturing to get raw parts requirements

    let recipe_requests = order.products.iter().map(|item| {
        let request = RecipeRequest {
            item_code: item.product.clone(),
            quantity: item.quantity as u32,
        };
        http.send(request).compat()
    });

    // Take our list of Futures and get back a Future with a list of results
    let joined_requests = futures::future::join_all(recipe_requests);
    let recipe_responses = joined_requests.await;

    let recipes: Vec<RecipeResponse> = {
        let mut recipes = Vec::with_capacity(recipe_responses.len());
        for response in recipe_responses {
            match response {
                Err(_) | Ok(Err(_)) => {
                    println!("Encountered error in recipe response");
                    return Err(());
                },
                Ok(Ok(recipe)) => recipes.push(recipe),
            }
        }
        recipes
    };

    // Check if we have enough raw parts to fulfill the production order

    // Create a map from part name to the quantity of those parts we need
    let mut recipe_parts: HashMap<String, u32> = HashMap::new();
    for recipe in &recipes {
        for part in &recipe.parts {
            match recipe_parts.get_mut(&part.item_code) {
                None => {
                    recipe_parts.insert(part.item_code.clone(), part.quantity);
                },
                Some(quantity) => {
                    *quantity += part.quantity;
                },
            }
        }
    }

    let mut need_to_order_parts = false;
    let mut parts_to_order: HashMap<String, u32> = HashMap::new();
    for (part, needed_quantity) in recipe_parts.into_iter() {
        match stock.get(&part) {
            // If we have none of this needed part in stock, we need to order it
            None => {
                need_to_order_parts = true;
                parts_to_order.insert(part, needed_quantity);
            },
            // If we have some stock but not enough, we need to order the difference
            Some(stock_quantity) if stock_quantity.quantity < needed_quantity => {
                need_to_order_parts = true;
                parts_to_order.insert(part, needed_quantity - stock_quantity.quantity);
            }
            // If we have enough stock, we don't need to order any of this part
            Some(stock_quantity) => {
                parts_to_order.insert(part, 0);
            },
        }
    }

    // If we don't have enough parts, make a budget request to Accounting

    if need_to_order_parts {
        match accounting_order(&state, &order, parts_to_order).await {
            // If we get the approved expense back, buy parts and send them to manufacturing
            Ok(_) => {
                unimplemented!()
            },
            // If we don't get the expense approved, petition for a budget increase
            Err(_) => {
                unimplemented!()
            }
        }
    }

    // The parts needed to make our products are in the amount given by the recipe
    let products: Vec<ProductRequest> = recipes.into_iter().map(|recipe| {
        let parts = recipe.parts.into_iter().map(|part| PartRequest {
            item_code: part.item_code,
            quantity: part.quantity,
        }).collect();
        ProductRequest {
            item_code: recipe.item_code,
            quantity: recipe.quantity,
            parts,
        }
    }).collect();

    // Send a "make" request to Manufacturing to create Products from raw parts
    let make_request = SendPartsRequest {
        order_id: order.order_id as u32,
        warehouse_id: "primary-warehouse".to_string(),
        products,
    };

    match http.send(make_request).compat().await {
        // If an error occurred, don't remove parts from inventory
        Err(_) | Ok(Err(_)) => {
            unimplemented!()
        },
        // If the make request succeeded, remove parts from inventory
        Ok(_) => {
            unimplemented!()
        },
    }

    Ok(())
}

pub async fn accounting_order(
    state: &web::Data<AppState>,
    order: &OrderRequest,
    needed_parts: HashMap<String, u32>,
) -> Result<(), ()> {
    let db = &state.db;
    let http = &state.http;

    // ACCOUNTING FLOW /////////////////////////////////////////////////////////

    let items: Vec<ItemResponse> = match db.send(ReadItems).compat().await {
        Err(_) | Ok(Err(_)) => {
            warn!("Error reading item costs for calculating expense");
            return Err(());
        },
        Ok(Ok(items)) => items,
    };

    let items_by_name: HashMap<String, ItemResponse> = items.into_iter()
        .map(|item| (item.item_code.clone(), item))
        .collect();

    let total_expense = needed_parts.iter()
        .filter_map(|(part, count)| {
            let part_cost = match items_by_name.get(part) {
                None => {
                    warn!("failed to find cost of part {}", part);
                    return None;
                },
                Some(item) => item.cost,
            };
            Some(part_cost * count)
        }).fold(0, |acc, current| acc + current);

    let expense_request = ExpenseRequest {
        amount: total_expense as f32 / 10.0,
        category: "Parts".to_string(),
        department: "Inventory".to_string(),
    };

    let expense_response: ExpenseResponse = match http.send(expense_request).compat().await {
        Err(_) | Ok(Err(_)) => {
            warn!("Error sending expense request to Accounting");
            return Err(());
        },
        Ok(Ok(response)) => response,
    };

    info!("Received expense response with status: {}", &expense_response.status);

    if &*expense_response.status.to_uppercase() != "SUCCESS" {
        info!("Failed to spend money on parts!");

        // If budget is not approved, make a budget increase request to Accounting

        //     Continue polling Accounting until budget increase is approved

        //     Continue to order raw parts

        unimplemented!()
    }

    // Order raw parts required for production order

    let parts_in_request: Vec<_> = needed_parts.into_iter()
        .map(|(item_code, quantity)| {
            ItemInRequest {
                order_id: Some(order.order_id),
                item_code,
                quantity,
                refurbished: false,
                warehouse: None,
            }
        }).collect();

    let parts_request = ReceiveItemsRequest {
        products: None,
        parts: Some(parts_in_request),
    };

    match db.send(parts_request).compat().await {
        Err(_) | Ok(Err(_)) => {
            warn!("Failed to order parts");
            return Err(());
        },
        Ok(_) => {
            info!("Ordered raw parts for order {}", order.order_id);
            return Ok(());
        },
    }
}

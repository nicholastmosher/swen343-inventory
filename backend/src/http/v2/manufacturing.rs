//! Defines http requests that are sent to Manufacturing and their responses

use actix::{Handler, Message};
use serde::{Deserialize, Serialize};
use crate::http::HttpExecutor;

/// A request for fetching the recipes and required parts for given products.
#[derive(Debug, Serialize)]
pub struct RecipeRequest {
    pub products: Vec<ProductInRecipeRequest>,
}

/// A description of a single product lookup in a recipe request.
#[derive(Debug, Serialize)]
pub struct ProductInRecipeRequest {
    pub item_code: String,
    pub quantity: u32,
}

/// A response describing the raw parts required for the requested products.
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeResponse {
    pub products: Vec<ProductInRecipeResponse>,
}

/// A description of a product that was requested and the parts needed to build it.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductInRecipeResponse {
    pub item_code: String,
    pub quantity: u32,
    pub parts: Vec<PartInRecipeResponse>,
}

/// A description of the parts needed to build a specific product.
#[derive(Debug, Serialize, Deserialize)]
pub struct PartInRecipeResponse {
    pub item_code: String,
    pub quantity: u32,
}

impl Message for RecipeRequest {
    type Result = Result<RecipeResponse, String>;
}

impl Handler<RecipeRequest> for HttpExecutor {
    type Result = <RecipeRequest as Message>::Result;

    /// Defines how to send a `RecipeRequest` to the Manufacturing silo.
    fn handle(&mut self, recipe_request: RecipeRequest, _: &mut Self::Context) -> Self::Result {
        let manufacturing_url = &self.config.manufacturing_url;

        let recipe_response = match manufacturing_url {
            Some(manufacturing_url) => {
                let mut response = self.client
                    .post(manufacturing_url)
                    .json(&recipe_request)
                    .send()
                    .map_err(|e| format!("failed to send request to Manufacturing: {:?}", e))?;

                let recipe_response: RecipeResponse = response.json()
                    .map_err(|_| "failed to parse response from Manufacturing")?;

                recipe_response
            },
            None => {
                RecipeResponse {
                    products: recipe_request.products.into_iter().map(|product| {
                        ProductInRecipeResponse {
                            item_code: product.item_code,
                            quantity: product.quantity,
                            parts: vec![
                                PartInRecipeResponse {
                                    item_code: "needed_part_1".to_string(),
                                    quantity: 10,
                                }
                            ],
                        }
                    }).collect()
                }
            }
        };

        Ok(recipe_response)
    }
}

/// Send parts to manufactoring
#[derive(Debug, Serialize)]
pub struct PartRequest {
    pub item_code: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize)]
pub struct ProductRequest {
    pub item_code: String,
    pub quantity: u32,
    pub parts: Vec<PartRequest>
}

#[derive(Debug, Serialize)]
pub struct SendPartsRequest {
    pub order_id: u32,
    pub warehouse_id: String,
    pub products: Vec<ProductRequest>
}

impl Message for SendPartsRequest {
    type Result = Result<(), String>;
}

impl Handler<SendPartsRequest> for HttpExecutor {
    type Result = <SendPartsRequest as Message>::Result;

    /// Defines how to send a `SendPartsRequest` to the Manufacturing silo.
    fn handle(&mut self, req: SendPartsRequest, _: &mut Self::Context) -> Self::Result {
        let url = &self.config.manufacturing_url;

        match url {
            Some(url) => {
                let url = &format!("{}/assembly/make", &url);

                let mut response = self.client
                    .post(url)
                    .json(&req)
                    .send()
                    .map_err(|e| format!("failed to send request to Manufacturing: {:?}", e))?;

                if (!response.status().is_success()) {
                    return Err("Failed to get request".to_string())
                }
            },
            None => ()
        };

        Ok(())
    }
}



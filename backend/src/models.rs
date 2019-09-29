use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
pub struct Product {
    product_id: i32,
    product_name: String,
    product_description: Option<String>,
}

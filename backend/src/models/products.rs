use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::products;
use crate::app::products::{CreateProduct, UpdateProduct};
use std::convert::TryFrom;

#[derive(Debug, Queryable, Identifiable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub price: i32,
    pub description: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub code: String,
    pub price: i32,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "products"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedProduct {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub price: i32,
    pub description: Option<String>,
}

impl TryFrom<CreateProduct> for NewProduct {
    type Error = String;

    fn try_from(CreateProduct { name, code, price, description }: CreateProduct) -> Result<Self, Self::Error> {
        if price > std::i32::MAX as u32 { return Err("price out of bounds".to_string()); }
        Ok(NewProduct { name, code, price: price as i32, description })
    }
}

impl TryFrom<UpdateProduct> for ChangedProduct {
    type Error = String;

    fn try_from(UpdateProduct { id, name, code, price, description }: UpdateProduct) -> Result<Self, Self::Error> {
        if price > std::i32::MAX as u32 { return Err("price out of bounds".to_string()); }
        Ok(ChangedProduct { id, name, code, price: price as i32, description })
    }
}

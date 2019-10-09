use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::products;
use crate::app::products::{CreateProduct, UpdateProduct};
use std::convert::TryFrom;

/// An in-memory representation of a Product entity in the database.
#[derive(Debug, Queryable, Identifiable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub code: String,
    pub price: i32,
    pub description: Option<String>,
    pub deleted: bool,
}

/// The minimum information needed to create a new Product in the database.
#[derive(Debug, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub code: String,
    pub price: i32,
    pub description: Option<String>,
}

/// A representation of changes to a Product entity in the database.
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

/// A CreateProduct message from the webapp may be translated to a NewProduct
///
/// CreateProduct is directly deserialized from the web request and contains
/// a u32 for the price. If the value of the u32 is in the range of the i32
/// values, we can define a translation of the CreateProduct message into the
/// NewProduct type used to insert a new entity into the database.
///
/// If the price value is out of bounds for an i32, we return an error value.
impl TryFrom<CreateProduct> for NewProduct {
    type Error = String;

    fn try_from(CreateProduct { name, code, price, description }: CreateProduct) -> Result<Self, Self::Error> {
        if price > std::i32::MAX as u32 { return Err("price out of bounds".to_string()); }
        Ok(NewProduct { name, code, price: price as i32, description })
    }
}

/// An UpdateProduct message from the webapp may be translated to a ChangedProduct
///
/// UpdateProduct is directly deserialized from the web request and contains a
/// u32 for the price. If the value of the u32 is in the range of i32 values,
/// we can define a translation of the UpdateProduct message into the
/// ChangedProduct type used to update an entity in the database.
///
/// If the price value is out of bounds for an i32, we return an error value.
impl TryFrom<UpdateProduct> for ChangedProduct {
    type Error = String;

    fn try_from(UpdateProduct { id, name, code, price, description }: UpdateProduct) -> Result<Self, Self::Error> {
        if price > std::i32::MAX as u32 { return Err("price out of bounds".to_string()); }
        Ok(ChangedProduct { id, name, code, price: price as i32, description })
    }
}

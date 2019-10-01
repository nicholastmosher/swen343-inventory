use diesel::{Queryable, Insertable, Identifiable};
use crate::schema::products;
use crate::app::products::CreateProduct;

#[derive(Debug, Queryable, Identifiable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub deleted: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
}

impl From<CreateProduct> for NewProduct {
    fn from(CreateProduct { name, description }: CreateProduct) -> Self {
        NewProduct { name, description }
    }
}

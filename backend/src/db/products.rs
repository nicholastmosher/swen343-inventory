use std::convert::TryInto;
use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::products::{CreateProduct, ReadProducts, UpdateProduct, DeleteProduct, ProductResponse};
use crate::db::DbExecutor;
use crate::models::products::{NewProduct, Product, ChangedProduct};

impl Message for CreateProduct {
    type Result = Result<ProductResponse, String>;
}

impl Handler<CreateProduct> for DbExecutor {
    type Result = <CreateProduct as Message>::Result;

    fn handle(&mut self, msg: CreateProduct, _: &mut Self::Context) -> Self::Result {
        use crate::schema::products::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let new_product: NewProduct = msg.try_into()?;
        diesel::insert_into(products)
            .values(&new_product)
            .get_result::<Product>(conn)
            .map(ProductResponse::from)
            .map_err(|_| "should get inserted product".to_string())
    }
}

impl Message for ReadProducts {
    type Result = Result<Vec<ProductResponse>, String>;
}

impl Handler<ReadProducts> for DbExecutor {
    type Result = <ReadProducts as Message>::Result;

    fn handle(&mut self, _: ReadProducts, _: &mut Self::Context) -> Self::Result {
        use crate::schema::products::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        products
            .filter(deleted.eq(false))
            .load::<Product>(conn)
            .map(|read_products| {
                read_products.into_iter()
                    .map(ProductResponse::from)
                    .collect()
            })
            .map_err(|_| "failed to get products".to_string())
    }
}

impl Message for UpdateProduct {
    type Result = Result<ProductResponse, String>;
}

impl Handler<UpdateProduct> for DbExecutor {
    type Result = <UpdateProduct as Message>::Result;

    fn handle(&mut self, msg: UpdateProduct, _: &mut Self::Context) -> Self::Result {
        use crate::schema::products::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let product_id = msg.id;
        let changed_product: ChangedProduct = msg.try_into()?;
        diesel::update(products.filter(id.eq(product_id)))
            .set(&changed_product)
            .get_result::<Product>(conn)
            .map(ProductResponse::from)
            .map_err(|_| "failed to update product".to_string())
    }
}

impl Message for DeleteProduct {
    type Result = Result<ProductResponse, String>;
}

impl Handler<DeleteProduct> for DbExecutor {
    type Result = <DeleteProduct as Message>::Result;

    fn handle(&mut self, msg: DeleteProduct, _: &mut Self::Context) -> Self::Result {
        use crate::schema::products::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        diesel::update(products.filter(id.eq(msg.id)))
            .set(deleted.eq(true))
            .get_result::<Product>(conn)
            .map(ProductResponse::from)
            .map_err(|_| "failed to delete product".to_string())
    }
}

use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::products::{CreateProduct, ReadProducts, UpdateProduct};
use crate::db::DbExecutor;
use crate::models::products::{NewProduct, Product};

impl Message for CreateProduct {
    type Result = Result<Product, String>;
}

impl Handler<CreateProduct> for DbExecutor {
    type Result = <CreateProduct as Message>::Result;

    fn handle(&mut self, msg: CreateProduct, _: &mut Self::Context) -> Self::Result {
        use crate::schema::products::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let new_product: NewProduct = msg.into();
        diesel::insert_into(products)
            .values(&new_product)
            .get_result::<Product>(conn)
            .map_err(|_| "should get inserted product".to_string())
    }
}

impl Message for ReadProducts {
    type Result = Result<Vec<Product>, String>;
}

impl Handler<ReadProducts> for DbExecutor {
    type Result = <ReadProducts as Message>::Result;

    fn handle(&mut self, _: ReadProducts, _: &mut Self::Context) -> Self::Result {
        use crate::schema::products::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        products.load::<Product>(conn)
            .map_err(|_| "failed to get products".to_string())
    }
}

impl Message for UpdateProduct {
    type Result = Result<Product, String>;
}

impl Handler<UpdateProduct> for DbExecutor {
    type Result = <UpdateProduct as Message>::Result;

    fn handle(&mut self, msg: UpdateProduct, _: &mut Self::Context) -> Self::Result {
        use crate::schema::products::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        diesel::update(products.filter(id.eq(msg.id)))
            .set((name.eq(msg.name), description.eq(msg.description)))
            .get_result(conn)
            .map_err(|_| "failed to update product".to_string())
    }
}

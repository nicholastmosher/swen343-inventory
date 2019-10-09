use std::convert::TryInto;
use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::products::{CreateProduct, ReadProducts, UpdateProduct, DeleteProduct, ProductResponse};
use crate::db::DbExecutor;
use crate::models::products::{NewProduct, Product, ChangedProduct};

/// Allows the `CreateProduct` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `ProductResponse` or
/// a String describing why the database action failed.
impl Message for CreateProduct {
    type Result = Result<ProductResponse, String>;
}

impl Handler<CreateProduct> for DbExecutor {
    type Result = <CreateProduct as Message>::Result;

    /// Defines how to handle the `CreateProduct` message.
    ///
    /// Here we try to convert `CreateProduct` into a `NewProduct` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an insert statement and execute it, transforming the result into a
    /// `ProductResponse` object.
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

/// Allows the `ReadProducts` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `Vec<ProductResponse>`
/// or a String describing why the database action failed.
impl Message for ReadProducts {
    type Result = Result<Vec<ProductResponse>, String>;
}

impl Handler<ReadProducts> for DbExecutor {
    type Result = <ReadProducts as Message>::Result;

    /// Defines how to handle the `ReadProduct` message.
    ///
    /// Here the query doesn't depend on any parameters from the message, so
    /// we just query for all `Products` and convert them into `ProductResponse`
    /// instances to return.
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

/// Allows the `UpdateProduct` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `ProductResponse`
/// or a String describing why the database action failed.
impl Message for UpdateProduct {
    type Result = Result<ProductResponse, String>;
}

impl Handler<UpdateProduct> for DbExecutor {
    type Result = <UpdateProduct as Message>::Result;

    /// Defines how to handle the `UpdateProduct` message.
    ///
    /// Here we try to convert `UpdateProduct` into a `ChangedProduct` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an update statement and execute it, transforming the result into a
    /// `ProductResponse` object.
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

/// Allows the `DeleteProduct` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `ProductResponse`
/// or a String describing why the database action failed.
impl Message for DeleteProduct {
    type Result = Result<ProductResponse, String>;
}

impl Handler<DeleteProduct> for DbExecutor {
    type Result = <DeleteProduct as Message>::Result;

    /// Defines how to handle the `DeleteProduct` message.
    ///
    /// Here we use the `id` from the `DeleteProduct` message to find the
    /// specified `Product` record (if it exists) and update the `deleted`
    /// field to be true. Other queries are expected to ignore entities which
    /// have set the `deleted` flag.
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

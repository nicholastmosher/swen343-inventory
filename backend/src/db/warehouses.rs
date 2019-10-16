use std::convert::TryInto;
use actix::{Message, Handler};
use diesel::prelude::*;
use crate::app::warehouses::{CreateWarehouse, ReadWarehouses, UpdateWarehouse, WarehouseResponse, DeleteWarehouse};
use crate::db::DbExecutor;
use crate::models::warehouses::{NewWarehouse, Warehouse, ChangedWarehouse};

/// Allows the `CreateWarehouse` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `WarehouseResponse` or
/// a String describing why the database action failed.
impl Message for CreateWarehouse {
    type Result = Result<WarehouseResponse, String>;
}

impl Handler<CreateWarehouse> for DbExecutor {
    type Result = <CreateWarehouse as Message>::Result;

    /// Defines how to handle the `CreateWarehouse` message.
    ///
    /// Here we try to convert `CreateWarehouse` into a `NewWarehouse` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an insert statement and execute it, transforming the result into a
    /// `WarehouseResponse` object.
    fn handle(&mut self, msg: CreateWarehouse, _: &mut Self::Context) -> Self::Result {
        use crate::schema::warehouses::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let new_warehouse: NewWarehouse = msg.try_into()?;
        diesel::insert_into(warehouses)
            .values(&new_warehouse)
            .get_result::<Warehouse>(conn)
            .map(WarehouseResponse::from)
            .map_err(|_| "should get inserted warehouse".to_string())
    }
}

/// Allows the `ReadWarehouses` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `Vec<WarehouseResponse>`
/// or a String describing why the database action failed.
impl Message for ReadWarehouses {
    type Result = Result<Vec<WarehouseResponse>, String>;
}

impl Handler<ReadWarehouses> for DbExecutor {
    type Result = <ReadWarehouses as Message>::Result;

    /// Defines how to handle the `ReadWarehouses` message.
    ///
    /// Here the query doesn't depend on any parameters from the message, so
    /// we just query for all `Warehouses` and convert them into `WarehouseResponse`
    /// instances to return.
    fn handle(&mut self, _: ReadWarehouses, _: &mut Self::Context) -> Self::Result {
        use crate::schema::warehouses::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        warehouses
            .load::<Warehouse>(conn)
            .map(|read_warehouses| {
                read_warehouses.into_iter()
                    .map(WarehouseResponse::from)
                    .collect()
            })
            .map_err(|_| "".to_string())
    }
}

/// Allows the `UpdateWarehouse` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `WarehouseResponse`
/// or a String describing why the database action failed.
impl Message for UpdateWarehouse {
    type Result = Result<WarehouseResponse, String>;
}

impl Handler<UpdateWarehouse> for DbExecutor {
    type Result = <UpdateWarehouse as Message>::Result;

    /// Defines how to handle the `UpdateWarehouse` message.
    ///
    /// Here we try to convert `UpdateWarehouse` into a `ChangedWarehouse` entity
    /// which we can use in our query DSL. Then we use diesel to construct
    /// an update statement and execute it, transforming the result into a
    /// `WarehouseResponse` object.
    fn handle(&mut self, msg: UpdateWarehouse, _: &mut Self::Context) -> Self::Result {
        use crate::schema::warehouses::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        let changed_warehouse: ChangedWarehouse = msg.try_into()?;
        diesel::update(warehouses.filter(name.eq(&changed_warehouse.name)))
            .set(&changed_warehouse)
            .get_result::<Warehouse>(conn)
            .map(WarehouseResponse::from)
            .map_err(|_| "failed to update warehouse".to_string())
    }
}

/// Allows the `DeleteWarehouse` type to be used as a Message to an Actor.
///
/// The response that the actor will return is either a `WarehouseResponse`
/// or a String describing why the database action failed.
impl Message for DeleteWarehouse {
    type Result = Result<WarehouseResponse, String>;
}

impl Handler<DeleteWarehouse> for DbExecutor {
    type Result = <DeleteWarehouse as Message>::Result;

    /// Defines how to handle the `DeleteWarehouse` message.
    ///
    /// Here we use the `name` from the `DeleteWarehouse` message to find the
    /// specified `Warehouse` record and delete it.
    fn handle(&mut self, msg: DeleteWarehouse, _: &mut Self::Context) -> Self::Result {
        use crate::schema::warehouses::dsl::*;
        let conn = &self.0.get().expect("should get db connection");

        diesel::delete(warehouses.filter(name.eq(msg.name)))
            .get_result::<Warehouse>(conn)
            .map(WarehouseResponse::from)
            .map_err(|_| "failed to delete warehouse".to_string())
    }
}

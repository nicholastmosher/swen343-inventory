use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::boxes;
use crate::app::products::{CreateProduct, UpdateProduct};
use std::convert::TryFrom;

/// An in-memory representation of a Box entity in the database.
#[derive(Debug, Queryable, Identifiable)]
#[table_name = "boxes"]
pub struct Box {
    pub id: i32,
    pub code: String,
    pub quantity: i32,
    pub pallet_id: String,
    pub deleted: bool,
}

/// The minimum information needed to create a new Box in the database.
#[derive(Debug, Insertable)]
#[table_name = "boxes"]
pub struct NewBox {
    pub code: String,
    pub quanitity: i32,
}

/// A representation of changes to a Box entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "boxes"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedBox {
    pub id: i32,
    pub code: String,
}

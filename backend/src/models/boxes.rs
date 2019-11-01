use std::convert::TryFrom;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::boxes;
use crate::app::v1::boxes::{CreateBox, UpdateBox};
use crate::models::pallets::Pallet;

/// An in-memory representation of a Box entity in the database.
#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Pallet)]
#[table_name = "boxes"]
pub struct Box {
    pub id: i32,
    pub pallet_id: i32,
    pub item_condition: String,
    pub item_quantity: i32,
    pub deleted: bool,
}

/// The minimum information needed to create a new Box in the database.
#[derive(Debug, Insertable)]
#[table_name = "boxes"]
pub struct NewBox {
    pub pallet_id: i32,
    pub item_condition: String,
    pub item_quantity: i32,
}



/// A representation of changes to a Box entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "boxes"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedBox {
    pub id: i32,
    pub item_condition: String,
    pub item_quantity: i32,
}

impl TryFrom<CreateBox> for NewBox {
    type Error = String;

    fn try_from(CreateBox { pallet_id, item_condition, item_quantity }: CreateBox) -> Result<Self, Self::Error> {
        if pallet_id < 0 { return Err("pallet_id cannot be less than 0".to_string()); }
        Ok(NewBox { pallet_id, item_condition, item_quantity })
    }
}

impl TryFrom<UpdateBox> for ChangedBox {
    type Error = String;

    fn try_from(UpdateBox { id, item_condition, item_quantity }: UpdateBox) -> Result<Self, Self::Error> {
        if item_quantity < 0 { return Err("cannot set item quantity in a box less than 0".to_string()); }
        Ok(ChangedBox { id, item_condition, item_quantity })
    }
}

use std::convert::TryFrom;
use diesel::{Queryable, Insertable, Identifiable, Associations};
use crate::schema::pallets;
use crate::app::v1::pallets::{CreatePallet, UpdatePallet};
use crate::models::warehouses::Warehouse;

/// An in-memory representation of a Pallet entity in the database.
#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Warehouse, foreign_key="warehouse_name")]
#[primary_key(id)]
pub struct Pallet {
    pub id: i32,
    pub item_code: String,
    pub warehouse_name: String,
    pub deleted: bool,
}

/// The minimum information needed to create a new Pallet in the database.
#[derive(Debug, Insertable)]
#[table_name = "pallets"]
pub struct NewPallet {
    pub item_code: String,
    pub warehouse_name: String,
}

/// A CreatePallet message from the webapp may be translated to a NewPallet
impl TryFrom<CreatePallet> for NewPallet {
    type Error = String;

    fn try_from(CreatePallet { item_code, warehouse_name }: CreatePallet) -> Result<Self, Self::Error> {
        Ok(NewPallet { item_code, warehouse_name })
    }
}

/// A representation of changes to a Pallet entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "pallets"]
pub struct ChangePallet {
    pub id: i32,
    pub item_code: String,
    pub warehouse_name: String,
}

/// An UpdatePallet message from the webapp may be translated to a ChangePallet
impl TryFrom<UpdatePallet> for ChangePallet {
    type Error = String;

    fn try_from(UpdatePallet { id, item_code, warehouse_name }: UpdatePallet) -> Result<Self, Self::Error> {
        Ok(ChangePallet { id, item_code, warehouse_name })
    }
}

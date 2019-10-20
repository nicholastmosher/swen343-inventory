use std::convert::TryFrom;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::rules;
use crate::app::rules::{CreateRule, UpdateRules};
use crate::models::warehouses::Warehouse;

/// An in-memory representation of a Box entity in the database.
#[derive(Debug, Queryable, Identifiable, Associations)]
#[belongs_to(Warehouse)]
pub struct Rule {
    pub id: i32,
    pub warehouse_id: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>,
    pub deleted: bool,
}

/// The minimum information needed to create a new Box in the database.
#[derive(Debug, Insertable)]
#[table_name = "rules"]
pub struct NewRule {
    pub warehouse_id: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>
}

/// A representation of changes to a Box entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "rules"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedRule {
    pub id: i32,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>
}

/*
impl TryFrom<CreateRule> for NewRule {
    type Error = String;

    /*
    fn try_from(CreateRule { pallet_id, item_quantity }: CreateRule) -> Result<Self, Self::Error> {
        if warehouse < 0 { return Err("pallet_id cannot be less than 0".to_string()); }
        Ok(NewRule { pallet_id, item_quantity })
    }*/
}

impl TryFrom<UpdateBox> for ChangedBox {
    type Error = String;

    fn try_from(UpdateBox { id, item_quantity }: UpdateBox) -> Result<Self, Self::Error> {
        if item_quantity < 0 { return Err("cannot set item quantity in a box less than 0".to_string()); }
        Ok(ChangedBox { id, item_quantity })
    }
}
*/
/*
/// A CreateRule message from the webapp may be translated to a NewRule
///
/// CreateRule is directly deserialized from the web request and contains
/// a u32 for the cost. If the value of the u32 is in the range of the i32
/// values, we can define a translation of the CreateRule message into the
/// NewRule type used to insert a new entity into the database.
///
/// If the cost value is out of bounds for an i32, we return an error value.
*/
/*
impl TryFrom<CreateRule> for NewRule {
    type Error = String;

    fn try_from(CreateRule { code, cost, description }: CreateRule) -> Result<Self, Self::Error> {
        if cost > std::i32::MAX as u32 { return Err("cost out of bounds".to_string()); }
        Ok(NewRule { code, cost: cost as i32, description })
    }
}

/// An UpdateRule message from the webapp may be translated to a ChangedRule
///
/// UpdateRule is directly deserialized from the web request and contains a
/// u32 for the cost. If the value of the u32 is in the range of i32 values,
/// we can define a translation of the UpdateRule message into the
/// ChangedRule type used to update an entity in the database.
///
/// If the cost value is out of bounds for an i32, we return an error value.
impl TryFrom<UpdateRules> for ChangedRule {
    type Error = String;

    fn try_from(UpdateRules { code, cost, description }: UpdateRules) -> Result<Self, Self::Error> {
        if cost > std::i32::MAX as u32 { return Err("cost out of bounds".to_string()); }
        Ok(ChangedRule { code, cost: cost as i32, description })
    }
}*/

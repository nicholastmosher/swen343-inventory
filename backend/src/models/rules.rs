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
    pub warehouse_id: String,
    pub item: String,
    pub minimum: i32,
    pub quantity: i32,
    pub description: Option<String>
}

impl TryFrom<CreateRule> for NewRule {
    type Error = String;

    fn try_from(CreateRule { id, warehouse_id, item, minimum, quantity, description }: CreateRule) -> Result<Self, Self::Error> {
        if id < 0 { return Err("id cannot be less than 0".to_string()); }
        Ok(NewRule { warehouse_id, item, minimum, quantity, description })
    }
}

impl TryFrom<UpdateRules> for ChangedRule {
    type Error = String;

    fn try_from(UpdateRules { id, warehouse_id, item, minimum, quantity, description }: UpdateRules) -> Result<Self, Self::Error> {
        if minimum < 0 || quantity < 0 { return Err("cannot set item quantity in a rule less than 0, or the minimum".to_string()); }
        Ok(ChangedRule { id, warehouse_id, item, minimum, quantity, description })
    }
}

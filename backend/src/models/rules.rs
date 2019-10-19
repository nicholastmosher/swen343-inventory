use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::rules;
use crate::app::rules::{CreateRule, UpdateRules};
use std::convert::TryFrom;

/// An in-memory representation of a Rule entity in the database.
#[derive(Debug, Queryable, Identifiable)]
#[primary_key(code)]
pub struct Rule {
    pub id: String,
    pub warehouse: u32,
    pub item: u32,
    pub minimum: u32,
    pub quantity: u32,
    pub description: Option<String>,
}

/// The minimum information needed to create a new Rule in the database.
#[derive(Debug, Insertable)]
#[table_name = "items"]
pub struct NewRule {
    pub warehouse: String,
    pub item: String,
    pub minimum: u32,
    pub quantity: u32,
    pub description: Option<String>,
}

/// A representation of changes to a Rule entity in the database.
#[derive(Debug, AsChangeset)]
#[table_name = "items"]
#[changeset_options(treat_none_as_null="true")]
pub struct ChangedRule {
    pub id: u32,
    pub warehouse: String,
    pub item: String,
    pub minimum: u32,
    pub quantity: u32,
    pub description: Option<String>,
}

/// A CreateRule message from the webapp may be translated to a NewRule
///
/// CreateRule is directly deserialized from the web request and contains
/// a u32 for the cost. If the value of the u32 is in the range of the i32
/// values, we can define a translation of the CreateRule message into the
/// NewRule type used to insert a new entity into the database.
///
/// If the cost value is out of bounds for an i32, we return an error value.
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
}

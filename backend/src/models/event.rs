//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "event")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub location: String,
    pub start_time: DateTime,
    pub end_time: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::event_invitee::Entity")]
    EventInvitee,
    #[sea_orm(has_many = "super::event_organization::Entity")]
    EventOrganization,
}

impl Related<super::event_invitee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventInvitee.def()
    }
}

impl Related<super::event_organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventOrganization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
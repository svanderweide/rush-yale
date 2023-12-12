//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[sea_orm(unique)]
    #[serde(skip_deserializing)]
    pub netid: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub location: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::event_invitee::Entity")]
    EventInvitee,
    #[sea_orm(has_many = "super::thread_message::Entity")]
    ThreadMessage,
    #[sea_orm(has_many = "super::thread_reader::Entity")]
    ThreadReader,
    #[sea_orm(has_many = "super::thread_writer::Entity")]
    ThreadWriter,
    #[sea_orm(has_many = "super::user_status::Entity")]
    UserStatus,
}

impl Related<super::event_invitee::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::EventInvitee.def()
    }
}

impl Related<super::thread_message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThreadMessage.def()
    }
}

impl Related<super::thread_reader::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThreadReader.def()
    }
}

impl Related<super::thread_writer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ThreadWriter.def()
    }
}

impl Related<super::user_status::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserStatus.def()
    }
}

// many-to-many relationship for Users and Events
impl Related<super::event::Entity> for Entity {
    fn to() -> RelationDef {
        super::event_invitee::Relation::Event.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::event_invitee::Relation::User.def().rev())
    }
}

// many-to-many relationship for Users and Threads
impl Related<super::thread::Entity> for Entity {
    fn to() -> RelationDef {
        super::thread_reader::Relation::Thread.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::thread_reader::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

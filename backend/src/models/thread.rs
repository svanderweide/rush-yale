//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "thread")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::thread_message::Entity")]
    ThreadMessage,
    #[sea_orm(has_many = "super::thread_reader::Entity")]
    ThreadReader,
    #[sea_orm(has_many = "super::thread_writer::Entity")]
    ThreadWriter,
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

impl ActiveModelBehavior for ActiveModel {}

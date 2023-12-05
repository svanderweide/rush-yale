use crate::models::thread::{self, Entity as Thread};
use sea_orm::*;

pub struct ThreadQuery;

impl ThreadQuery {
    pub async fn get_all_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        Thread::find()
            .select_only()
            .column(thread::Column::Id)
            .into_tuple::<i32>()
            .all(db)
            .await
    }

    pub async fn create(db: &DbConn, form: thread::Model) -> Result<thread::Model, DbErr> {
        thread::ActiveModel {
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn get(db: &DbConn, id: i32) -> Result<Option<thread::Model>, DbErr> {
        Thread::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DbConn, id: i32, form: thread::Model) -> Result<thread::Model, DbErr> {
        let thread = Thread::find_by_id(id).one(db).await?.unwrap();
        thread::ActiveModel {
            id: Unchanged(thread.id),
            ..Default::default()
        }
        .update(db)
        .await
    }
}

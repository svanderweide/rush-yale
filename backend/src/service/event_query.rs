use crate::models::event::{self, Entity as Event};
use sea_orm::*;

pub struct EventQuery;

impl EventQuery {
    pub async fn get_all_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        Event::find()
            .select_only()
            .column(event::Column::Id)
            .into_tuple::<i32>()
            .all(db)
            .await
    }

    pub async fn create(db: &DbConn, form: event::Model) -> Result<event::Model, DbErr> {
        event::ActiveModel {
            name: Set(form.name.to_owned()),
            description: Set(form.description.to_owned()),
            location: Set(form.location.to_owned()),
            start_time: Set(form.start_time.to_owned()),
            end_time: Set(form.end_time.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn get(db: &DbConn, id: i32) -> Result<Option<event::Model>, DbErr> {
        Event::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DbConn, id: i32, form: event::Model) -> Result<event::Model, DbErr> {
        let event = Event::find_by_id(id).one(db).await?.unwrap();
        event::ActiveModel {
            id: Unchanged(event.id),
            name: Set(form.name.to_owned()),
            description: Set(form.description.to_owned()),
            location: Set(form.location.to_owned()),
            start_time: Set(form.start_time.to_owned()),
            end_time: Set(form.end_time.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }
}

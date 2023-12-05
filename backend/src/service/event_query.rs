use crate::models::{
    event::{self, Entity as Event},
    event_organization::{self, Entity as EventOrganization},
    organization::{self, Entity as Organization},
};
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct EventQuery;

#[derive(Serialize)]
pub struct EventWithHosts {
    #[serde(flatten)]
    event: event::Model,
    hosts: Vec<organization::Model>,
}

#[derive(Deserialize)]
pub struct NewEventWithHosts {
    #[serde(flatten)]
    event: event::Model,
    hosts: Vec<i32>,
}

impl EventQuery {
    pub async fn get_all_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        Event::find()
            .select_only()
            .column(event::Column::Id)
            .into_tuple::<i32>()
            .all(db)
            .await
    }

    pub async fn create(db: &DbConn, json: NewEventWithHosts) -> Result<EventWithHosts, DbErr> {
        // begin transaction
        let txn = db.begin().await?;
        // create event!
        let event = event::ActiveModel {
            name: Set(json.event.name.to_owned()),
            description: Set(json.event.description.to_owned()),
            location: Set(json.event.location.to_owned()),
            start_time: Set(json.event.start_time.to_owned()),
            end_time: Set(json.event.end_time.to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        // add hosts!
        EventOrganization::insert_many(json.hosts.into_iter().map(|id| {
            event_organization::ActiveModel {
                event_id: Set(event.id),
                organization_id: Set(id),
                ..Default::default()
            }
        }))
        .exec(&txn)
        .await?;
        // commit transaction
        txn.commit().await?;
        EventQuery::get(db, event.id).await
    }

    pub async fn get(db: &DbConn, id: i32) -> Result<EventWithHosts, DbErr> {
        let event = Event::find_by_id(id).one(db).await?.unwrap();
        let hosts = event.find_related(Organization).all(db).await?;
        Ok(EventWithHosts { event, hosts })
    }

    pub async fn update(
        db: &DbConn,
        id: i32,
        json: NewEventWithHosts,
    ) -> Result<EventWithHosts, DbErr> {
        // get event and hosts
        let stored = EventQuery::get(&db, id).await?;
        let event = stored.event;
        // begin transaction
        let txn = db.begin().await?;
        // update event!
        let event = event::ActiveModel {
            id: Unchanged(event.id),
            name: Set(json.event.name.to_owned()),
            description: Set(json.event.description.to_owned()),
            location: Set(json.event.location.to_owned()),
            start_time: Set(json.event.start_time.to_owned()),
            end_time: Set(json.event.end_time.to_owned()),
        }
        .update(&txn)
        .await?;
        // remove current hosts
        EventOrganization::delete_many()
            .filter(event_organization::Column::EventId.eq(event.id))
            .exec(&txn)
            .await?;
        // add new hosts!
        EventOrganization::insert_many(json.hosts.into_iter().map(|id| {
            event_organization::ActiveModel {
                event_id: Set(event.id),
                organization_id: Set(id),
                ..Default::default()
            }
        }))
        .exec(&txn)
        .await?;
        // commit transaction
        txn.commit().await?;
        EventQuery::get(db, event.id).await
    }
}

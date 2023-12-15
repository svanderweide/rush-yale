use crate::models::{
    event::{self, Entity as Event},
    event_invitee::{self, Entity as EventInvitee},
    event_organization::{self, Entity as EventOrganization},
    organization::{self, Entity as Organization},
    user::{self, Entity as User},
};
use futures::stream::{self, StreamExt};
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct EventControl;

#[derive(Serialize)]
pub struct EventResponse {
    #[serde(flatten)]
    event: event::Model,
    hosts: Vec<organization::Model>,
    invitees: Vec<i32>,
}

#[derive(Deserialize)]
pub struct EventParams {
    #[serde(flatten)]
    event: event::Model,
    hosts: Vec<i32>,
    invitees: Vec<i32>,
}

impl EventControl {
    /// retrieves the IDs of all events in the database
    pub async fn get_event_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        Event::find()
            .select_only()
            .column(event::Column::Id)
            .into_tuple()
            .all(db)
            .await
    }

    /// creates a new event in the database
    pub async fn create_event(db: &DbConn, data: EventParams) -> Result<EventResponse, DbErr> {
        // begin transaction
        let txn = db.begin().await?;
        // create event!
        let event = event::ActiveModel {
            name: Set(data.event.name.to_owned()),
            description: Set(data.event.description.to_owned()),
            location: Set(data.event.location.to_owned()),
            start_time: Set(data.event.start_time.to_owned()),
            end_time: Set(data.event.end_time.to_owned()),
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        // add hosts!
        EventOrganization::insert_many(data.hosts.into_iter().map(|id| {
            event_organization::ActiveModel {
                event_id: Set(event.id),
                organization_id: Set(id),
                ..Default::default()
            }
        }))
        .exec(&txn)
        .await?;
        // add invitees! (only users)
        EventInvitee::insert_many(
            data.invitees
                .into_iter()
                .map(|id| event_invitee::ActiveModel {
                    event_id: Set(event.id),
                    user_id: Set(id),
                    ..Default::default()
                }),
        )
        .exec(&txn)
        .await?;
        // commit transaction
        txn.commit().await?;
        // find the event and return
        EventControl::get_event_by_id(db, event.id).await
    }

    /// retrieves the information for an event with the requested id
    pub async fn get_event_by_id(db: &DbConn, id: i32) -> Result<EventResponse, DbErr> {
        let event = Event::find_by_id(id).one(db).await?.unwrap();
        let hosts = event.find_related(Organization).all(db).await?;
        let invitees = event
            .find_related(User)
            .select_only()
            .column(user::Column::Id)
            .into_tuple()
            .all(db)
            .await?;
        Ok(EventResponse {
            event,
            hosts,
            invitees,
        })
    }

    /// retrieves the information for multiple events with the requested ids
    pub async fn get_events_by_id(db: &DbConn, ids: Vec<i32>) -> Result<Vec<EventResponse>, DbErr> {
        let events = Event::find()
            .filter(event::Column::Id.is_in(ids))
            .all(db)
            .await?;
        // find hosts and invitees for each Event requested
        // O(n) in the number of Event entities
        Ok(stream::iter(events)
            .then(|event| async move {
                let hosts = event.find_related(Organization).all(db).await.unwrap();
                let invitees = event
                    .find_related(User)
                    .select_only()
                    .column(user::Column::Id)
                    .into_tuple()
                    .all(db)
                    .await
                    .unwrap();
                EventResponse {
                    event,
                    hosts,
                    invitees,
                }
            })
            .collect()
            .await)
    }

    /// updates the information for the event with the requested id
    pub async fn update_event(
        db: &DbConn,
        id: i32,
        data: EventParams,
    ) -> Result<EventResponse, DbErr> {
        // get event and hosts
        let stored = EventControl::get_event_by_id(&db, id).await?;
        // begin transaction
        let txn = db.begin().await?;
        // update event!
        let event = event::ActiveModel {
            id: Unchanged(stored.event.id),
            name: Set(data.event.name.to_owned()),
            description: Set(data.event.description.to_owned()),
            location: Set(data.event.location.to_owned()),
            start_time: Set(data.event.start_time.to_owned()),
            end_time: Set(data.event.end_time.to_owned()),
        }
        .update(&txn)
        .await?;
        // remove current hosts
        EventOrganization::delete_many()
            .filter(event_organization::Column::EventId.eq(event.id))
            .exec(&txn)
            .await?;
        // add new hosts!
        EventOrganization::insert_many(data.hosts.into_iter().map(|id| {
            event_organization::ActiveModel {
                event_id: Set(event.id),
                organization_id: Set(id),
                ..Default::default()
            }
        }))
        .exec(&txn)
        .await?;
        // remove current invitees
        EventInvitee::delete_many()
            .filter(event_organization::Column::EventId.eq(event.id))
            .exec(&txn)
            .await?;
        // add new invitees!
        EventInvitee::insert_many(
            data.invitees
                .into_iter()
                .map(|id| event_invitee::ActiveModel {
                    event_id: Set(event.id),
                    user_id: Set(id),
                    ..Default::default()
                }),
        )
        .exec(&txn)
        .await?;
        // commit transaction
        txn.commit().await?;
        // find the event and return
        EventControl::get_event_by_id(db, event.id).await
    }
}

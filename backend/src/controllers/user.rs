use crate::models::{
    event::Entity as Event,
    organization::{self, Entity as Organization},
    thread::{self, Entity as Thread},
    user::{self, Entity as User},
    user_status::{self, Entity as UserStatus},
    user_status_option::{self, Entity as UserStatusOption},
};
use futures::stream::{self, StreamExt};
use sea_orm::*;
use serde::Serialize;

pub struct UserControl;

pub use user::Model as UserResponse;
pub use user::Model as UserParams;

#[derive(Serialize)]
pub struct UserOrganizationStatus {
    organization: organization::Model,
    status: user_status_option::Model,
}

use super::{EventControl, EventResponse};

impl UserControl {
    /// retrieves the IDs of all users in the database
    pub async fn get_user_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        User::find()
            .select_only()
            .column(user::Column::Id)
            .into_tuple()
            .all(db)
            .await
    }

    /// creates a new user in the database
    pub async fn create_user(db: &DbConn, data: UserParams) -> Result<UserResponse, DbErr> {
        user::ActiveModel {
            netid: Set(data.netid.to_owned()),
            first_name: Set(data.first_name.to_owned()),
            last_name: Set(data.last_name.to_owned()),
            email: Set(data.email.to_owned()),
            location: Set(data.location.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// retrieves the user with the requested id
    pub async fn get_user_by_id(db: &DbConn, id: i32) -> Result<UserResponse, DbErr> {
        Ok(User::find_by_id(id).one(db).await?.unwrap())
    }

    /// updates the information for the user with the requested id
    pub async fn update_user(
        db: &DbConn,
        id: i32,
        data: UserParams,
    ) -> Result<UserResponse, DbErr> {
        // find user
        let user = User::find_by_id(id).one(db).await?.unwrap();
        // update user information
        user::ActiveModel {
            id: Unchanged(user.id),
            netid: Unchanged(user.netid),
            first_name: Set(data.first_name.to_owned()),
            last_name: Set(data.last_name.to_owned()),
            email: Set(data.email.to_owned()),
            location: Set(data.location.to_owned()),
        }
        .update(db)
        .await
    }

    /// retrieves the events visible to the user with the requested id
    pub async fn get_user_events(db: &DbConn, id: i32) -> Result<Vec<EventResponse>, DbErr> {
        // find user
        let user = UserControl::get_user_by_id(db, id).await?;
        // find IDs of events
        let event_ids = user.find_related(Event).into_tuple().all(db).await?;
        // delegate to EventControl
        EventControl::get_events_by_id(db, event_ids).await
    }

    pub async fn get_user_statuses(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<UserOrganizationStatus>, DbErr> {
        // find all UserStatus for the organization
        let user_statuses = UserStatus::find()
            .filter(user_status::Column::UserId.eq(id))
            .all(db)
            .await?;
        // find organization and status for each UserStatus
        // O(n) in the number of UserStatus entities
        Ok(stream::iter(user_statuses)
            .then(|user_status| async move {
                let organization = Organization::find_by_id(user_status.organization_id)
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap();
                let status = UserStatusOption::find_by_id(user_status.user_status_option_id)
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap();
                UserOrganizationStatus {
                    organization,
                    status,
                }
            })
            .collect()
            .await)
    }

    pub async fn get_user_thread_ids(db: &DbConn, id: i32) -> Result<Vec<i32>, DbErr> {
        User::find_by_id(id)
            .one(db)
            .await?
            .unwrap()
            .find_related(Thread)
            .column(thread::Column::Id)
            .into_tuple()
            .all(db)
            .await
    }
}

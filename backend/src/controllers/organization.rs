use crate::models::{
    organization::{self, Entity as Organization, OrganizationToEvent},
    user::{self, Entity as User},
    user_status::{self, Entity as UserStatus},
    user_status_option::{self, Entity as UserStatusOption},
};
use futures::stream::{self, StreamExt};
use sea_orm::*;
use serde::Serialize;

pub struct OrganizationControl;

pub use organization::Model as OrganizationResponse;
pub use organization::Model as OrganizationParams;

#[derive(Serialize)]
pub struct OrganizationUserStatus {
    user: user::Model,
    status: user_status_option::Model,
}

use super::{EventControl, EventResponse};

impl OrganizationControl {
    /// retrieves the IDs of all organizations in the database
    pub async fn get_organization_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        Organization::find()
            .select_only()
            .column(organization::Column::Id)
            .into_tuple()
            .all(db)
            .await
    }

    /// creates a new organization in the database
    pub async fn create_organization(
        db: &DbConn,
        data: OrganizationParams,
    ) -> Result<OrganizationResponse, DbErr> {
        organization::ActiveModel {
            name: Set(data.name.to_owned()),
            email: Set(data.email.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// retrieves the information for the organization with the requested id
    pub async fn get_organization_by_id(
        db: &DbConn,
        id: i32,
    ) -> Result<OrganizationResponse, DbErr> {
        Ok(Organization::find_by_id(id).one(db).await?.unwrap())
    }

    /// updates the information for the organization with the requested id
    pub async fn update_organization(
        db: &DbConn,
        id: i32,
        data: OrganizationParams,
    ) -> Result<OrganizationResponse, DbErr> {
        // find organization
        let organization = OrganizationControl::get_organization_by_id(db, id).await?;
        // update information
        organization::ActiveModel {
            id: Unchanged(organization.id),
            name: Set(data.name.to_owned()),
            email: Set(data.email.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    /// retrieves the events hosted by the organization with the requested id
    pub async fn get_organization_events(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<EventResponse>, DbErr> {
        // find organization
        let organization = OrganizationControl::get_organization_by_id(db, id).await?;
        // find IDs of hosted events
        let event_ids = organization
            .find_linked(OrganizationToEvent)
            .into_tuple()
            .all(db)
            .await?;
        // delegate to EventControl
        EventControl::get_events_by_id(db, event_ids).await
    }

    /// retrieves the status of all users with a status with the organization with the requested id
    pub async fn get_organization_users(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<OrganizationUserStatus>, DbErr> {
        // find all UserStatus for the organization
        let user_statuses = UserStatus::find()
            .filter(user_status::Column::OrganizationId.eq(id))
            .all(db)
            .await?;
        // find user and status for each UserStatus
        // O(n) in the number of UserStatus entities
        Ok(stream::iter(user_statuses)
            .then(|user_status| async move {
                let user = User::find_by_id(user_status.user_id)
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap();
                let status = UserStatusOption::find_by_id(user_status.user_status_option_id)
                    .one(db)
                    .await
                    .unwrap()
                    .unwrap();
                OrganizationUserStatus { user, status }
            })
            .collect::<Vec<OrganizationUserStatus>>()
            .await)
    }

    pub async fn get_user_status(
        db: &DbConn,
        org_id: i32,
        usr_id: i32,
    ) -> Result<OrganizationUserStatus, DbErr> {
        // find user by id
        let user = User::find_by_id(usr_id).one(db).await?.unwrap();
        // find UserStatus for (user, organization) tuple
        let status = UserStatus::find_by_id((usr_id, org_id))
            .one(db)
            .await?
            .unwrap();
        // find status option
        let status = UserStatusOption::find_by_id(status.user_status_option_id)
            .one(db)
            .await?
            .unwrap();
        // return response
        Ok(OrganizationUserStatus { user, status })
    }

    pub async fn create_user_status(
        db: &DbConn,
        org_id: i32,
        usr_id: i32,
        data: i32,
    ) -> Result<OrganizationUserStatus, DbErr> {
        // find user by id
        let user = User::find_by_id(usr_id).one(db).await?.unwrap();
        // create UserStatus for (user, organization) tuple
        let status = user_status::ActiveModel {
            organization_id: Set(org_id),
            user_id: Set(usr_id),
            user_status_option_id: Set(data),
        }
        .insert(db)
        .await?;
        // find status option
        let status = UserStatusOption::find_by_id(status.user_status_option_id)
            .one(db)
            .await?
            .unwrap();
        // return response
        Ok(OrganizationUserStatus { user, status })
    }

    pub async fn update_user_status(
        db: &DbConn,
        org_id: i32,
        usr_id: i32,
        data: i32,
    ) -> Result<OrganizationUserStatus, DbErr> {
        // find user by id
        let user = User::find_by_id(usr_id).one(db).await?.unwrap();
        // find UserStatus
        let status = UserStatus::find_by_id((usr_id, org_id))
            .one(db)
            .await?
            .unwrap();
        // update UserStatus for (user, organization) tuple
        let status = user_status::ActiveModel {
            organization_id: Unchanged(status.organization_id),
            user_id: Unchanged(status.user_id),
            user_status_option_id: Set(data),
        }
        .update(db)
        .await?;
        // find status option
        let status = UserStatusOption::find_by_id(status.user_status_option_id)
            .one(db)
            .await?
            .unwrap();
        // return response
        Ok(OrganizationUserStatus { user, status })
    }
}

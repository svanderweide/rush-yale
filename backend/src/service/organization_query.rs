use crate::models::{
    organization::{self, Entity as Organization},
    user::{self, Entity as User},
    user_status::{self, Entity as UserStatus},
    user_status_option::{self, Entity as UserStatusOption},
};
use sea_orm::*;

pub struct OrganizationQuery;

impl OrganizationQuery {
    pub async fn get_all_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        Organization::find()
            .select_only()
            .column(organization::Column::Id)
            .into_tuple::<i32>()
            .all(db)
            .await
    }

    pub async fn create(
        db: &DbConn,
        form: organization::Model,
    ) -> Result<organization::Model, DbErr> {
        organization::ActiveModel {
            name: Set(form.name.to_owned()),
            email: Set(form.email.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn get(db: &DbConn, id: i32) -> Result<Option<organization::Model>, DbErr> {
        Organization::find_by_id(id).one(db).await
    }

    pub async fn update(
        db: &DbConn,
        id: i32,
        form: organization::Model,
    ) -> Result<organization::Model, DbErr> {
        let organization = OrganizationQuery::get(db, id).await?.unwrap();
        organization::ActiveModel {
            id: Unchanged(organization.id),
            name: Set(form.name.to_owned()),
            email: Set(form.email.to_owned()),
            ..Default::default()
        }
        .update(db)
        .await
    }

    pub async fn get_users(db: &DbConn, id: i32) -> Result<Vec<user::Model>, DbErr> {
        Organization::find_by_id(id)
            .one(db)
            .await?
            .unwrap()
            .find_related(User)
            .all(db)
            .await
    }

    pub async fn get_user_status(
        db: &DbConn,
        org_id: i32,
        usr_id: i32,
    ) -> Result<Option<user_status_option::Model>, DbErr> {
        UserStatus::find_by_id((org_id, usr_id))
            .one(db)
            .await?
            .unwrap()
            .find_related(UserStatusOption)
            .one(db)
            .await
    }

    pub async fn create_user_status(
        db: &DbConn,
        org_id: i32,
        usr_id: i32,
        form: user_status_option::Model,
    ) -> Result<user_status::Model, DbErr> {
        user_status::ActiveModel {
            organization_id: Set(org_id),
            user_id: Set(usr_id),
            user_status_option_id: Set(form.id),
        }
        .insert(db)
        .await
    }

    pub async fn update_user_status(
        db: &DbConn,
        org_id: i32,
        usr_id: i32,
        form: user_status_option::Model,
    ) -> Result<user_status::Model, DbErr> {
        let user_status = UserStatus::find_by_id((org_id, usr_id))
            .one(db)
            .await?
            .unwrap();
        user_status::ActiveModel {
            organization_id: Unchanged(user_status.organization_id),
            user_id: Unchanged(user_status.user_id),
            user_status_option_id: Set(form.id),
        }
        .update(db)
        .await
    }
}

use crate::models::{
    event::{self, Entity as Event},
    thread::{self, Entity as Thread},
    user::{self, Entity as User},
    user_status::{self, Entity as UserStatus},
};
use sea_orm::*;

pub struct UserQuery;

impl UserQuery {
    pub async fn get_all_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        User::find()
            .select_only()
            .column(user::Column::Id)
            .into_tuple::<i32>()
            .all(db)
            .await
    }

    pub async fn create(db: &DbConn, form: user::Model) -> Result<user::Model, DbErr> {
        user::ActiveModel {
            netid: Set(form.netid.to_owned()),
            first_name: Set(form.first_name.to_owned()),
            last_name: Set(form.last_name.to_owned()),
            email: Set(form.email.to_owned()),
            address: Set(form.address.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    pub async fn get(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn update(db: &DbConn, id: i32, form: user::Model) -> Result<user::Model, DbErr> {
        let user = User::find_by_id(id).one(db).await?.unwrap();
        user::ActiveModel {
            id: Unchanged(user.id),
            netid: Unchanged(user.netid),
            first_name: Set(form.first_name.to_owned()),
            last_name: Set(form.first_name.to_owned()),
            email: Set(form.first_name.to_owned()),
            address: Set(form.first_name.to_owned()),
        }
        .update(db)
        .await
    }

    pub async fn get_events(db: &DbConn, id: i32) -> Result<Vec<event::Model>, DbErr> {
        User::find_by_id(id)
            .one(db)
            .await?
            .unwrap()
            .find_related(Event)
            .all(db)
            .await
    }

    pub async fn get_statuses(db: &DbConn, id: i32) -> Result<Vec<user_status::Model>, DbErr> {
        User::find_by_id(id)
            .one(db)
            .await?
            .unwrap()
            .find_related(UserStatus)
            .all(db)
            .await
    }

    pub async fn get_thread_ids(db: &DbConn, id: i32) -> Result<Vec<i32>, DbErr> {
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

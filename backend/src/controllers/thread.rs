use crate::models::{
    thread::{self, Entity as Thread},
    thread_message::{self, Entity as ThreadMessage},
    thread_reader::{self, Entity as ThreadReader},
    thread_writer::{self, Entity as ThreadWriter},
    user::{self, Entity as User},
};
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ThreadMetadata {
    readers: Vec<i32>,
    writers: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct ThreadResponse {
    id: i32,
    readers: Vec<i32>,
    writers: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct ThreadMessageResponse {
    sender: Option<user::Model>,
    #[serde(flatten)]
    message: thread_message::Model,
}

pub struct ThreadControl;

impl ThreadControl {
    /// retrieves the IDs of all threads in the database
    pub async fn get_thread_ids(db: &DbConn) -> Result<Vec<i32>, DbErr> {
        Thread::find()
            .select_only()
            .column(thread::Column::Id)
            .into_tuple::<i32>()
            .all(db)
            .await
    }

    /// creates a new thread in the database
    pub async fn create_thread(db: &DbConn, data: ThreadMetadata) -> Result<ThreadResponse, DbErr> {
        // begin transaction
        let txn = db.begin().await?;
        // create thread
        let thread = thread::ActiveModel {
            ..Default::default()
        }
        .insert(&txn)
        .await?;
        // create readers
        ThreadReader::insert_many(
            data.readers
                .iter()
                .map(|&user_id| thread_reader::ActiveModel {
                    thread_id: Set(thread.id),
                    user_id: Set(user_id),
                    ..Default::default()
                }),
        )
        .exec(&txn)
        .await?;
        // create writers
        ThreadWriter::insert_many(
            data.writers
                .iter()
                .map(|&user_id| thread_writer::ActiveModel {
                    thread_id: Set(thread.id),
                    user_id: Set(user_id),
                    ..Default::default()
                }),
        )
        .exec(&txn)
        .await?;
        // commit transaction
        txn.commit().await?;
        // return response
        Ok(ThreadResponse {
            id: thread.id,
            readers: data.readers,
            writers: data.writers,
        })
    }

    /// retrieves the metadata for the thread with the requested id
    pub async fn get_thread_metadata(db: &DbConn, id: i32) -> Result<ThreadResponse, DbErr> {
        // find thread
        let thread = Thread::find_by_id(id).one(db).await?.unwrap();
        // find readers
        let readers = thread
            .find_linked(thread::ThreadToThreadReader)
            .into_tuple::<i32>()
            .all(db)
            .await?;
        // find writers
        let writers = thread
            .find_linked(thread::ThreadToThreadWriter)
            .into_tuple::<i32>()
            .all(db)
            .await?;
        // return response
        Ok(ThreadResponse {
            id: thread.id,
            readers,
            writers,
        })
    }

    /// updates the metadata for the thread with requested id
    pub async fn update_thread_metadata(
        db: &DbConn,
        id: i32,
        data: ThreadMetadata,
    ) -> Result<ThreadResponse, DbErr> {
        // find thread
        let thread = Thread::find_by_id(id).one(db).await?.unwrap();
        // begin transaction
        let txn = db.begin().await?;
        // update existing readers
        ThreadReader::delete_many()
            .filter(thread_reader::Column::ThreadId.eq(thread.id))
            .exec(&txn)
            .await?;
        ThreadReader::insert_many(
            data.readers
                .iter()
                .map(|&user_id| thread_reader::ActiveModel {
                    thread_id: Set(thread.id),
                    user_id: Set(user_id),
                    ..Default::default()
                }),
        )
        .exec(&txn)
        .await?;
        // update existing writers
        ThreadWriter::delete_many()
            .filter(thread_writer::Column::ThreadId.eq(thread.id))
            .exec(&txn)
            .await?;
        ThreadWriter::insert_many(
            data.writers
                .iter()
                .map(|&user_id| thread_writer::ActiveModel {
                    thread_id: Set(thread.id),
                    user_id: Set(user_id),
                    ..Default::default()
                }),
        )
        .exec(&txn)
        .await?;
        // commit transaction
        txn.commit().await?;
        // create response
        Ok(ThreadResponse {
            id: thread.id,
            readers: data.readers,
            writers: data.writers,
        })
    }

    /// retrieves all the messages for a given thread
    pub async fn get_thread_messages(
        db: &DbConn,
        id: i32,
    ) -> Result<Vec<ThreadMessageResponse>, DbErr> {
        // find thread
        let thread = Thread::find_by_id(id).one(db).await?.unwrap();
        // find messages
        let messages = ThreadMessage::find()
            .filter(thread_message::Column::ThreadId.eq(thread.id))
            .find_also_related(User)
            .all(db)
            .await?;
        // create responses
        Ok(messages
            .iter()
            .map(|(message, user)| ThreadMessageResponse {
                sender: user.clone(),
                message: message.clone(),
            })
            .collect())
    }

    /// retrieves all the messages for a given thread
    pub async fn create_thread_message(
        db: &DbConn,
        id: i32,
        contents: String,
    ) -> Result<ThreadMessageResponse, DbErr> {
        // find thread
        let thread = Thread::find_by_id(id).one(db).await?.unwrap();
        // create message
        let message = thread_message::ActiveModel {
            sender_id: Set(1),
            thread_id: Set(thread.id),
            contents: Set(contents),
            ..Default::default()
        }
        .insert(db)
        .await?;
        // find messages
        let message = ThreadMessage::find_by_id(message.id)
            .one(db)
            .await?
            .unwrap();
        let user = message.find_related(User).one(db).await?;
        // create responses
        Ok(ThreadMessageResponse {
            sender: user.clone(),
            message: message.clone(),
        })
    }
}

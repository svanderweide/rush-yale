use crate::{
    controllers::{
        ThreadControl, ThreadMessageParams, ThreadMessageResponse, ThreadParams, ThreadResponse,
    },
    errors::Error,
    AppState,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path, ServiceConfig},
};

/// public config holding private thread routes
pub fn thread_config(cfg: &mut ServiceConfig) {
    cfg.service(thread_get_all_ids)
        .service(thread_create)
        .service(thread_get)
        .service(thread_update)
        .service(thread_get_messages)
        .service(thread_create_message);
}

#[get("")]
async fn thread_get_all_ids(data: Data<AppState>) -> Result<Json<Vec<i32>>, Error> {
    let conn = &data.conn;
    let ids = ThreadControl::get_thread_ids(&conn).await?;
    Ok(Json(ids))
}

#[post("")]
async fn thread_create(
    data: Data<AppState>,
    json: Json<ThreadParams>,
) -> Result<Json<ThreadResponse>, Error> {
    let conn = &data.conn;
    let json = json.into_inner();
    let thread = ThreadControl::create_thread(&conn, json).await?;
    Ok(Json(thread))
}

#[get("/{id}")]
async fn thread_get(data: Data<AppState>, id: Path<i32>) -> Result<Json<ThreadResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let thread_metadata = ThreadControl::get_thread_metadata(&conn, id).await?;
    Ok(Json(thread_metadata))
}

#[put("/{id}")]
async fn thread_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<ThreadParams>,
) -> Result<Json<ThreadResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    let thread_metadata = ThreadControl::update_thread_metadata(&conn, id, json).await?;
    Ok(Json(thread_metadata))
}

#[get("/{id}/messages")]
async fn thread_get_messages(
    data: Data<AppState>,
    id: Path<i32>,
) -> Result<Json<Vec<ThreadMessageResponse>>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let thread_messages = ThreadControl::get_thread_messages(&conn, id).await?;
    Ok(Json(thread_messages))
}

#[post("/{id}/messages")]
async fn thread_create_message(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<ThreadMessageParams>,
) -> Result<Json<ThreadMessageResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    let thread_message = ThreadControl::create_thread_message(&conn, id, json).await?;
    Ok(Json(thread_message))
}

use crate::{
    controllers::{
        ThreadControl, ThreadMessageParams, ThreadMessageResponse, ThreadParams, ThreadResponse,
    },
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
async fn thread_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(ThreadControl::get_thread_ids(&conn).await.unwrap())
}

#[post("")]
async fn thread_create(data: Data<AppState>, json: Json<ThreadParams>) -> Json<ThreadResponse> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(ThreadControl::create_thread(&conn, json).await.unwrap())
}

#[get("/{id}")]
async fn thread_get(data: Data<AppState>, id: Path<i32>) -> Json<ThreadResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(ThreadControl::get_thread_metadata(&conn, id).await.unwrap())
}

#[put("/{id}")]
async fn thread_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<ThreadParams>,
) -> Json<ThreadResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    Json(
        ThreadControl::update_thread_metadata(&conn, id, json)
            .await
            .unwrap(),
    )
}

#[get("/{id}/messages")]
async fn thread_get_messages(
    data: Data<AppState>,
    id: Path<i32>,
) -> Json<Vec<ThreadMessageResponse>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(ThreadControl::get_thread_messages(&conn, id).await.unwrap())
}

#[post("/{id}/messages")]
async fn thread_create_message(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<ThreadMessageParams>,
) -> Json<ThreadMessageResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    Json(
        ThreadControl::create_thread_message(&conn, id, json)
            .await
            .unwrap(),
    )
}

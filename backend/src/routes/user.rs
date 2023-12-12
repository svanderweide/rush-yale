use crate::{
    controllers::{EventResponse, UserControl, UserOrganizationStatus, UserParams, UserResponse},
    AppState,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path, ServiceConfig},
};

/// public config holding private user routes
pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.service(user_get_all_ids)
        .service(user_create)
        .service(user_get)
        .service(user_update)
        .service(user_get_events)
        .service(user_get_statuses)
        .service(user_get_thread_ids);
}

#[get("")]
async fn user_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(UserControl::get_user_ids(&conn).await.unwrap())
}

#[post("")]
async fn user_create(data: Data<AppState>, json: Json<UserParams>) -> Json<UserResponse> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(UserControl::create_user(&conn, json).await.unwrap())
}

#[get("/{id}")]
async fn user_get(data: Data<AppState>, id: Path<i32>) -> Json<UserResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_by_id(&conn, id).await.unwrap())
}

#[put("/{id}")]
async fn user_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<UserParams>,
) -> Json<UserResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    Json(UserControl::update_user(&conn, id, json).await.unwrap())
}

#[get("/{id}/events")]
async fn user_get_events(data: Data<AppState>, id: Path<i32>) -> Json<Vec<EventResponse>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_events(&conn, id).await.unwrap())
}

#[get("/{id}/statuses")]
async fn user_get_statuses(
    data: Data<AppState>,
    id: Path<i32>,
) -> Json<Vec<UserOrganizationStatus>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_statuses(&conn, id).await.unwrap())
}

#[get("/{id}/threads")]
async fn user_get_thread_ids(data: Data<AppState>, id: Path<i32>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_thread_ids(&conn, id).await.unwrap())
}

use crate::{
    controllers::{EventResponse, UserControl, UserOrganizationStatus, UserParams, UserResponse},
    errors::Error,
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
async fn user_get_all_ids(data: Data<AppState>) -> Result<Json<Vec<i32>>, Error> {
    let conn = &data.conn;
    let ids = UserControl::get_user_ids(&conn).await?;
    Ok(Json(ids))
}

#[post("")]
async fn user_create(
    data: Data<AppState>,
    json: Json<UserParams>,
) -> Result<Json<UserResponse>, Error> {
    let conn = &data.conn;
    let json = json.into_inner();
    let user = UserControl::create_user(&conn, json).await?;
    Ok(Json(user))
}

#[get("/{id}")]
async fn user_get(data: Data<AppState>, id: Path<i32>) -> Result<Json<UserResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let user = UserControl::get_user_by_id(&conn, id).await?;
    Ok(Json(user))
}

#[put("/{id}")]
async fn user_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<UserParams>,
) -> Result<Json<UserResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    let user = UserControl::update_user(&conn, id, json).await?;
    Ok(Json(user))
}

#[get("/{id}/events")]
async fn user_get_events(
    data: Data<AppState>,
    id: Path<i32>,
) -> Result<Json<Vec<EventResponse>>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let events = UserControl::get_user_events(&conn, id).await?;
    Ok(Json(events))
}

#[get("/{id}/statuses")]
async fn user_get_statuses(
    data: Data<AppState>,
    id: Path<i32>,
) -> Result<Json<Vec<UserOrganizationStatus>>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let statuses = UserControl::get_user_statuses(&conn, id).await?;
    Ok(Json(statuses))
}

#[get("/{id}/threads")]
async fn user_get_thread_ids(data: Data<AppState>, id: Path<i32>) -> Result<Json<Vec<i32>>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let thread_ids = UserControl::get_user_thread_ids(&conn, id).await?;
    Ok(Json(thread_ids))
}

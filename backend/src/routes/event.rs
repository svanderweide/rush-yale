use crate::{
    controllers::{EventControl, EventParams, EventResponse},
    errors::Error,
    AppState,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path, ServiceConfig},
};

/// public config holding private event routes
pub fn event_config(cfg: &mut ServiceConfig) {
    cfg.service(event_get_all_ids)
        .service(event_create)
        .service(event_get)
        .service(event_update);
}

#[get("")]
async fn event_get_all_ids(data: Data<AppState>) -> Result<Json<Vec<i32>>, Error> {
    let conn = &data.conn;
    let ids = EventControl::get_event_ids(&conn).await?;
    Ok(Json(ids))
}

#[post("")]
async fn event_create(
    data: Data<AppState>,
    json: Json<EventParams>,
) -> Result<Json<EventResponse>, Error> {
    let conn = &data.conn;
    let json = json.into_inner();
    let event = EventControl::create_event(&conn, json).await?;
    Ok(Json(event))
}

#[get("/{id}")]
async fn event_get(data: Data<AppState>, id: Path<i32>) -> Result<Json<EventResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let event = EventControl::get_event_by_id(&conn, id).await?;
    Ok(Json(event))
}

#[put("/{id}")]
async fn event_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<EventParams>,
) -> Result<Json<EventResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    let event = EventControl::update_event(&conn, id, json).await?;
    Ok(Json(event))
}

use crate::{
    controllers::{EventControl, EventParams, EventResponse},
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
async fn event_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(EventControl::get_event_ids(&conn).await.unwrap())
}

#[post("")]
async fn event_create(data: Data<AppState>, json: Json<EventParams>) -> Json<EventResponse> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(EventControl::create_event(&conn, json).await.unwrap())
}

#[get("/{id}")]
async fn event_get(data: Data<AppState>, id: Path<i32>) -> Json<EventResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(EventControl::get_event_by_id(&conn, id).await.unwrap())
}

#[put("/{id}")]
async fn event_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<EventParams>,
) -> Json<EventResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    Json(EventControl::update_event(&conn, id, json).await.unwrap())
}

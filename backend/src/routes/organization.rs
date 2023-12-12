use crate::{
    controllers::{
        EventResponse, OrganizationControl, OrganizationParams, OrganizationResponse,
        OrganizationUserStatus,
    },
    AppState,
};
use actix_web::{
    get, post, put,
    web::{Data, Json, Path, ServiceConfig},
};

/// public config holding private organization routes
pub fn organization_config(cfg: &mut ServiceConfig) {
    cfg.service(organization_get_all_ids)
        .service(organization_create)
        .service(organization_get)
        .service(organization_update)
        .service(organization_get_events)
        .service(organization_get_users)
        .service(organization_get_user_status)
        .service(organization_create_user_status)
        .service(organization_update_user_status);
}

#[get("")]
async fn organization_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(
        OrganizationControl::get_organization_ids(&conn)
            .await
            .unwrap(),
    )
}

#[post("")]
async fn organization_create(
    data: Data<AppState>,
    json: Json<OrganizationParams>,
) -> Json<OrganizationResponse> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(
        OrganizationControl::create_organization(&conn, json)
            .await
            .unwrap(),
    )
}

#[get("/{id}")]
async fn organization_get(data: Data<AppState>, id: Path<i32>) -> Json<OrganizationResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(
        OrganizationControl::get_organization_by_id(&conn, id)
            .await
            .unwrap(),
    )
}

#[put("/{id}")]
async fn organization_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<OrganizationParams>,
) -> Json<OrganizationResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    Json(
        OrganizationControl::update_organization(&conn, id, json)
            .await
            .unwrap(),
    )
}

#[get("/{id}/events")]
async fn organization_get_events(data: Data<AppState>, id: Path<i32>) -> Json<Vec<EventResponse>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(
        OrganizationControl::get_organization_events(&conn, id)
            .await
            .unwrap(),
    )
}

#[get("/{id}/users")]
async fn organization_get_users(
    data: Data<AppState>,
    id: Path<i32>,
) -> Json<Vec<OrganizationUserStatus>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(
        OrganizationControl::get_organization_users(&conn, id)
            .await
            .unwrap(),
    )
}

#[get("/{org_id}/users/{usr_id}")]
async fn organization_get_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
) -> Json<OrganizationUserStatus> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    Json(
        OrganizationControl::get_user_status(&conn, org_id, usr_id)
            .await
            .unwrap(),
    )
}

#[post("/{org_id}/users/{usr_id}")]
async fn organization_create_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
    json: Json<i32>,
) -> Json<OrganizationUserStatus> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    let json = json.into_inner();
    Json(
        OrganizationControl::create_user_status(&conn, org_id, usr_id, json)
            .await
            .unwrap(),
    )
}

#[put("/{org_id}/users/{usr_id}")]
async fn organization_update_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
    json: Json<i32>,
) -> Json<OrganizationUserStatus> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    let json = json.into_inner();
    Json(
        OrganizationControl::update_user_status(&conn, org_id, usr_id, json)
            .await
            .unwrap(),
    )
}

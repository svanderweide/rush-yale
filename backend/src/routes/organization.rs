use crate::{
    controllers::{
        EventResponse, OrganizationControl, OrganizationParams, OrganizationResponse,
        OrganizationUserStatus,
    },
    errors::Error,
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
async fn organization_get_all_ids(data: Data<AppState>) -> Result<Json<Vec<i32>>, Error> {
    let conn = &data.conn;
    let ids = OrganizationControl::get_organization_ids(&conn).await?;
    Ok(Json(ids))
}

#[post("")]
async fn organization_create(
    data: Data<AppState>,
    json: Json<OrganizationParams>,
) -> Result<Json<OrganizationResponse>, Error> {
    let conn = &data.conn;
    let json = json.into_inner();
    let organization = OrganizationControl::create_organization(&conn, json).await?;
    Ok(Json(organization))
}

#[get("/{id}")]
async fn organization_get(
    data: Data<AppState>,
    id: Path<i32>,
) -> Result<Json<OrganizationResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let organization = OrganizationControl::get_organization_by_id(&conn, id).await?;
    Ok(Json(organization))
}

#[put("/{id}")]
async fn organization_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<OrganizationParams>,
) -> Result<Json<OrganizationResponse>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    let organization = OrganizationControl::update_organization(&conn, id, json).await?;
    Ok(Json(organization))
}

#[get("/{id}/events")]
async fn organization_get_events(
    data: Data<AppState>,
    id: Path<i32>,
) -> Result<Json<Vec<EventResponse>>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let events = OrganizationControl::get_organization_events(&conn, id).await?;
    Ok(Json(events))
}

#[get("/{id}/users")]
async fn organization_get_users(
    data: Data<AppState>,
    id: Path<i32>,
) -> Result<Json<Vec<OrganizationUserStatus>>, Error> {
    let conn = &data.conn;
    let id = id.into_inner();
    let users = OrganizationControl::get_organization_users(&conn, id).await?;
    Ok(Json(users))
}

#[get("/{org_id}/users/{usr_id}")]
async fn organization_get_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
) -> Result<Json<OrganizationUserStatus>, Error> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    let user_status = OrganizationControl::get_user_status(&conn, org_id, usr_id).await?;
    Ok(Json(user_status))
}

#[post("/{org_id}/users/{usr_id}")]
async fn organization_create_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
    json: Json<i32>,
) -> Result<Json<OrganizationUserStatus>, Error> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    let json = json.into_inner();
    let user_status = OrganizationControl::create_user_status(&conn, org_id, usr_id, json).await?;
    Ok(Json(user_status))
}

#[put("/{org_id}/users/{usr_id}")]
async fn organization_update_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
    json: Json<i32>,
) -> Result<Json<OrganizationUserStatus>, Error> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    let json = json.into_inner();
    let user_status = OrganizationControl::update_user_status(&conn, org_id, usr_id, json).await?;
    Ok(Json(user_status))
}

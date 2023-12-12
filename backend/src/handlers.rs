use crate::{controllers::*, AppState};
use actix_identity::Identity;
use actix_web::{
    get, post, put,
    web::{Data, Json, Path, Query, Redirect},
    HttpMessage, HttpRequest, HttpResponse,
};
use serde::Deserialize;

#[get("/health-check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct AuthResource {
    ticket: Option<String>,
}

#[get("/login")]
async fn login(req: HttpRequest, data: Data<AppState>, auth: Query<AuthResource>) -> Redirect {
    let auth = auth.into_inner();
    match auth.ticket {
        None => {
            // CAS authentication
            Redirect::to(format!(
                "https://secure6.its.yale.edu/cas/login?service=https://{}/api/auth/login",
                &data.base_url
            ))
        }
        Some(ticket) => {
            // CAS authentication
            let cas_url = format!(
                "https://secure6.its.yale.edu/cas/validate?service=https://{}/api/auth/login&ticket={}",
                &data.base_url, ticket
            );
            let body = reqwest::get(cas_url).await.unwrap().text().await.unwrap();
            // result format = "yes\n<netid>\n" or "no\n\n"
            match body.lines().nth(1) {
                // authentication success!
                Some(netid) => {
                    Identity::login(&req.extensions(), netid.to_owned()).unwrap();
                    Redirect::to("/health-check")
                }
                // authentication failure!
                None => Redirect::to("/login"),
            }
        }
    }
}

#[get("/logout")]
async fn logout(user: Identity) -> HttpResponse {
    user.logout();
    HttpResponse::Ok().finish()
}

#[get("/events")]
async fn event_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(EventControl::get_event_ids(&conn).await.unwrap())
}

#[post("/events")]
async fn event_create(data: Data<AppState>, json: Json<EventParams>) -> Json<EventResponse> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(EventControl::create_event(&conn, json).await.unwrap())
}

#[get("/events/{id}")]
async fn event_get(data: Data<AppState>, id: Path<i32>) -> Json<EventResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(EventControl::get_event_by_id(&conn, id).await.unwrap())
}

#[put("/events/{id}")]
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

#[get("/organizations")]
async fn organization_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(
        OrganizationControl::get_organization_ids(&conn)
            .await
            .unwrap(),
    )
}

#[post("/organizations")]
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

#[get("/organizations/{id}")]
async fn organization_get(data: Data<AppState>, id: Path<i32>) -> Json<OrganizationResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(
        OrganizationControl::get_organization_by_id(&conn, id)
            .await
            .unwrap(),
    )
}

#[put("/organizations/{id}")]
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

#[get("/organizations/{id}/events")]
async fn organization_get_events(data: Data<AppState>, id: Path<i32>) -> Json<Vec<EventResponse>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(
        OrganizationControl::get_organization_events(&conn, id)
            .await
            .unwrap(),
    )
}

#[get("/organizations/{id}/users")]
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

#[get("/organizations/{org_id}/users/{usr_id}")]
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

#[post("/organizations/{org_id}/users/{usr_id}")]
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

#[put("/organizations/{org_id}/users/{usr_id}")]
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

#[get("/threads")]
async fn thread_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(ThreadControl::get_thread_ids(&conn).await.unwrap())
}

#[post("/threads")]
async fn thread_create(data: Data<AppState>, json: Json<ThreadMetadata>) -> Json<ThreadResponse> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(ThreadControl::create_thread(&conn, json).await.unwrap())
}

#[get("/threads/{id}")]
async fn thread_get(data: Data<AppState>, id: Path<i32>) -> Json<ThreadResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(ThreadControl::get_thread_metadata(&conn, id).await.unwrap())
}

#[put("/threads/{id}")]
async fn thread_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<ThreadMetadata>,
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

#[get("/threads/{id}/messages")]
async fn thread_get_messages(
    data: Data<AppState>,
    id: Path<i32>,
) -> Json<Vec<ThreadMessageResponse>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(ThreadControl::get_thread_messages(&conn, id).await.unwrap())
}

#[post("/threads/{id}/messages")]
async fn thread_create_message(
    netid: Identity,
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<String>,
) -> Json<ThreadMessageResponse> {
    let netid = netid.id().unwrap();
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    Json(
        ThreadControl::create_thread_message(&conn, id, json, netid)
            .await
            .unwrap(),
    )
}

#[get("/users")]
async fn user_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(UserControl::get_user_ids(&conn).await.unwrap())
}

#[post("/users")]
async fn user_create(data: Data<AppState>, json: Json<UserParams>) -> Json<UserResponse> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(UserControl::create_user(&conn, json).await.unwrap())
}

#[get("/users/{id}")]
async fn user_get(data: Data<AppState>, id: Path<i32>) -> Json<UserResponse> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_by_id(&conn, id).await.unwrap())
}

#[put("/users/{id}")]
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

#[get("/users/{id}/events")]
async fn user_get_events(data: Data<AppState>, id: Path<i32>) -> Json<Vec<EventResponse>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_events(&conn, id).await.unwrap())
}

#[get("/users/{id}/statuses")]
async fn user_get_statuses(
    data: Data<AppState>,
    id: Path<i32>,
) -> Json<Vec<UserOrganizationStatus>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_statuses(&conn, id).await.unwrap())
}

#[get("/users/{id}/threads")]
async fn user_get_thread_ids(data: Data<AppState>, id: Path<i32>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserControl::get_user_thread_ids(&conn, id).await.unwrap())
}

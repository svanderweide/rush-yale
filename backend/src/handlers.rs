use crate::{
    models::{event, organization, user, user_status, user_status_option},
    service::*,
    AppState,
};
use actix_identity::Identity;
use actix_web::{
    get, post, put,
    web::{Data, Form, Json, Path, Query, Redirect},
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
                "https://secure6.its.yale.edu/cas/login?service=https://{}/login",
                &data.base_url
            ))
        }
        Some(ticket) => {
            // CAS authentication
            let cas_url = format!(
                "https://secure6.its.yale.edu/cas/validate?service=https://{}/login&ticket={}",
                &data.base_url, ticket
            );
            let body = reqwest::get(cas_url).await.unwrap().text().await.unwrap();
            if body.contains("yes") {
                // authenticated!
                Identity::login(&req.extensions(), "netid".to_string()).unwrap();
                Redirect::to("/health-check")
            } else {
                // authentication failed!
                Redirect::to("/login")
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
    Json(EventQuery::get_all_ids(&conn).await.unwrap())
}

#[post("/events")]
async fn event_create(data: Data<AppState>, json: Json<NewEventWithHosts>) -> Json<EventWithHosts> {
    let conn = &data.conn;
    let json = json.into_inner();
    Json(EventQuery::create(&conn, json).await.unwrap())
}

#[get("/events/{id}")]
async fn event_get(data: Data<AppState>, id: Path<i32>) -> Json<EventWithHosts> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(EventQuery::get(&conn, id).await.unwrap())
}

#[put("/events/{id}")]
async fn event_update(
    data: Data<AppState>,
    id: Path<i32>,
    json: Json<NewEventWithHosts>,
) -> Json<EventWithHosts> {
    let conn = &data.conn;
    let id = id.into_inner();
    let json = json.into_inner();
    Json(EventQuery::update(&conn, id, json).await.unwrap())
}

#[get("/organizations")]
async fn organization_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(OrganizationQuery::get_all_ids(&conn).await.unwrap())
}

#[post("/organizations")]
async fn organization_create(
    data: Data<AppState>,
    form: Form<organization::Model>,
) -> Json<organization::Model> {
    let conn = &data.conn;
    let form = form.into_inner();
    Json(OrganizationQuery::create(&conn, form).await.unwrap())
}

#[get("/organizations/{id}")]
async fn organization_get(data: Data<AppState>, id: Path<i32>) -> Json<organization::Model> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(OrganizationQuery::get(&conn, id).await.unwrap().unwrap())
}

#[put("/organizations/{id}")]
async fn organization_update(
    data: Data<AppState>,
    id: Path<i32>,
    form: Form<organization::Model>,
) -> Json<organization::Model> {
    let conn = &data.conn;
    let id = id.into_inner();
    let form = form.into_inner();
    Json(OrganizationQuery::update(&conn, id, form).await.unwrap())
}

#[get("/organizations/{id}/users")]
async fn organization_get_all_users(data: Data<AppState>, id: Path<i32>) -> Json<Vec<user::Model>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(OrganizationQuery::get_users(&conn, id).await.unwrap())
}

#[get("/organization/{org_id}/users/{usr_id}")]
async fn organization_get_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
) -> Json<user_status_option::Model> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    Json(
        OrganizationQuery::get_user_status(&conn, org_id, usr_id)
            .await
            .unwrap()
            .unwrap(),
    )
}

#[post("/organization/{org_id}/users/{usr_id}")]
async fn organization_create_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
    form: Form<user_status_option::Model>,
) -> Json<user_status::Model> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    let form = form.into_inner();
    Json(
        OrganizationQuery::create_user_status(&conn, org_id, usr_id, form)
            .await
            .unwrap(),
    )
}

#[put("/organization/{org_id}/users/{usr_id}")]
async fn organization_update_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
    form: Form<user_status_option::Model>,
) -> Json<user_status::Model> {
    let conn = &data.conn;
    let (org_id, usr_id) = ids.into_inner();
    let form = form.into_inner();
    Json(
        OrganizationQuery::update_user_status(&conn, org_id, usr_id, form)
            .await
            .unwrap(),
    )
}

#[get("/threads")]
async fn thread_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(ThreadQuery::get_all_ids(&conn).await.unwrap())
}

#[post("/threads")]
async fn thread_create(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
}

#[get("/threads/{id}")]
async fn thread_get(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[post("/threads/{id}")]
async fn thread_create_message(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[get("/users")]
async fn user_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(UserQuery::get_all_ids(&conn).await.unwrap())
}

#[post("/users")]
async fn user_create(data: Data<AppState>, form: Form<user::Model>) -> Json<user::Model> {
    let conn = &data.conn;
    let form = form.into_inner();
    Json(UserQuery::create(&conn, form).await.unwrap())
}

#[get("/users/{id}")]
async fn user_get(data: Data<AppState>, id: Path<i32>) -> Json<user::Model> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserQuery::get(&conn, id).await.unwrap().unwrap())
}

#[put("/users/{id}")]
async fn user_update(
    data: Data<AppState>,
    id: Path<i32>,
    form: Form<user::Model>,
) -> Json<user::Model> {
    let conn = &data.conn;
    let id = id.into_inner();
    let form = form.into_inner();
    Json(UserQuery::update(&conn, id, form).await.unwrap())
}

#[get("/users/{id}/events")]
async fn user_get_events(data: Data<AppState>, id: Path<i32>) -> Json<Vec<event::Model>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserQuery::get_events(&conn, id).await.unwrap())
}

#[get("/users/{id}/statuses")]
async fn user_get_statuses(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[get("/users/{id}/threads")]
async fn user_get_thread_ids(data: Data<AppState>, id: Path<i32>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    let id = id.into_inner();
    Json(UserQuery::get_thread_ids(&conn, id).await.unwrap())
}

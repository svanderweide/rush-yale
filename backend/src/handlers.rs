use crate::{models::event, service::*, AppState};
use actix_web::{
    get, post, put,
    web::{Data, Form, Json, Path},
    HttpResponse,
};

#[get("/health-check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/events")]
async fn event_get_all_ids(data: Data<AppState>) -> Json<Vec<i32>> {
    let conn = &data.conn;
    Json(EventQuery::get_all_ids(&conn).await.unwrap())
}

#[post("/events")]
async fn event_create(data: Data<AppState>, form: Form<NewEventWithHosts>) -> Json<EventWithHosts> {
    let conn = &data.conn;
    let form = form.into_inner();
    Json(EventQuery::create(&conn, form).await.unwrap())
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
    form: Form<NewEventWithHosts>,
) -> Json<EventWithHosts> {
    let conn = &data.conn;
    let id = id.into_inner();
    let form = form.into_inner();
    Json(EventQuery::update(&conn, id, form).await.unwrap())
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
async fn organization_get_all_users(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[get("/organization/{org_id}/users/{usr_id}")]
async fn organization_get_user_status(data: Data<AppState>, ids: Path<(i32, i32)>) -> HttpResponse {
    let _conn = &data.conn;
    let (_org_id, _usr_id) = ids.into_inner();
    todo!()
}

#[post("/organization/{org_id}/users/{usr_id}")]
async fn organization_create_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
) -> HttpResponse {
    let _conn = &data.conn;
    let (_org_id, _usr_id) = ids.into_inner();
    todo!()
}

#[put("/organization/{org_id}/users/{usr_id}")]
async fn organization_update_user_status(
    data: Data<AppState>,
    ids: Path<(i32, i32)>,
) -> HttpResponse {
    let _conn = &data.conn;
    let (_org_id, _usr_id) = ids.into_inner();
    todo!()
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
async fn user_create(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
}

#[get("/users/{id}")]
async fn user_get(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[put("/users/{id}")]
async fn user_update(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[get("/users/{id}/events")]
async fn user_get_events(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[get("/users/{id}/statuses")]
async fn user_get_statuses(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[get("/users/{id}/threads")]
async fn user_get_thread_ids(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

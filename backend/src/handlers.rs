use crate::AppState;
use actix_web::{
    get, post, put,
    web::{Data, Path},
    HttpResponse,
};

#[get("/health-check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/events")]
async fn event_get_all_ids(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
}

#[post("/events")]
async fn event_create(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
}

#[get("/events/{id}")]
async fn event_get(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[put("/events/{id}")]
async fn event_update(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[get("/organizations")]
async fn organization_get_all_ids(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
}

#[post("/organizations")]
async fn organization_create(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
}

#[get("/organizations/{id}")]
async fn organization_get(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
}

#[put("/organizations/{id}")]
async fn organization_update(data: Data<AppState>, id: Path<i32>) -> HttpResponse {
    let _conn = &data.conn;
    let _id = id.into_inner();
    todo!()
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
async fn thread_get_all_ids(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
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
async fn user_get_all_ids(data: Data<AppState>) -> HttpResponse {
    let _conn = &data.conn;
    todo!()
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

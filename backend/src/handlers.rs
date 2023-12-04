use actix_web::web::{scope, ServiceConfig};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(routes::health_check)
        .service(scope("/api").configure(api_config));
}

pub fn api_config(cfg: &mut ServiceConfig) {
    cfg.service(routes::user_get_all_ids);
}

mod routes {

    use actix_web::{get, post, put, web::Path, HttpResponse};

    #[get("/health-check")]
    async fn health_check() -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    #[get("/events")]
    async fn event_get_all_ids() -> HttpResponse {
        todo!()
    }

    #[post("/events")]
    async fn event_create() -> HttpResponse {
        todo!()
    }

    #[get("/events/{id}")]
    async fn event_get(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[put("/events/{id}")]
    async fn event_update(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[get("/organizations")]
    async fn organization_get_all_ids() -> HttpResponse {
        todo!()
    }

    #[post("/organizations")]
    async fn organization_create() -> HttpResponse {
        todo!()
    }

    #[get("/organizations/{id}")]
    async fn organization_get(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[put("/organizations/{id}")]
    async fn organization_update(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[get("/organizations/{id}/users")]
    async fn organization_get_all_users(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[get("/organization/{org_id}/users/{usr_id}")]
    async fn organization_get_user_status(_ids: Path<(i32, i32)>) -> HttpResponse {
        todo!()
    }

    #[post("/organization/{org_id}/users/{usr_id}")]
    async fn organization_create_user_status(_ids: Path<(i32, i32)>) -> HttpResponse {
        todo!()
    }

    #[put("/organization/{org_id}/users/{usr_id}")]
    async fn organization_update_user_status(_ids: Path<(i32, i32)>) -> HttpResponse {
        todo!()
    }

    #[get("/threads")]
    async fn thread_get_all_ids() -> HttpResponse {
        todo!()
    }

    #[post("/threads")]
    async fn thread_create() -> HttpResponse {
        todo!()
    }

    #[get("/threads/{id}")]
    async fn thread_get(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[post("/threads/{id}")]
    async fn thread_create_message(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[get("/users")]
    async fn user_get_all_ids() -> HttpResponse {
        todo!()
    }

    #[post("/users")]
    async fn user_create() -> HttpResponse {
        todo!()
    }

    #[get("/users/{id}")]
    async fn user_get(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[put("/users/{id}")]
    async fn user_update(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[get("/users/{id}/events")]
    async fn user_get_events(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[get("/users/{id}/statuses")]
    async fn user_get_all_statuses(_id: Path<i32>) -> HttpResponse {
        todo!()
    }

    #[get("/users/{id}/threads")]
    async fn user_get_thread_ids(_id: Path<i32>) -> HttpResponse {
        todo!()
    }
}

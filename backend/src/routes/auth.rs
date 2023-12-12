use crate::AppState;
use actix_identity::Identity;
use actix_web::{
    get,
    web::{Data, Query, Redirect, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse,
};
use serde::Deserialize;

/// public config holding private auth routes
pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.service(login).service(logout);
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
                    Redirect::to("/logout")
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

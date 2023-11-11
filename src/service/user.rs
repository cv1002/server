use actix_web::{post, web, Responder};
use serde::{Deserialize, Serialize};

use crate::utils::transform::Transformation;

#[derive(Deserialize)]
struct RegisterArgs {
    _name: String,
    _password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    ok: bool,
    error: String,
    token: String,
}

/// Default handler
#[post("/user/register")]
async fn register(_arg: web::Json<RegisterArgs>) -> impl Responder {
    let _ = crate::database::extend::hello().await;
    RegisterResponse {
        ok: true,
        error: "".to_string(),
        token: "".to_string(),
    }
    .transformation(web::Json)
}

use crate::{config, utils::transform::Transformation};

use actix_web::{dev::Server, get, middleware, web, App, HttpServer, Responder};

/// Use this function to initialize acatix
pub fn actix_config_initalize() -> Server {
    // 获取监听的端口
    let port = config::get_config().port();
    // let database = config::get_config().
    // 初始化ActixWeb
    HttpServer::new(|| {
        App::new()
            .configure(router)
            .wrap(middleware::Logger::default())
    })
    // HTTP初始化
    .transformation(|server| server.bind(("0.0.0.0", port)).unwrap())
    // 启动程序
    .run()
}

/// Use this function to initialize routers
fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(fake_service);
}

/// Default handler
#[get("/")]
async fn fake_service() -> impl Responder {
    web::Json("Hello world.")
}

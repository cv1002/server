mod config;
mod database;
mod service;
mod utils;

#[tokio::main]
async fn main() {
    // 初始化Tracing
    utils::logger::tracing_config_initalize();
    // 初始化ActixWeb
    service::actix_config_initalize().await.unwrap();
}

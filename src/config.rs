use crate::{utils::lazy::Lazy, LazyNew};

use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};

/// Configuration file format
/// # Example
///
/// ```ron
/// AppConfig(
///     port: 8888,
///     database: DatabaseConfig(
///         user: "****",
///         host: "****:3306",
///         database: "****",
///         password: "****",
///     ),
/// )
/// ```
#[derive(Deserialize, Serialize, Getters, CopyGetters)]
pub struct AppConfig {
    #[getset(get_copy = "pub")]
    port: u16,

    #[getset(get = "pub")]
    database: DatabaseConfig,
}

#[non_exhaustive]
#[derive(Deserialize, Serialize, CopyGetters)]
pub struct DatabaseConfig {
    pub user: String,
    pub host: String,
    pub database: String,
    pub password: String,
}

static CONFIG: Lazy<AppConfig> = LazyNew! {
    config::Config::builder()
        .add_source(config::File::with_name("./config/config.ron"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
};

pub fn get_config() -> &'static AppConfig {
    &CONFIG
}

use sea_orm::{ActiveModelTrait, DbErr};

use crate::{
    active_model,
    database::{model::preclude::*, CONN_POLL},
    utils::inspect::*,
};

pub async fn hello() -> Result<UserModel, DbErr> {
    active_model!(user {
        id: 1,
        name: "Hello".to_string(),
        password: "World".to_string(),
        rank: 0,
    })
    .insert(CONN_POLL.get().await)
    .await
    .inspect_error(|err| log::error!("{}", err.to_string()))
}

use crate::app::server::database::*;
use leptos::prelude::ServerFnError;
use leptos::server_fn::server;
use std::env;

#[server(GetProfile, "/api")]
pub async fn get_profile_api() -> Result<i64, ServerFnError> {
    let num_users: i64 = get_num_users(&establish_pool());
    Ok(num_users)
}

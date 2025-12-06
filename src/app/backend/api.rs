use crate::app::backend::database::*;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server(GetNumUsers, "/api")]
pub async fn get_num_users_api() -> Result<i64, ServerFnError> {
    let num_users: i64 = get_num_users(&establish_pool());
    Ok(num_users)
}

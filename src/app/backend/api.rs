use crate::app::backend::database::*;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server(GetNumUsers, "/api")]
pub async fn get_num_users_api() -> Result<i64, ServerFnError> {
    let num_users: i64 = get_num_users(&establish_pool());
    Ok(num_users)
}

// #[cfg(feature = "ssr")]
// #[server((name = "GetEventById", endpoint = "/api"))]
// pub async fn get_event_by_id_api(event_id: i32) -> Result<Option<Event>, ServerFnError> {
//     let event = get_event_by_id(&establish_pool(), event_id);
//     Ok(event)
// }

#[cfg(not(target_arch = "wasm32"))]
use crate::app::backend::database::{
    establish_pool, 
    get_event_by_id, 
    get_num_users,
    add_event,
};

use leptos::prelude::ServerFnError;
use leptos::*;
use crate::models::Event;

#[server(GetNumUsers, "/api")]
pub async fn get_num_users_api() -> Result<i64, ServerFnError> {
    let num_users: i64 = get_num_users(&establish_pool());
    Ok(num_users)
}

#[server(GetEventById,"/api")]
pub async fn get_event_by_id_api(event_id: i32) -> Result<Option<Event>, ServerFnError> {
    let event = get_event_by_id(&establish_pool(), event_id);
    Ok(event)
}


// Add Event to database api
#[server(AddEvent, "/api")]
pub async fn add_event_api(name: String, event_date: String, location: Option<String>, image_base64: Option<String>) -> Result<(), ServerFnError> {
    let event_date = chrono::NaiveDateTime::parse_from_str(&event_date, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| ServerFnError::new(format!("Invalid date format: {}", e)))?;
    
    add_event(&establish_pool(), name, event_date, location, image_base64);
    Ok(())
}
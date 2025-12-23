// #[cfg(not(target_arch = "wasm32"))]
// use crate::app::backend::database::{add_event, establish_pool, get_event_by_id, get_num_users};

use crate::models::Event;
use leptos::prelude::ServerFnError;
use leptos::*;

#[server(GetNumUsers, "/api")]
pub async fn get_num_users_api() -> Result<i64, ServerFnError> {
    use crate::app::backend::database::{establish_pool, get_num_users};
    let num_users: i64 = get_num_users(&establish_pool());
    Ok(num_users)
}

#[server(GetEventById, "/api")]
pub async fn get_event_by_id_api(event_id: i32) -> Result<Option<Event>, ServerFnError> {
    use crate::app::backend::database::{establish_pool, get_event_by_id};
    let event = get_event_by_id(&establish_pool(), event_id);
    Ok(event)
}

// Add Event to database api
#[server]
pub async fn add_event_api(
    name: String,
    event_date: String,
    location: Option<String>,
    image_base64: Option<String>,
) -> Result<(), ServerFnError> {
    use crate::app::backend::database::{add_event, establish_pool};
    // let event_date = chrono::NaiveDateTime::parse_from_str(&event_date, "%Y-%m-%d %H:%M:%S")
        // .map_err(|e| ServerFnError::new(format!("Invalid date format: {}", e)))?;

    // parse html datetime-local timestamp (2025-12-21T19:46)
    logging::log!("Parsing event date: {}", event_date);
    // Accept both 'T' and space separators, and optional seconds
    let naive_event_date = ["%Y-%m-%dT%H:%M:%S", "%Y-%m-%dT%H:%M", "%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M"]
        .iter()
        .find_map(|fmt| chrono::NaiveDateTime::parse_from_str(&event_date, fmt).ok())
        .ok_or_else(|| {
            logging::error!("Invalid date format: {}", event_date);
            ServerFnError::new(format!("Invalid date format: {}", event_date))
        })?;

    logging::log!("Parsed naive_event_date: {}", naive_event_date);

    logging::log!("add_event_api called with: {}, {}, {:?}, {:?}", name, event_date, location, image_base64);
    
    add_event(&establish_pool(), name, naive_event_date, location, image_base64)?;
    Ok(())
}

// use leptos::{server, ServerFnError};
// use std::env;

// #[server(GetProfile, "/api")]
// pub async fn get_profile_api() -> Result<Profile, ServerFnError> {
//     let data = retrieve_profile_api().await;
//     match data {
//         Ok(Some(profile)) => Ok(profile),
//         Ok(None) => Err(ServerFnError::ServerError("No profile found".to_string())),
//         Err(e) => Err(ServerFnError::from(e)),
//     }
// }

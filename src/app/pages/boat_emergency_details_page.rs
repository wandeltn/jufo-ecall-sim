use leptos::prelude::*;
use leptos::{view, IntoView};
use crate::app::pages::boat_emergency_page::BoatEmergencyData;
use crate::app::components::map::LocationMap;

#[component]
pub fn BoatEmergencyDetailsPage() -> impl IntoView {
    let data = RwSignal::new(BoatEmergencyData {
        captain_name: String::new(),
        phone: String::new(),
        boat_name: String::new(),
        vessel_type: String::new(),
        crew_size: String::new(),
        emergency_type: String::new(),
        description: String::new(),
        water_condition: String::new(),
        latitude: None,
        longitude: None,
    });

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json_str)) = storage.get_item("boat_emergency_data") {
                    if let Ok(loaded_data) = serde_json::from_str::<BoatEmergencyData>(&json_str) {
                        data.set(loaded_data);
                    }
                }
            }
        }
    }

    view! {
        <div class="min-h-screen bg-gray-50 py-12 px-4">
            <div class="max-w-2xl mx-auto bg-white rounded-lg shadow-lg p-8">
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-blue-600 mb-2">"Maritime Rescue Dispatch"</h1>
                    <p class="text-gray-600">"Distress signal received and coordinated with coast guard"</p>
                </div>

                <div class="space-y-6">
                    <div class="border-l-4 border-blue-600 pl-4">
                        <p class="text-sm text-gray-600">"Captain/Contact Name"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().captain_name}</p>
                    </div>

                    <div class="border-l-4 border-blue-600 pl-4">
                        <p class="text-sm text-gray-600">"Phone Number"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().phone}</p>
                    </div>

                    <div class="border-l-4 border-blue-600 pl-4">
                        <p class="text-sm text-gray-600">"Vessel Name"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().boat_name}</p>
                    </div>

                    <div class="border-l-4 border-blue-600 pl-4">
                        <p class="text-sm text-gray-600">"Vessel Type"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().vessel_type}</p>
                    </div>

                    <div class="border-l-4 border-blue-600 pl-4">
                        <p class="text-sm text-gray-600">"Crew Size"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().crew_size}</p>
                    </div>

                    <div class="border-l-4 border-orange-600 pl-4 bg-orange-50 p-4 rounded">
                        <p class="text-sm text-gray-600">"Water Condition"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().water_condition}</p>
                    </div>

                    <div class="border-l-4 border-blue-600 pl-4">
                        <p class="text-sm text-gray-600">"Emergency Type"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().emergency_type}</p>
                    </div>

                    <div class="border-l-4 border-blue-600 pl-4">
                        <p class="text-sm text-gray-600">"Description"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().description}</p>
                    </div>

                    <Show when=move || data.get().latitude.is_some() && data.get().longitude.is_some()>
                        <div class="border-l-4 border-green-600 pl-4 bg-green-50 p-4 rounded">
                            <p class="text-sm text-gray-600">"GPS Coordinates"</p>
                            <p class="text-lg font-semibold text-gray-800">
                                {move || {
                                    let d = data.get();
                                    format!(
                                        "Latitude: {:.6}, Longitude: {:.6}",
                                        d.latitude.unwrap_or(0.0),
                                        d.longitude.unwrap_or(0.0)
                                    )
                                }}
                            </p>
                        </div>
                        {move || {
                            let d = data.get();
                            view! {
                                <LocationMap
                                    latitude=d.latitude.unwrap_or(0.0)
                                    longitude=d.longitude.unwrap_or(0.0)
                                    location_name="Distress Location".to_string()
                                />
                            }
                        }}
                    </Show>
                </div>

                <div class="mt-8 pt-8 border-t border-gray-200">
                    <a
                        href="/boat-emergency"
                        class="inline-block bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded transition duration-200"
                    >
                        "New Distress Signal"
                    </a>
                </div>
            </div>
        </div>
    }
}

use leptos::prelude::*;
use leptos::{view, IntoView};
use crate::app::pages::wheelchair_emergency_page::WheelchairEmergencyData;
use crate::app::components::map::LocationMap;

#[component]
pub fn WheelchairEmergencyDetailsPage() -> impl IntoView {
    let data = RwSignal::new(WheelchairEmergencyData {
        user_name: String::new(),
        phone: String::new(),
        wheelchair_model: String::new(),
        emergency_type: String::new(),
        description: String::new(),
        medical_info: String::new(),
        location: String::new(),
        latitude: None,
        longitude: None,
    });

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json_str)) = storage.get_item("wheelchair_emergency_data") {
                    if let Ok(loaded_data) = serde_json::from_str::<WheelchairEmergencyData>(&json_str) {
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
                    <h1 class="text-3xl font-bold text-purple-600 mb-2">"Wheelchair Assistance Dispatch"</h1>
                    <p class="text-gray-600">"Request received and assistance being coordinated"</p>
                </div>

                <div class="space-y-6">
                    <div class="border-l-4 border-purple-600 pl-4">
                        <p class="text-sm text-gray-600">"User Name"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().user_name}</p>
                    </div>

                    <div class="border-l-4 border-purple-600 pl-4">
                        <p class="text-sm text-gray-600">"Phone Number"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().phone}</p>
                    </div>

                    <div class="border-l-4 border-purple-600 pl-4">
                        <p class="text-sm text-gray-600">"Location"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().location}</p>
                    </div>

                    <div class="border-l-4 border-purple-600 pl-4">
                        <p class="text-sm text-gray-600">"Wheelchair Type"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().wheelchair_model}</p>
                    </div>

                    <div class="border-l-4 border-orange-600 pl-4 bg-orange-50 p-4 rounded">
                        <p class="text-sm text-gray-600">"Issue Type"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().emergency_type}</p>
                    </div>

                    <div class="border-l-4 border-purple-600 pl-4">
                        <p class="text-sm text-gray-600">"Description"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().description}</p>
                    </div>

                    <Show when=move || !data.get().medical_info.is_empty()>
                        <div class="border-l-4 border-red-600 pl-4 bg-red-50 p-4 rounded">
                            <p class="text-sm text-gray-600">"Medical Information"</p>
                            <p class="text-lg font-semibold text-gray-800">{move || data.get().medical_info}</p>
                        </div>
                    </Show>

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
                                    location_name=d.location
                                />
                            }
                        }}
                    </Show>
                </div>

                <div class="mt-8 pt-8 border-t border-gray-200">
                    <a
                        href="/wheelchair-emergency"
                        class="inline-block bg-purple-600 hover:bg-purple-700 text-white font-bold py-2 px-6 rounded transition duration-200"
                    >
                        "New Request"
                    </a>
                </div>
            </div>
        </div>
    }
}

use leptos::prelude::*;
use leptos::{view, IntoView};
use crate::app::pages::lift_emergency_page::LiftEmergencyData;
use crate::app::components::map::LocationMap;

#[component]
pub fn LiftEmergencyDetailsPage() -> impl IntoView {
    let data = RwSignal::new(LiftEmergencyData {
        user_name: String::new(),
        phone: String::new(),
        building_name: String::new(),
        current_floor: String::new(),
        people_trapped: String::new(),
        emergency_type: String::new(),
        description: String::new(),
        medical_info: String::new(),
        latitude: None,
        longitude: None,
    });

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(json_str)) = storage.get_item("lift_emergency_data") {
                    if let Ok(loaded_data) = serde_json::from_str::<LiftEmergencyData>(&json_str) {
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
                    <h1 class="text-3xl font-bold text-yellow-600 mb-2">"Lift Emergency Dispatch"</h1>
                    <p class="text-gray-600">"Rescue team notified and en route"</p>
                </div>

                <div class="space-y-6">
                    <div class="border-l-4 border-yellow-600 pl-4">
                        <p class="text-sm text-gray-600">"User Name"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().user_name}</p>
                    </div>

                    <div class="border-l-4 border-yellow-600 pl-4">
                        <p class="text-sm text-gray-600">"Phone Number"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().phone}</p>
                    </div>

                    <div class="border-l-4 border-yellow-600 pl-4">
                        <p class="text-sm text-gray-600">"Building Name/Address"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().building_name}</p>
                    </div>

                    <div class="border-l-4 border-yellow-600 pl-4">
                        <p class="text-sm text-gray-600">"Current Floor"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().current_floor}</p>
                    </div>

                    <div class="border-l-4 border-yellow-600 pl-4">
                        <p class="text-sm text-gray-600">"Number of People Trapped"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().people_trapped}</p>
                    </div>

                    <div class="border-l-4 border-red-600 pl-4 bg-red-50 p-4 rounded">
                        <p class="text-sm text-gray-600">"Emergency Type"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().emergency_type}</p>
                    </div>

                    <div class="border-l-4 border-yellow-600 pl-4">
                        <p class="text-sm text-gray-600">"Description"</p>
                        <p class="text-lg font-semibold text-gray-800">{move || data.get().description}</p>
                    </div>

                    <Show when=move || !data.get().medical_info.is_empty()>
                        <div class="border-l-4 border-orange-600 pl-4 bg-orange-50 p-4 rounded">
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
                                    location_name=d.building_name
                                />
                            }
                        }}
                    </Show>
                </div>

                <div class="mt-8 pt-8 border-t border-gray-200">
                    <a
                        href="/lift-emergency"
                        class="inline-block bg-yellow-600 hover:bg-yellow-700 text-white font-bold py-2 px-6 rounded transition duration-200"
                    >
                        "New Emergency"
                    </a>
                </div>
            </div>
        </div>
    }
}

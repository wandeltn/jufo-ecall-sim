use leptos::prelude::*;
use leptos::{view, IntoView};
use crate::app::pages::home_page::EcallData;
use crate::app::components::map::LocationMap;

#[component]
pub fn EcallDetailsPage() -> impl IntoView {
    let ecall_data = RwSignal::new(EcallData {
        current_latitude: None,
        current_longitude: None,
        latitude_10s: None,
        longitude_10s: None,
        latitude_20s: None,
        longitude_20s: None,
        driving_direction: String::new(),
        activation_type: String::new(),
        vehicle_phone: String::new(),
        vehicle_type: String::new(),
        propulsion_type: String::new(),
        vin: String::new(),
        restrained_occupants: String::new(),
    });

    // Load from localStorage on mount
    {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(json_str)) = storage.get_item("ecall_data") {
                        if let Ok(data) = serde_json::from_str::<EcallData>(&json_str) {
                            ecall_data.set(data);
                        }
                    }
                }
            }
        }
    }

    view! {
        <div class="min-h-screen bg-gray-50 py-12 px-4">
            <div class="max-w-2xl mx-auto bg-white rounded-lg shadow-lg p-8">
                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-red-600 mb-2">
                        "Emergency Dispatch Details"
                    </h1>
                    <p class="text-gray-600">"Call received and processed by emergency dispatch"</p>
                </div>

                <div class="space-y-6">
                    <div class="border-l-4 border-red-600 pl-4">
                        <p class="text-sm text-gray-600">"eCall Activation Type"</p>
                        <p class="text-lg font-semibold text-gray-800">
                            {move || ecall_data.get().activation_type}
                        </p>
                    </div>

                    <div class="border-l-4 border-red-600 pl-4">
                        <p class="text-sm text-gray-600">"Vehicle Phone Number"</p>
                        <p class="text-lg font-semibold text-gray-800">
                            {move || ecall_data.get().vehicle_phone}
                        </p>
                    </div>

                    <div class="border-l-4 border-red-600 pl-4">
                        <p class="text-sm text-gray-600">"Vehicle Type"</p>
                        <p class="text-lg font-semibold text-gray-800">
                            {move || ecall_data.get().vehicle_type}
                        </p>
                    </div>

                    <div class="border-l-4 border-red-600 pl-4">
                        <p class="text-sm text-gray-600">"Propulsion Type"</p>
                        <p class="text-lg font-semibold text-gray-800">
                            {move || ecall_data.get().propulsion_type}
                        </p>
                    </div>

                    <div class="border-l-4 border-red-600 pl-4">
                        <p class="text-sm text-gray-600">"VIN (Vehicle Identification Number)"</p>
                        <p class="text-lg font-semibold text-gray-800">
                            {move || ecall_data.get().vin}
                        </p>
                    </div>

                    <div class="border-l-4 border-red-600 pl-4">
                        <p class="text-sm text-gray-600">"Number of Restrained Occupants"</p>
                        <p class="text-lg font-semibold text-gray-800">
                            {move || ecall_data.get().restrained_occupants}
                        </p>
                    </div>

                    <div class="border-l-4 border-red-600 pl-4">
                        <p class="text-sm text-gray-600">"Driving Direction"</p>
                        <p class="text-lg font-semibold text-gray-800">
                            {move || ecall_data.get().driving_direction}
                        </p>
                    </div>

                    <Show when=move || {
                        ecall_data.get().current_latitude.is_some() && ecall_data.get().current_longitude.is_some()
                    }>
                        <div class="border-l-4 border-green-600 pl-4 bg-green-50 p-4 rounded">
                            <p class="text-sm text-gray-600">"Current GPS Coordinates"</p>
                            <p class="text-lg font-semibold text-gray-800">
                                {move || {
                                    let data = ecall_data.get();
                                    format!(
                                        "Latitude: {:.6}, Longitude: {:.6}",
                                        data.current_latitude.unwrap_or(0.0),
                                        data.current_longitude.unwrap_or(0.0),
                                    )
                                }}
                            </p>
                        </div>
                        {move || {
                            let data = ecall_data.get();
                            view! {
                                <LocationMap
                                    latitude=data.current_latitude.unwrap_or(0.0)
                                    longitude=data.current_longitude.unwrap_or(0.0)
                                    location_name="Emergency Location".to_string()
                                />
                            }
                        }}
                    </Show>

                    <Show when=move || {
                        ecall_data.get().latitude_10s.is_some() && ecall_data.get().longitude_10s.is_some()
                    }>
                        <div class="border-l-4 border-blue-600 pl-4 bg-blue-50 p-4 rounded">
                            <p class="text-sm text-gray-600">"GPS Coordinates (10 seconds ago)"</p>
                            <p class="text-lg font-semibold text-gray-800">
                                {move || {
                                    let data = ecall_data.get();
                                    format!(
                                        "Latitude: {:.6}, Longitude: {:.6}",
                                        data.latitude_10s.unwrap_or(0.0),
                                        data.longitude_10s.unwrap_or(0.0),
                                    )
                                }}
                            </p>
                        </div>
                    </Show>

                    <Show when=move || {
                        ecall_data.get().latitude_20s.is_some() && ecall_data.get().longitude_20s.is_some()
                    }>
                        <div class="border-l-4 border-purple-600 pl-4 bg-purple-50 p-4 rounded">
                            <p class="text-sm text-gray-600">"GPS Coordinates (20 seconds ago)"</p>
                            <p class="text-lg font-semibold text-gray-800">
                                {move || {
                                    let data = ecall_data.get();
                                    format!(
                                        "Latitude: {:.6}, Longitude: {:.6}",
                                        data.latitude_20s.unwrap_or(0.0),
                                        data.longitude_20s.unwrap_or(0.0),
                                    )
                                }}
                            </p>
                        </div>
                    </Show>
                </div>

                <div class="mt-8 pt-8 border-t border-gray-200">
                    <a
                        href="/"
                        class="inline-block bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-6 rounded transition duration-200"
                    >
                        "Create New Call"
                    </a>
                </div>
            </div>
        </div>
    }
}

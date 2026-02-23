use crate::app::pages::i18n::{Language, Translations};
use crate::app::pages::presets::{get_ecall_presets, EcallEmergencyPreset};
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_router::hooks::use_navigate;
use leptos_use::use_geolocation;
use serde::{Deserialize, Serialize};

// Define the ecall data structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcallData {
    pub current_latitude: Option<f64>,
    pub current_longitude: Option<f64>,
    pub latitude_10s: Option<f64>,
    pub longitude_10s: Option<f64>,
    pub latitude_20s: Option<f64>,
    pub longitude_20s: Option<f64>,
    pub driving_direction: String,
    pub activation_type: String,
    pub vehicle_phone: String,
    pub vehicle_type: String,
    pub propulsion_type: String,
    pub vin: String,
    pub restrained_occupants: String,
}

// Helper function to estimate previous position based on heading and distance
fn estimate_previous_position(
    current_lat: f64,
    current_lon: f64,
    heading: f64,
    seconds_ago: f64,
    estimated_speed: f64,
) -> (f64, f64) {
    // Estimate distance traveled (in degrees, rough approximation)
    // At equator, 1 degree ≈ 111 km
    // Assume speed in km/h, convert to degrees per second
    let distance_degrees = (estimated_speed / 111.0) * (seconds_ago / 3600.0);

    // Convert heading to radians (0=North, 90=East, 180=South, 270=West)
    let heading_rad = (heading * std::f64::consts::PI) / 180.0;

    // Calculate previous position by moving backward along the heading
    let prev_lat = current_lat - (distance_degrees * heading_rad.cos());
    let prev_lon = current_lon - (distance_degrees * heading_rad.sin());

    (prev_lat, prev_lon)
}

#[component]
pub fn EcallSimulatorPage(language: RwSignal<Language>) -> impl IntoView {
    let navigate = use_navigate();
    let geolocation = use_geolocation();

    let (presets, _set_presets) = signal(get_ecall_presets());

    let use_preset_mode = RwSignal::new(true);

    // Auto-populated from geolocation
    let current_latitude = RwSignal::new(String::new());
    let current_longitude = RwSignal::new(String::new());
    let latitude_10s = RwSignal::new(String::new());
    let longitude_10s = RwSignal::new(String::new());
    let latitude_20s = RwSignal::new(String::new());
    let longitude_20s = RwSignal::new(String::new());
    let heading_value = RwSignal::new(0.0);

    let driving_direction = RwSignal::new(String::from("North"));
    let activation_type = RwSignal::new(String::from("Accident"));
    let vehicle_phone = RwSignal::new(String::new());
    let vehicle_type = RwSignal::new(String::from("PKW"));
    let propulsion_type = RwSignal::new(String::from("Diesel"));
    let vin = RwSignal::new(String::new());
    let restrained_occupants = RwSignal::new(String::from("1"));

    // Geolocation status
    let location_acquired = RwSignal::new(false);
    let location_error = RwSignal::new(String::new());

    // Initialize geolocation data on component load
    {
        let coords = geolocation.coords;
        Memo::new(move |_| {
            if let Some(coord) = coords.get() {
                let lat = coord.latitude();
                let lon = coord.longitude();
                let heading = coord.heading().unwrap_or(0.0);

                current_latitude.set(lat.to_string());
                current_longitude.set(lon.to_string());
                heading_value.set(heading);

                // Update driving direction based on heading
                let direction = match heading {
                    h if h >= 337.5 || h < 22.5 => "North",
                    h if h >= 22.5 && h < 67.5 => "Northeast",
                    h if h >= 67.5 && h < 112.5 => "East",
                    h if h >= 112.5 && h < 157.5 => "Southeast",
                    h if h >= 157.5 && h < 202.5 => "South",
                    h if h >= 202.5 && h < 247.5 => "Southwest",
                    h if h >= 247.5 && h < 292.5 => "West",
                    _ => "Northwest",
                };
                driving_direction.set(direction.to_string());

                // Estimate previous positions (assuming 80 km/h average speed)
                let estimated_speed = 80.0;
                let (prev_lat_10s, prev_lon_10s) =
                    estimate_previous_position(lat, lon, heading, 10.0, estimated_speed);
                let (prev_lat_20s, prev_lon_20s) =
                    estimate_previous_position(lat, lon, heading, 20.0, estimated_speed);

                latitude_10s.set(prev_lat_10s.to_string());
                longitude_10s.set(prev_lon_10s.to_string());
                latitude_20s.set(prev_lat_20s.to_string());
                longitude_20s.set(prev_lon_20s.to_string());

                location_acquired.set(true);
                location_error.set(String::new());
            } else {
                location_error.set("Waiting for GPS signal...".to_string());
            }
        });
    }

    let apply_preset = move |preset: EcallEmergencyPreset| {
        activation_type.set(preset.activation_type);
        vehicle_type.set(preset.vehicle_type);
        propulsion_type.set(preset.propulsion_type);
        restrained_occupants.set(preset.restrained_occupants);
        driving_direction.set(preset.driving_direction);
        vin.set(preset.vin);
        vehicle_phone.set(preset.vehicle_phone);
    };

    let t = move |key: &str| Translations::new(language.get()).t(key);

    view! {
        <div class="min-h-screen bg-gradient-to-b from-red-50 to-white py-12 px-4">
            <div class="max-w-2xl mx-auto">
                <div class="mb-12">
                    <h1 class="text-4xl font-bold text-gray-900 mb-2">
                        {move || t("ecall.title")}
                    </h1>
                    <p class="text-lg text-gray-600">{move || t("ecall.subtitle")}</p>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-8">

                    {move || match use_preset_mode.get() {
                        true if use_preset_mode.get() == true => {
                            // returns HtmlElement<Pre>
                            view! {
                                <div>
                                    <div class="bg-red-50 border-l-4 border-red-500 p-4 mb-6">
                                        <h2 class="text-2xl font-bold text-gray-800 mb-2">
                                            {move || t("simulator.quick_selection")}
                                        </h2>
                                        <p class="text-gray-700">
                                            {move || t("simulator.quick_selection_desc")}
                                        </p>
                                    </div>

                                    <div class="grid grid-cols-1 gap-3 mb-8">
                                        <For
                                            each=move || presets.get()
                                            key=|preset| preset.name.clone()
                                            children=move |preset: EcallEmergencyPreset| {
                                                let preset_clone = preset.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        on:click=move |_| {
                                                            apply_preset(preset_clone.clone());
                                                            use_preset_mode.set(false);
                                                        }
                                                        class="p-4 text-left bg-white border-2 border-red-200 rounded-lg hover:bg-red-50 hover:border-red-500 active:border-red-700 transition duration-200 cursor-pointer shadow-sm hover:shadow-md"
                                                    >
                                                        <h3 class="font-bold text-lg text-red-700 mb-1">
                                                            {preset.name}
                                                        </h3>
                                                        <p class="text-sm text-gray-600">{preset.description}</p>
                                                    </button>
                                                }
                                            }
                                        />
                                    </div>

                                    <div class="border-t-2 border-gray-200 pt-6">
                                        <p class="text-sm text-gray-500 mb-3">
                                            {move || t("simulator.need_control")}
                                        </p>
                                        <button
                                            type="button"
                                            on:click=move |_| use_preset_mode.set(false)
                                            class="w-full bg-white hover:bg-gray-100 text-gray-700 border-2 border-gray-300 font-semibold py-3 px-4 rounded-lg transition duration-200"
                                        >
                                            {move || t("simulator.manual_entry")}
                                        </button>
                                    </div>
                                </div>
                            }
                                .into_any()
                        }
                        false if use_preset_mode.get() == false => {
                            let navigate_clone = navigate.clone();
                            // returns HtmlElement<P>
                            view! {
                                <div class="mb-6">
                                    <button
                                        type="button"
                                        on:click=move |_| use_preset_mode.set(true)
                                        class="w-full bg-gray-500 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded-lg transition duration-200 mb-6"
                                    >
                                        {move || t("simulator.back_presets")}
                                    </button>
                                </div>

                                <form
                                    class="space-y-6"
                                    on:submit=move |e| {
                                        e.prevent_default();
                                        let lat = current_latitude.get().parse::<f64>().ok();
                                        let lon = current_longitude.get().parse::<f64>().ok();
                                        let lat_10s = latitude_10s.get().parse::<f64>().ok();
                                        let lon_10s = longitude_10s.get().parse::<f64>().ok();
                                        let lat_20s = latitude_20s.get().parse::<f64>().ok();
                                        let lon_20s = longitude_20s.get().parse::<f64>().ok();
                                        let _ecall_data = EcallData {
                                            current_latitude: lat,
                                            current_longitude: lon,
                                            latitude_10s: lat_10s,
                                            longitude_10s: lon_10s,
                                            latitude_20s: lat_20s,
                                            longitude_20s: lon_20s,
                                            driving_direction: driving_direction.get(),
                                            activation_type: activation_type.get(),
                                            vehicle_phone: vehicle_phone.get(),
                                            vehicle_type: vehicle_type.get(),
                                            propulsion_type: propulsion_type.get(),
                                            vin: vin.get(),
                                            restrained_occupants: restrained_occupants.get(),
                                        };
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(window) = web_sys::window() {
                                                if let Ok(Some(storage)) = window.local_storage() {
                                                    if let Ok(json_str) = serde_json::to_string(&_ecall_data) {
                                                        let _ = storage.set_item("ecall_data", &json_str);
                                                    }
                                                }
                                            }
                                        }
                                        navigate_clone("/ecall-details", Default::default());
                                    }
                                >
                                    <div class="mb-6">
                                        <div class="bg-blue-50 border-l-4 border-blue-500 p-4 rounded">
                                            <p class="text-sm font-medium text-gray-700 mb-2">
                                                "GPS Status"
                                            </p>
                                            <Show
                                                when=move || location_acquired.get()
                                                fallback=move || {
                                                    view! {
                                                        <p class="text-sm text-orange-700">
                                                            {move || {
                                                                let msg = location_error.get();
                                                                if msg.is_empty() {
                                                                    "🔍 Acquiring GPS signal...".to_string()
                                                                } else {
                                                                    msg
                                                                }
                                                            }}
                                                        </p>
                                                    }
                                                }
                                            >
                                                <div>
                                                    <p class="text-sm text-green-700 font-semibold mb-2">
                                                        "✓ GPS Signal Acquired"
                                                    </p>
                                                    <div class="grid grid-cols-2 gap-2 text-xs text-gray-600">
                                                        <p>
                                                            "Current: "
                                                            {move || {
                                                                format!(
                                                                    "{:.4}",
                                                                    current_latitude.get().parse::<f64>().unwrap_or(0.0),
                                                                )
                                                            }} "/"
                                                            {move || {
                                                                format!(
                                                                    "{:.4}",
                                                                    current_longitude.get().parse::<f64>().unwrap_or(0.0),
                                                                )
                                                            }}
                                                        </p>
                                                        <p>
                                                            "Heading: "
                                                            {move || format!("{}°", heading_value.get().round())}
                                                        </p>
                                                    </div>
                                                </div>
                                            </Show>
                                        </div>
                                    </div>

                                    <div>
                                        <label
                                            for="activation_type"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "eCall Activation Type"
                                        </label>
                                        <select
                                            id="activation_type"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                activation_type.set(event_target_value(&e));
                                            }
                                            prop:value=activation_type
                                        >
                                            <option value="Accident">"Accident"</option>
                                            <option value="SOS Button">"SOS Button"</option>
                                        </select>
                                    </div>

                                    <div>
                                        <label
                                            for="vehicle_phone"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Vehicle Phone Number"
                                        </label>
                                        <input
                                            id="vehicle_phone"
                                            type="tel"
                                            required
                                            placeholder="e.g., +49-123-456789"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| {
                                                vehicle_phone.set(event_target_value(&e));
                                            }
                                            prop:value=vehicle_phone
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="vehicle_type"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Vehicle Type"
                                        </label>
                                        <select
                                            id="vehicle_type"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                vehicle_type.set(event_target_value(&e));
                                            }
                                            prop:value=vehicle_type
                                        >
                                            <option value="PKW">"PKW (Car)"</option>
                                            <option value="LKW">"LKW (Truck)"</option>
                                            <option value="Krad">"Krad (Motorcycle)"</option>
                                            <option value="Bus">"Bus"</option>
                                            <option value="Other">"Other"</option>
                                        </select>
                                    </div>

                                    <div>
                                        <label
                                            for="propulsion_type"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Propulsion Type"
                                        </label>
                                        <select
                                            id="propulsion_type"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                propulsion_type.set(event_target_value(&e));
                                            }
                                            prop:value=propulsion_type
                                        >
                                            <option value="Benzin">"Benzin (Gasoline)"</option>
                                            <option value="Diesel">"Diesel"</option>
                                            <option value="LPG">"LPG"</option>
                                            <option value="Elektro">"Elektro (Electric)"</option>
                                            <option value="Hybrid">"Hybrid"</option>
                                        </select>
                                    </div>

                                    <div>
                                        <label
                                            for="vin"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Vehicle Identification Number (VIN)"
                                        </label>
                                        <input
                                            id="vin"
                                            type="text"
                                            required
                                            placeholder="e.g., WBADT43452G216401"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| {
                                                vin.set(event_target_value(&e));
                                            }
                                            prop:value=vin
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="restrained_occupants"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Number of Restrained Occupants"
                                        </label>
                                        <input
                                            id="restrained_occupants"
                                            type="number"
                                            min="1"
                                            required
                                            placeholder="1"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| {
                                                restrained_occupants.set(event_target_value(&e));
                                            }
                                            prop:value=restrained_occupants
                                        />
                                    </div>

                                    <div class="pt-4">
                                        <button
                                            type="submit"
                                            class="w-full bg-red-600 hover:bg-red-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200 text-lg"
                                        >
                                            "Simulate Emergency Call"
                                        </button>
                                    </div>
                                </form>
                            }
                                .into_any()
                        }
                        _ => {
                            // returns HtmlElement<Textarea>
                            view! { <textarea>{use_preset_mode.get()}</textarea> }
                                .into_any()
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

use crate::app::pages::i18n::{Language, Translations};
use crate::app::pages::presets::{get_lift_presets, LiftEmergencyPreset};
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_router::hooks::use_navigate;
use leptos_use::use_geolocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LiftEmergencyData {
    pub user_name: String,
    pub phone: String,
    pub building_name: String,
    pub current_floor: String,
    pub people_trapped: String,
    pub emergency_type: String,
    pub description: String,
    pub medical_info: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[component]
pub fn LiftEmergencyPage(language: RwSignal<Language>) -> impl IntoView {
    let navigate = use_navigate();
    let geolocation = use_geolocation();
    let (presets, _set_presets) = signal(get_lift_presets());

    let use_preset_mode = RwSignal::new(true);

    let user_name = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());
    let building_name = RwSignal::new(String::new());
    let current_floor = RwSignal::new(String::from("1"));
    let people_trapped = RwSignal::new(String::from("1"));
    let emergency_type = RwSignal::new(String::from("Stuck"));
    let description = RwSignal::new(String::new());
    let medical_info = RwSignal::new(String::new());

    // Geolocation status
    let location_acquired = RwSignal::new(false);
    let location_error = RwSignal::new(String::new());

    // Initialize geolocation data on component load
    {
        let coords = geolocation.coords;
        Effect::new(move || {
            if let Some(_coord) = coords.get() {
                location_acquired.set(true);
                location_error.set(String::new());
            } else {
                location_error.set("🔍 Acquiring GPS signal...".to_string());
            }
        });
    }

    let apply_preset = move |preset: LiftEmergencyPreset| {
        user_name.set(preset.user_name);
        phone.set(preset.phone);
        building_name.set(preset.building_name);
        current_floor.set(preset.current_floor);
        people_trapped.set(preset.people_trapped);
        emergency_type.set(preset.emergency_type);
        description.set(preset.description);
        medical_info.set(preset.medical_info);
    };

    let t = move |key: &str| Translations::new(language.get()).t(key);

    view! {
        <div class="min-h-screen bg-gradient-to-b from-yellow-50 to-white py-12 px-4">
            <div class="max-w-2xl mx-auto">
                <div class="mb-12">
                    <h1 class="text-4xl font-bold text-gray-900 mb-2">{move || t("lift.title")}</h1>
                    <p class="text-lg text-gray-600">{move || t("lift.subtitle")}</p>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-8">

                    {move || match use_preset_mode.get() {
                        true if use_preset_mode.get() == true => {
                            // returns HtmlElement<Pre>
                            view! {
                                <div>
                                    <div class="bg-yellow-50 border-l-4 border-yellow-500 p-4 mb-6">
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
                                            children=move |preset: LiftEmergencyPreset| {
                                                let preset_clone = preset.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        on:click=move |_| {
                                                            apply_preset(preset_clone.clone());
                                                            use_preset_mode.set(false);
                                                        }
                                                        class="p-4 text-left bg-white border-2 border-yellow-200 rounded-lg hover:bg-yellow-50 hover:border-yellow-500 active:border-yellow-700 transition duration-200 cursor-pointer shadow-sm hover:shadow-md"
                                                    >
                                                        <h3 class="font-bold text-lg text-yellow-700 mb-1">
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
                                        let (lat, lon) = geolocation
                                            .coords
                                            .get()
                                            .map(|c| (Some(c.latitude()), Some(c.longitude())))
                                            .unwrap_or((None, None));
                                        let _data = LiftEmergencyData {
                                            user_name: user_name.get(),
                                            phone: phone.get(),
                                            building_name: building_name.get(),
                                            current_floor: current_floor.get(),
                                            people_trapped: people_trapped.get(),
                                            emergency_type: emergency_type.get(),
                                            description: description.get(),
                                            medical_info: medical_info.get(),
                                            latitude: lat,
                                            longitude: lon,
                                        };
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(window) = web_sys::window() {
                                                if let Ok(Some(storage)) = window.local_storage() {
                                                    if let Ok(json_str) = serde_json::to_string(&_data) {
                                                        let _ = storage.set_item("lift_emergency_data", &json_str);
                                                    }
                                                }
                                            }
                                        }
                                        navigate_clone(
                                            "/lift-emergency-details",
                                            Default::default(),
                                        );
                                    }
                                >
                                    <div class="mb-6">
                                        <div class="bg-yellow-50 border-l-4 border-yellow-500 p-4 rounded">
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
                                                <p class="text-sm text-green-700 font-semibold">
                                                    "✓ GPS Signal Acquired"
                                                </p>
                                            </Show>
                                        </div>
                                    </div>

                                    <div>
                                        <label
                                            for="user_name"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Your Name"
                                        </label>
                                        <input
                                            id="user_name"
                                            type="text"
                                            required
                                            placeholder="Enter your name"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| user_name.set(event_target_value(&e))
                                            prop:value=user_name
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="phone"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Phone Number"
                                        </label>
                                        <input
                                            id="phone"
                                            type="tel"
                                            required
                                            placeholder="e.g., 123-456-7890"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| phone.set(event_target_value(&e))
                                            prop:value=phone
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="building_name"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Building Name/Address"
                                        </label>
                                        <input
                                            id="building_name"
                                            type="text"
                                            required
                                            placeholder="Enter building name or address"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| building_name.set(event_target_value(&e))
                                            prop:value=building_name
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="current_floor"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Current Floor"
                                        </label>
                                        <input
                                            id="current_floor"
                                            type="text"
                                            required
                                            placeholder="e.g., 3, B2, Roof"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| current_floor.set(event_target_value(&e))
                                            prop:value=current_floor
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="people_trapped"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Number of People Trapped"
                                        </label>
                                        <input
                                            id="people_trapped"
                                            type="number"
                                            min="1"
                                            required
                                            placeholder="1"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| people_trapped.set(event_target_value(&e))
                                            prop:value=people_trapped
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="emergency_type"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Emergency Type"
                                        </label>
                                        <select
                                            id="emergency_type"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                emergency_type.set(event_target_value(&e))
                                            }
                                            prop:value=emergency_type
                                        >
                                            <option value="Stuck">"Stuck Between Floors"</option>
                                            <option value="Door Stuck">"Door Stuck"</option>
                                            <option value="Emergency Stop">
                                                "Emergency Stop Activated"
                                            </option>
                                            <option value="Mechanical Failure">
                                                "Mechanical Failure"
                                            </option>
                                            <option value="Medical Emergency">
                                                "Medical Emergency"
                                            </option>
                                            <option value="Other">"Other"</option>
                                        </select>
                                    </div>

                                    <div>
                                        <label
                                            for="description"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Detailed Description"
                                        </label>
                                        <textarea
                                            id="description"
                                            required
                                            placeholder="Describe the emergency situation"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition h-32 resize-none"
                                            on:input=move |e| description.set(event_target_value(&e))
                                            prop:value=description
                                        ></textarea>
                                    </div>

                                    <div>
                                        <label
                                            for="medical_info"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Medical Information (Optional)"
                                        </label>
                                        <textarea
                                            id="medical_info"
                                            placeholder="Any medical conditions that need attention"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-yellow-500 focus:border-transparent outline-none transition h-24 resize-none"
                                            on:input=move |e| medical_info.set(event_target_value(&e))
                                            prop:value=medical_info
                                        ></textarea>
                                    </div>

                                    <div class="pt-4">
                                        <button
                                            type="submit"
                                            class="w-full bg-yellow-600 hover:bg-yellow-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200 text-lg"
                                        >
                                            "Call for Rescue"
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

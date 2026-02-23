use crate::app::pages::i18n::{Language, Translations};
use crate::app::pages::presets::{get_boat_presets, BoatEmergencyPreset};
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_router::hooks::use_navigate;
use leptos_use::use_geolocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoatEmergencyData {
    pub captain_name: String,
    pub phone: String,
    pub boat_name: String,
    pub vessel_type: String,
    pub crew_size: String,
    pub emergency_type: String,
    pub description: String,
    pub water_condition: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[component]
pub fn BoatEmergencyPage(language: RwSignal<Language>) -> impl IntoView {
    let navigate = use_navigate();
    let geolocation = use_geolocation();
    let (presets, _set_presets) = signal(get_boat_presets());

    let use_preset_mode = RwSignal::new(true);

    let captain_name = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());
    let boat_name = RwSignal::new(String::new());
    let vessel_type = RwSignal::new(String::from("Sailboat"));
    let crew_size = RwSignal::new(String::from("1"));
    let emergency_type = RwSignal::new(String::from("Medical"));
    let description = RwSignal::new(String::new());
    let water_condition = RwSignal::new(String::from("Sinking"));

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

    let apply_preset = move |preset: BoatEmergencyPreset| {
        captain_name.set(preset.captain_name);
        phone.set(preset.phone);
        boat_name.set(preset.boat_name);
        vessel_type.set(preset.vessel_type);
        crew_size.set(preset.crew_size);
        emergency_type.set(preset.emergency_type);
        water_condition.set(preset.water_condition);
        description.set(preset.description);
    };

    let t = move |key: &str| Translations::new(language.get()).t(key);

    view! {
        <div class="min-h-screen bg-gradient-to-b from-blue-50 to-white py-12 px-4">
            <div class="max-w-2xl mx-auto">
                <div class="mb-12">
                    <h1 class="text-4xl font-bold text-gray-900 mb-2">{move || t("boat.title")}</h1>
                    <p class="text-lg text-gray-600">{move || t("boat.subtitle")}</p>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-8">

                    {move || match use_preset_mode.get() {
                        true if use_preset_mode.get() == true => {
                            // returns HtmlElement<Pre>
                            view! {
                                <div>
                                    <div class="bg-blue-50 border-l-4 border-blue-500 p-4 mb-6">
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
                                            children=move |preset: BoatEmergencyPreset| {
                                                let preset_clone = preset.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        on:click=move |_| {
                                                            apply_preset(preset_clone.clone());
                                                            use_preset_mode.set(false);
                                                        }
                                                        class="p-4 text-left bg-white border-2 border-blue-200 rounded-lg hover:bg-blue-50 hover:border-blue-500 active:border-blue-700 transition duration-200 cursor-pointer shadow-sm hover:shadow-md"
                                                    >
                                                        <h3 class="font-bold text-lg text-blue-700 mb-1">
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
                                        let _data = BoatEmergencyData {
                                            captain_name: captain_name.get(),
                                            phone: phone.get(),
                                            boat_name: boat_name.get(),
                                            vessel_type: vessel_type.get(),
                                            crew_size: crew_size.get(),
                                            emergency_type: emergency_type.get(),
                                            description: description.get(),
                                            water_condition: water_condition.get(),
                                            latitude: lat,
                                            longitude: lon,
                                        };
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(window) = web_sys::window() {
                                                if let Ok(Some(storage)) = window.local_storage() {
                                                    if let Ok(json_str) = serde_json::to_string(&_data) {
                                                        let _ = storage.set_item("boat_emergency_data", &json_str);
                                                    }
                                                }
                                            }
                                        }
                                        navigate_clone(
                                            "/boat-emergency-details",
                                            Default::default(),
                                        );
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
                                                <p class="text-sm text-green-700 font-semibold">
                                                    "✓ GPS Signal Acquired"
                                                </p>
                                            </Show>
                                        </div>
                                    </div>

                                    <div>
                                        <label
                                            for="captain_name"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Captain/Contact Name"
                                        </label>
                                        <input
                                            id="captain_name"
                                            type="text"
                                            required
                                            placeholder="Enter name"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| captain_name.set(event_target_value(&e))
                                            prop:value=captain_name
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
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| phone.set(event_target_value(&e))
                                            prop:value=phone
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="boat_name"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Boat/Vessel Name"
                                        </label>
                                        <input
                                            id="boat_name"
                                            type="text"
                                            required
                                            placeholder="Enter vessel name"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| boat_name.set(event_target_value(&e))
                                            prop:value=boat_name
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="vessel_type"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Vessel Type"
                                        </label>
                                        <select
                                            id="vessel_type"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| vessel_type.set(event_target_value(&e))
                                            prop:value=vessel_type
                                        >
                                            <option value="Sailboat">"Sailboat"</option>
                                            <option value="Motorboat">"Motorboat"</option>
                                            <option value="Yacht">"Yacht"</option>
                                            <option value="Fishing Boat">"Fishing Boat"</option>
                                            <option value="Cargo Ship">"Cargo Ship"</option>
                                            <option value="Passenger Ship">"Passenger Ship"</option>
                                            <option value="Other">"Other"</option>
                                        </select>
                                    </div>

                                    <div>
                                        <label
                                            for="crew_size"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Crew Size"
                                        </label>
                                        <input
                                            id="crew_size"
                                            type="number"
                                            min="1"
                                            required
                                            placeholder="Number of people on board"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| crew_size.set(event_target_value(&e))
                                            prop:value=crew_size
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
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                emergency_type.set(event_target_value(&e))
                                            }
                                            prop:value=emergency_type
                                        >
                                            <option value="Medical">"Medical Emergency"</option>
                                            <option value="Engine Failure">"Engine Failure"</option>
                                            <option value="Fire">"Fire"</option>
                                            <option value="Collision">"Collision"</option>
                                            <option value="Other">"Other"</option>
                                        </select>
                                    </div>

                                    <div>
                                        <label
                                            for="water_condition"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Water Condition"
                                        </label>
                                        <select
                                            id="water_condition"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                water_condition.set(event_target_value(&e))
                                            }
                                            prop:value=water_condition
                                        >
                                            <option value="Sinking">"Sinking"</option>
                                            <option value="Taking Water">"Taking Water"</option>
                                            <option value="Disabled">"Disabled"</option>
                                            <option value="Stable">"Stable"</option>
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
                                            placeholder="Describe the emergency situation in detail"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition h-32 resize-none"
                                            on:input=move |e| description.set(event_target_value(&e))
                                            prop:value=description
                                        ></textarea>
                                    </div>

                                    <div class="pt-4">
                                        <button
                                            type="submit"
                                            class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200 text-lg"
                                        >
                                            "Send Distress Signal"
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

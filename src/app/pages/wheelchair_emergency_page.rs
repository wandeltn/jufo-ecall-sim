use crate::app::pages::i18n::{Language, Translations};
use crate::app::pages::presets::{get_wheelchair_presets, WheelchairEmergencyPreset};
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_router::hooks::use_navigate;
use leptos_use::use_geolocation;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WheelchairEmergencyData {
    pub user_name: String,
    pub phone: String,
    pub wheelchair_model: String,
    pub emergency_type: String,
    pub description: String,
    pub medical_info: String,
    pub location: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[component]
pub fn WheelchairEmergencyPage(language: RwSignal<Language>) -> impl IntoView {
    let navigate = use_navigate();
    let geolocation = use_geolocation();
    let (presets, set_presets) = signal(get_wheelchair_presets());

    let use_preset_mode = RwSignal::new(true);

    let user_name = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());
    let wheelchair_model = RwSignal::new(String::from("Manual Wheelchair"));
    let emergency_type = RwSignal::new(String::from("Breakdown"));
    let description = RwSignal::new(String::new());
    let medical_info = RwSignal::new(String::new());
    let location = RwSignal::new(String::new());

    let apply_preset = move |preset: WheelchairEmergencyPreset| {
        user_name.set(preset.user_name);
        phone.set(preset.phone);
        location.set(preset.location);
        wheelchair_model.set(preset.wheelchair_model);
        emergency_type.set(preset.emergency_type);
        description.set(preset.description);
        medical_info.set(preset.medical_info);
    };

    // let fallback_view = view! {
    //     <div class="mb-6">
    //         <button
    //             type="button"
    //             on:click=move |_| use_preset_mode.set(true)
    //             class="w-full bg-gray-500 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded-lg transition duration-200 mb-6"
    //         >
    //             "← Back to Presets"
    //         </button>
    //     </div>

    //     <form
    //         class="space-y-6"
    //         on:submit=move |e| {
    //             e.prevent_default();
    //             let (lat, lon) = geolocation
    //                 .coords
    //                 .get()
    //                 .map(|c| (Some(c.latitude()), Some(c.longitude())))
    //                 .unwrap_or((None, None));
    //             let data = WheelchairEmergencyData {
    //                 user_name: user_name.get(),
    //                 phone: phone.get(),
    //                 wheelchair_model: wheelchair_model.get(),
    //                 emergency_type: emergency_type.get(),
    //                 description: description.get(),
    //                 medical_info: medical_info.get(),
    //                 location: location.get(),
    //                 latitude: lat,
    //                 longitude: lon,
    //             };
    //             #[cfg(target_arch = "wasm32")]
    //             {
    //                 if let Some(window) = web_sys::window() {
    //                     if let Ok(Some(storage)) = window.local_storage() {
    //                         if let Ok(json_str) = serde_json::to_string(&data) {
    //                             let _ = storage.set_item("wheelchair_emergency_data", &json_str);
    //                         }
    //                     }
    //                 }
    //             }
    //             navigate("/wheelchair-emergency-details", Default::default());
    //         }
    //     >
    //         <div>
    //             <label for="user_name" class="block text-sm font-medium text-gray-700 mb-2">
    //                 "Your Name"
    //             </label>
    //             <input
    //                 id="user_name"
    //                 type="text"
    //                 required
    //                 placeholder="Enter your name"
    //                 class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
    //                 on:input=move |e| user_name.set(event_target_value(&e))
    //                 prop:value=user_name
    //             />
    //         </div>

    //         <div>
    //             <label for="phone" class="block text-sm font-medium text-gray-700 mb-2">
    //                 "Phone Number"
    //             </label>
    //             <input
    //                 id="phone"
    //                 type="tel"
    //                 required
    //                 placeholder="e.g., 123-456-7890"
    //                 class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
    //                 on:input=move |e| phone.set(event_target_value(&e))
    //                 prop:value=phone
    //             />
    //         </div>

    //         <div>
    //             <label for="location" class="block text-sm font-medium text-gray-700 mb-2">
    //                 "Current Location"
    //             </label>
    //             <input
    //                 id="location"
    //                 type="text"
    //                 required
    //                 placeholder="Enter your location"
    //                 class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
    //                 on:input=move |e| location.set(event_target_value(&e))
    //                 prop:value=location
    //             />
    //         </div>

    //         <div>
    //             <label for="wheelchair_model" class="block text-sm font-medium text-gray-700 mb-2">
    //                 "Wheelchair Type"
    //             </label>
    //             <select
    //                 id="wheelchair_model"
    //                 class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
    //                 on:change=move |e| wheelchair_model.set(event_target_value(&e))
    //                 prop:value=wheelchair_model
    //             >
    //                 <option value="Manual Wheelchair">"Manual Wheelchair"</option>
    //                 <option value="Power Wheelchair">"Power Wheelchair"</option>
    //                 <option value="Scooter">"Scooter"</option>
    //                 <option value="Motorized Scooter">"Motorized Scooter"</option>
    //                 <option value="Other">"Other"</option>
    //             </select>
    //         </div>

    //         <div>
    //             <label for="emergency_type" class="block text-sm font-medium text-gray-700 mb-2">
    //                 "Issue Type"
    //             </label>
    //             <select
    //                 id="emergency_type"
    //                 class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
    //                 on:change=move |e| emergency_type.set(event_target_value(&e))
    //                 prop:value=emergency_type
    //             >
    //                 <option value="Breakdown">"Breakdown"</option>
    //                 <option value="Flat Tire">"Flat Tire"</option>
    //                 <option value="Battery Dead">"Battery Dead"</option>
    //                 <option value="Mobility Assistance">"Mobility Assistance"</option>
    //                 <option value="Medical Emergency">"Medical Emergency"</option>
    //                 <option value="Other">"Other"</option>
    //             </select>
    //         </div>

    //         <div>
    //             <label for="description" class="block text-sm font-medium text-gray-700 mb-2">
    //                 "Description"
    //             </label>
    //             <textarea
    //                 id="description"
    //                 required
    //                 placeholder="Describe the issue and current situation"
    //                 class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition h-32 resize-none"
    //                 on:input=move |e| description.set(event_target_value(&e))
    //                 prop:value=description
    //             ></textarea>
    //         </div>

    //         <div>
    //             <label for="medical_info" class="block text-sm font-medium text-gray-700 mb-2">
    //                 "Medical Information (Optional)"
    //             </label>
    //             <textarea
    //                 id="medical_info"
    //                 placeholder="Any medical conditions or allergies"
    //                 class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition h-24 resize-none"
    //                 on:input=move |e| medical_info.set(event_target_value(&e))
    //                 prop:value=medical_info
    //             ></textarea>
    //         </div>

    //         <div class="pt-4">
    //             <button
    //                 type="submit"
    //                 class="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200 text-lg"
    //             >
    //                 "Request Assistance"
    //             </button>
    //         </div>
    //     </form>
    // };

    // let preset_view = // Preset Mode View
    // view! {
    //     <div>
    //         <div class="bg-purple-50 border-l-4 border-purple-500 p-4 mb-6">
    //             <h2 class="text-2xl font-bold text-gray-800 mb-2">"Quick Emergency Selection"</h2>
    //             <p class="text-gray-700">
    //                 "Choose a preset scenario that matches your situation. This is the fastest way to get help."
    //             </p>
    //         </div>

    //         <div class="grid grid-cols-1 gap-3 mb-8">
    //             <For
    //                 each=move || presets.clone()
    //                 key=|preset| preset.name.clone()
    //                 children=move |preset: WheelchairEmergencyPreset| {
    //                     let preset_clone = preset.clone();
    //                     view! {
    //                         <button
    //                             type="button"
    //                             on:click=move |_| {
    //                                 apply_preset(preset_clone.clone());
    //                                 use_preset_mode.set(false);
    //                             }
    //                             class="p-4 text-left bg-white border-2 border-purple-200 rounded-lg hover:bg-purple-50 hover:border-purple-500 active:border-purple-700 transition duration-200 cursor-pointer shadow-sm hover:shadow-md"
    //                         >
    //                             <h3 class="font-bold text-lg text-purple-700 mb-1">
    //                                 {preset.name}
    //                             </h3>
    //                             <p class="text-sm text-gray-600">{preset.description}</p>
    //                         </button>
    //                     }
    //                 }
    //             />
    //         </div>

    //         <div class="border-t-2 border-gray-200 pt-6">
    //             <p class="text-sm text-gray-500 mb-3">"Need more control over the details?"</p>
    //             <button
    //                 type="button"
    //                 on:click=move |_| use_preset_mode.set(false)
    //                 class="w-full bg-white hover:bg-gray-100 text-gray-700 border-2 border-gray-300 font-semibold py-3 px-4 rounded-lg transition duration-200"
    //             >
    //                 "✎ Enter Information Manually"
    //             </button>
    //         </div>
    //     </div>
    // };

    let t = move |key: &str| Translations::new(language.get()).t(key);

    view! {
        <div class="min-h-screen bg-gradient-to-b from-purple-50 to-white py-12 px-4">
            <div class="max-w-2xl mx-auto">
                <div class="mb-12">
                    <h1 class="text-4xl font-bold text-gray-900 mb-2">
                        {move || t("wheelchair.title")}
                    </h1>
                    <p class="text-lg text-gray-600">{move || t("wheelchair.subtitle")}</p>
                </div>

                // <Show
                <div class="bg-white rounded-lg shadow-lg p-8">

                    // <Show
                    // when=move ||  {use_preset_mode.get()}
                    // fallback=|| { content_view }
                    // </Show>

                    {move || match use_preset_mode.get() {
                        true if use_preset_mode.get() == true => {
                            view! {
                                <div>
                                    <div class="bg-purple-50 border-l-4 border-purple-500 p-4 mb-6">
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
                                            children=move |preset: WheelchairEmergencyPreset| {
                                                let preset_clone = preset.clone();
                                                view! {
                                                    <button
                                                        type="button"
                                                        on:click=move |_| {
                                                            apply_preset(preset_clone.clone());
                                                            use_preset_mode.set(false);
                                                        }
                                                        class="p-4 text-left bg-white border-2 border-purple-200 rounded-lg hover:bg-purple-50 hover:border-purple-500 active:border-purple-700 transition duration-200 cursor-pointer shadow-sm hover:shadow-md"
                                                    >
                                                        <h3 class="font-bold text-lg text-purple-700 mb-1">
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
                        _ => {
                            let navigate_clone = navigate.clone();
                            // false if use_preset_mode.get() == false => {
                            // // returns HtmlElement<Pre>

                            // // returns HtmlElement<P>
                            // view! { <p>"Two"</p> }
                            // .into_any()
                            // }
                            // returns HtmlElement<Textarea>
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
                                        let data = WheelchairEmergencyData {
                                            user_name: user_name.get(),
                                            phone: phone.get(),
                                            wheelchair_model: wheelchair_model.get(),
                                            emergency_type: emergency_type.get(),
                                            description: description.get(),
                                            medical_info: medical_info.get(),
                                            location: location.get(),
                                            latitude: lat,
                                            longitude: lon,
                                        };
                                        #[cfg(target_arch = "wasm32")]
                                        {
                                            if let Some(window) = web_sys::window() {
                                                if let Ok(Some(storage)) = window.local_storage() {
                                                    if let Ok(json_str) = serde_json::to_string(&data) {
                                                        let _ = storage
                                                            .set_item("wheelchair_emergency_data", &json_str);
                                                    }
                                                }
                                            }
                                        }
                                        navigate_clone(
                                            "/wheelchair-emergency-details",
                                            Default::default(),
                                        );
                                    }
                                >
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
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
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
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| phone.set(event_target_value(&e))
                                            prop:value=phone
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="location"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Current Location"
                                        </label>
                                        <input
                                            id="location"
                                            type="text"
                                            required
                                            placeholder="Enter your location"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
                                            on:input=move |e| location.set(event_target_value(&e))
                                            prop:value=location
                                        />
                                    </div>

                                    <div>
                                        <label
                                            for="wheelchair_model"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Wheelchair Type"
                                        </label>
                                        <select
                                            id="wheelchair_model"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                wheelchair_model.set(event_target_value(&e))
                                            }
                                            prop:value=wheelchair_model
                                        >
                                            <option value="Manual Wheelchair">
                                                "Manual Wheelchair"
                                            </option>
                                            <option value="Power Wheelchair">"Power Wheelchair"</option>
                                            <option value="Scooter">"Scooter"</option>
                                            <option value="Motorized Scooter">
                                                "Motorized Scooter"
                                            </option>
                                            <option value="Other">"Other"</option>
                                        </select>
                                    </div>

                                    <div>
                                        <label
                                            for="emergency_type"
                                            class="block text-sm font-medium text-gray-700 mb-2"
                                        >
                                            "Issue Type"
                                        </label>
                                        <select
                                            id="emergency_type"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition"
                                            on:change=move |e| {
                                                emergency_type.set(event_target_value(&e))
                                            }
                                            prop:value=emergency_type
                                        >
                                            <option value="Breakdown">"Breakdown"</option>
                                            <option value="Flat Tire">"Flat Tire"</option>
                                            <option value="Battery Dead">"Battery Dead"</option>
                                            <option value="Mobility Assistance">
                                                "Mobility Assistance"
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
                                            "Description"
                                        </label>
                                        <textarea
                                            id="description"
                                            required
                                            placeholder="Describe the issue and current situation"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition h-32 resize-none"
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
                                            placeholder="Any medical conditions or allergies"
                                            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-transparent outline-none transition h-24 resize-none"
                                            on:input=move |e| medical_info.set(event_target_value(&e))
                                            prop:value=medical_info
                                        ></textarea>
                                    </div>

                                    <div class="pt-4">
                                        <button
                                            type="submit"
                                            class="w-full bg-purple-600 hover:bg-purple-700 text-white font-bold py-3 px-6 rounded-lg transition duration-200 text-lg"
                                        >
                                            "Request Assistance"
                                        </button>
                                    </div>
                                </form>
                            }
                                .into_any()
                        }
                    }}

                </div>
            </div>
        </div>
    }
}

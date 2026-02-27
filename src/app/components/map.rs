use leptos::prelude::*;
use leptos::{view, IntoView};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = L)]
    pub fn map(element_id: &str) -> JsValue;

    #[wasm_bindgen(js_name = tileLayer, js_namespace = L)]
    pub fn tile_layer(url: &str, options: &JsValue) -> JsValue;

    #[wasm_bindgen(js_name = marker, js_namespace = L)]
    pub fn marker(lat_lng: &JsValue) -> JsValue;
}

#[component]
pub fn LocationMap(latitude: f64, longitude: f64, location_name: String) -> impl IntoView {
    let map_container_id = format!("map-{}", generate_id());
    let id_for_effect = map_container_id.clone();

    Effect::new(move || {
        setup_map(&id_for_effect, latitude, longitude, &location_name);
    });

    view! {
        <div
            id=map_container_id
            class="w-full h-96 rounded-lg shadow-lg border-2 border-gray-200"
            style="margin: 20px 0;"
        ></div>
    }
}

#[allow(unused_variables)]
fn setup_map(element_id: &str, latitude: f64, longitude: f64, location_name: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(_window) = web_sys::window() {
            let id_clone = element_id.to_string();
            let name_clone = location_name.to_string();

            let timeout_fn = Closure::once(move || {
                let map_element = web_sys::window()
                    .and_then(|w| w.document())
                    .and_then(|d| d.get_element_by_id(&id_clone));

                if map_element.is_some() {
                    initialize_map(&id_clone, latitude, longitude, &name_clone);
                }
            });

            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                timeout_fn.as_ref().unchecked_ref(),
                100,
            );
            timeout_fn.forget();
        }
    }
}

#[allow(unused_variables)]
#[cfg(target_arch = "wasm32")]
fn initialize_map(element_id: &str, latitude: f64, longitude: f64, location_name: &str) {
    // Create the map
    let map = map(element_id);

    // Create lat/lng array [latitude, longitude]
    let lat_lng = js_sys::Array::new();
    lat_lng.push(&JsValue::from_f64(latitude));
    lat_lng.push(&JsValue::from_f64(longitude));

    // Set the view on the map using Reflect::apply
    let set_view_fn = js_sys::Reflect::get(&map, &"setView".into()).ok();
    if let Some(set_view_fn) = set_view_fn {
        let args = js_sys::Array::new();
        args.push(&lat_lng);
        args.push(&JsValue::from_f64(13.0));
        let _ = js_sys::Reflect::apply(
            &set_view_fn.unchecked_into::<js_sys::Function>(),
            &map,
            &args,
        );
    }

    // Create tile layer options with attribution
    let options = js_sys::Object::new();
    js_sys::Reflect::set(
        &options,
        &"attribution".into(),
        &"© OpenStreetMap contributors".into(),
    )
    .ok();

    // Add the tile layer
    let tile_layer = tile_layer(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &options,
    );

    // Call addTo on tile layer
    let add_to_fn = js_sys::Reflect::get(&tile_layer, &"addTo".into()).ok();
    if let Some(add_to_fn) = add_to_fn {
        let args = js_sys::Array::new();
        args.push(&map);
        let _ = js_sys::Reflect::apply(
            &add_to_fn.unchecked_into::<js_sys::Function>(),
            &tile_layer,
            &args,
        );
    }

    // Add a marker at the location
    let marker_obj = marker(&lat_lng);
    let popup_text = format!("{}\n({:.6}, {:.6})", location_name, latitude, longitude);

    // Call bindPopup on marker
    let bind_popup_fn = js_sys::Reflect::get(&marker_obj, &"bindPopup".into()).ok();
    if let Some(bind_popup_fn) = bind_popup_fn {
        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(&popup_text));
        let _ = js_sys::Reflect::apply(
            &bind_popup_fn.unchecked_into::<js_sys::Function>(),
            &marker_obj,
            &args,
        );
    }

    // Call addTo on marker
    let add_to_fn = js_sys::Reflect::get(&marker_obj, &"addTo".into()).ok();
    if let Some(add_to_fn) = add_to_fn {
        let args = js_sys::Array::new();
        args.push(&map);
        let _ = js_sys::Reflect::apply(
            &add_to_fn.unchecked_into::<js_sys::Function>(),
            &marker_obj,
            &args,
        );
    }
}

fn generate_id() -> String {
    thread_local! {
        static COUNTER: RefCell<u32> = RefCell::new(0);
    }

    COUNTER.with(|c| {
        let mut val = c.borrow_mut();
        let current = *val;
        *val += 1;
        format!("map-container-{}", current)
    })
}

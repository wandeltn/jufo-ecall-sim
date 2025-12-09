use crate::app::backend::api::add_event_api;
use crate::app::components::navbar::NavbarComponent;
use leptos::leptos_dom::logging::console_debug_log;
use leptos::prelude::*;
use leptos::{view, IntoView};
use phosphor_leptos::{Icon, IconWeight, CUBE};
// use thaw::DatePicker;

#[component]
pub fn CreateEventPage() -> impl IntoView {
    let event_name = RwSignal::new("".to_string());
    let event_date = RwSignal::new("".to_string());
    let location = RwSignal::new("".to_string());

    // add_event_api(name, event_date, location, image_base64);

    fn on_submit(
        event_name: RwSignal<String>,
        event_date: RwSignal<String>,
        location: RwSignal<String>,
    ) {
        console_debug_log("Creating Event");
    }

    view! {
        <div>
            <NavbarComponent/>
            <h1>"Create Event"</h1>
            <div class="lg:items-center lg:justify-center lg:flex lg:gap-20 lg:mr-8 lg:ml-8 mt-8">
                <div class="flex items-center flex-wrap gap-4 mb-4 sm:mb-8 lg:mb-12">
                    <form on:submit=move |_| on_submit(event_name.clone(), event_date.clone(), location.clone()) class="flex flex-col gap-4 bg-white p-6 rounded shadow-md w-full max-w-md">
                        <h2 class="text-2xl font-bold mb-4">"New Event Details"</h2>
                        <input class="flex flex-2 bg-gray-200 p-2 rounded" type="text" bind:value=event_name />
                        <input class="flex flex-1 bg-gray-200 p-2 rounded" type="date" bind:value=event_date />
                        <input class="flex flex-2 bg-gray-200 p-2 rounded" type="text" bind:value=location />
                        <button type="submit" class="bg-blue-500 text-white p-2 rounded">"Create Event"</button>
                    </form>
                </div>
            </div>
            <Icon icon=CUBE size="48px" weight=IconWeight::Regular/>
            </div>
    }
}

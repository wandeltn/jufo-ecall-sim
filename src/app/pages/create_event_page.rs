use crate::app::backend::api::add_event_api;
use crate::app::components::navbar::NavbarComponent;
use leptos::leptos_dom::logging::console_debug_log;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::{view, IntoView};
use phosphor_leptos::{Icon, IconWeight, TICKET};
// use thaw::DatePicker;

#[component]
pub fn CreateEventPage() -> impl IntoView {
    let event_name = RwSignal::new("".to_string());
    let event_date = RwSignal::new("".to_string());
    let location = RwSignal::new("".to_string());
    let description = RwSignal::new("".to_string());
    let category = RwSignal::new("".to_string());
    let event_time = RwSignal::new("".to_string());
    let location_name = RwSignal::new("".to_string());
    let ticket_price = RwSignal::new("".to_string());
    let ticket_quantity = RwSignal::new("".to_string());


    // add_event_api(name, event_date, location, image_base64);

    async fn on_submit(event_name: String, event_date: String, location: String) {

        console_debug_log(&format!(
            "Creating Event with details: Name: {}, Date: {}, Location: {}",
            event_name, event_date, location
        ));

        match add_event_api(event_name, event_date, Some(location), Some("".to_string())).await {
            Ok(_) => console_debug_log("Event created successfully."),
            Err(e) => console_debug_log(&format!("Failed to create event: {:?}", e)),
        }
    }

    view! {
        <div class="min-h-screen bg-background">
            <NavbarComponent />
            <h1 class="text-4xl font-heading font-bold mb-2">"Create Event"</h1>
            <p class="text-muted-foreground mb-8">
                "Fill in the details below to create a new event."
            </p>
            <div class="">

                <div class="flex items-center flex-col flex-wrap gap-4 mb-4 sm:mb-8 lg:mb-12">
                    <div class="flex flex-col gap-4 bg-white p-6 rounded shadow-md w-full max-w-md">
                        <h2 class="text-2xl font-bold mb-4">"Event Details"</h2>
                        <input
                            class="flex flex-2 bg-gray-200 p-2 rounded"
                            type="text"
                            bind:value=event_name
                            placeholder="Event Name"
                        />

                        <div class="flex gap-2">
                            <div class="size-4 flex-none">
                                <Icon icon=TICKET size="24px" weight=IconWeight::Regular />
                            </div>
                            <p class="grow">"Date & Time:"</p>
                            <input
                                class="flex-1 bg-gray-200 p-2 rounded"
                                type="date"
                                bind:value=event_date
                            />

                            <input
                                class="flex-1 bg-gray-200 p-2 rounded"
                                type="time"
                                bind:value=event_time
                            />
                        </div>

                        <input
                            class="flex flex-2 bg-gray-200 p-2 rounded"
                            type="text"
                            bind:value=location
                            placeholder="Where is the event happening?"
                        />

                        <input
                            class="flex flex-2 bg-gray-200 p-2 rounded"
                            type="text"
                            bind:value=location_name
                            placeholder="Location Name"
                        />

                        <input
                            class="flex flex-1 bg-gray-200 p-2 rounded"
                            type="text"
                            bind:value=description
                            placeholder="Describe your event in detail..."
                        />

                        <select class="flex flex-1 bg-gray-200 p-2 rounded" bind:value=category>
                            <option value="">Select a category</option>
                            <option value="Music">Music</option>
                            <option value="Art">Art</option>
                            <option value="Tech">Tech</option>
                        </select>

                        <div class="gap-4">
                            <input
                                class="flex flex-1 bg-gray-200 p-2 rounded"
                                type="number"
                                bind:value=ticket_price
                                placeholder="Ticket Price"
                                min="0"
                                step="0.01"
                            />
                            <input
                                class="flex flex-1 bg-gray-200 p-2 rounded"
                                type="number"
                                bind:value=ticket_quantity
                                placeholder="Ticket Quantity"
                                min="0"
                            />
                        </div>
                    </div>
                    <button
                        type="button"
                        on:click=move |_| spawn_local(
                            on_submit(event_name.get(), event_date.get(), location.get()),
                        )
                        class="bg-blue-500 text-white p-2 rounded"
                    >
                        "Create Event"
                    </button>
                </div>
            </div>
        </div>
    }
}

use crate::app::backend::api::add_event_api;
use chrono::Local;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::{view, IntoView};
use phosphor_leptos::{Icon, IconWeight, TICKET};
use thaw::{
    DatePicker, 
    Input,
    TimePicker,
    Button,
    ButtonAppearance,
    Select,
};

#[component]
pub fn CreateEventPage() -> impl IntoView {
    let event_name = RwSignal::new("".to_string());
    let event_date = RwSignal::new(Local::now().date_naive());
    let location = RwSignal::new("".to_string());
    let description = RwSignal::new("".to_string());
    let category = RwSignal::new("".to_string());
    let event_time = RwSignal::new("".to_string());
    let location_name = RwSignal::new("".to_string());
    let ticket_price = RwSignal::new("".to_string());
    let ticket_quantity = RwSignal::new("".to_string());

    // add_event_api(name, event_date, location, image_base64);

    async fn on_submit(event_name: String, event_date: String, location: String) {

        console_log(&format!(
            "Creating Event with details: Name: {}, Date: {}, Location: {}",
            event_name, event_date, location
        ));

        match add_event_api(event_name, event_date, Some(location), Some("".to_string())).await {
            Ok(_) => console_log("Event created successfully."),
            Err(e) => console_log(&format!("Failed to create event: {:?}", e)),
        }
    }

    view! {
        <div class="min-h-screen bg-background">
            <h1 class="text-4xl font-heading font-bold mb-2">"Create Event"</h1>
            <p class="text-muted-foreground mb-8">
                "Fill in the details below to create a new event."
            </p>
            <div class="">

                <div class="flex items-center flex-col flex-wrap gap-4 mb-4 sm:mb-8 lg:mb-12">
                    <div class="flex flex-col gap-4 bg-white p-6 rounded shadow-md w-full max-w-md">
                        <h2 class="text-2xl font-bold mb-4">"Event Details"</h2>

                        <Input value=event_name placeholder="Event Name" />

                        <div>
                            <div>
                                <Icon icon=TICKET size="24px" weight=IconWeight::Regular />
                            </div>
                            <p>"Date & Time:"</p>
                            <DatePicker value=event_date />

                        // <TimePicker value=event_time />
                        </div>

                        <Input value=location placeholder="Where is the event happening?" />

                        <Input value=location_name placeholder="Location Name" />

                        <Input value=description placeholder="Describe your event in detail..." />

                        <Select value=category>
                            <option value="">Select a category</option>
                            <option value="Music">Music</option>
                            <option value="Art">Art</option>
                            <option value="Tech">Tech</option>
                        </Select>

                        <div>
                            <Input value=ticket_price placeholder="Ticket Price" />
                            <Input value=ticket_quantity placeholder="Ticket Quantity" />
                        </div>
                    </div>

                    <Button appearance=ButtonAppearance::Primary>"Click Me"</Button>
                // <button
                // type="button"
                // on:click=move |_| spawn_local(
                // on_submit(event_name.get(), event_date.get(), location.get()),
                // )
                // class="bg-blue-500 text-white p-2 rounded"
                // >
                // "Create Event"
                // </button>
                </div>
            </div>
        </div>
    }
}

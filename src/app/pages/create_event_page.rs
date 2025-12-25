use std::f64;

use crate::app::backend::api::add_event_api;
use chrono::Local;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::{view, IntoView};
use phosphor_leptos::{Icon, IconWeight, CALENDAR_DOT, COINS, CURRENCY_EUR, MAP_PIN, TICKET};
use thaw::{
    Button, ButtonAppearance, Card, DatePicker, Input, InputSuffix, Label, LabelSize, Select,
    SpinButton, Textarea, TimePicker,
};

#[component]
pub fn CreateEventPage() -> impl IntoView {
    let event_name = RwSignal::new("".to_string());
    let event_date = RwSignal::new(Local::now().date_naive());
    let location = RwSignal::new("".to_string());
    let description = RwSignal::new("".to_string());
    let category = RwSignal::new("".to_string());
    let event_time = RwSignal::new(Local::now().time());
    let location_name = RwSignal::new("".to_string());
    let ticket_price = RwSignal::new(0.0f64);
    let ticket_quantity = RwSignal::new(0i32);

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
        <div class="flex flex-col md:mx-16 mx-4 my-8 gap-8">
            <h1 class="text-4xl font-heading font-bold mb-2 sm:justify-self-center justify-self-center">"Create Event"</h1>
            <p class="text-muted-foreground mb-8">
                "Fill in the details below to create a new event."
            </p>
            <Card class="mt-4">
                <div class="flex items-center gap-2 mb-2">
                    <Icon icon=TICKET size="24px" weight=IconWeight::Regular />
                    <Label size=LabelSize::Large>"Event Details"</Label>
                </div>

                <div class="flex flex-col gap-2">
                    <Label>"Event Title"</Label>
                    <Input value=event_name placeholder="e.g., Summer Music Festival 2025" />
                </div>

                <div class="flex flex-col gap-2 mt-4">
                    <Label>"Event Description"</Label>
                    <Textarea value=description placeholder="Describe your event in detail..." />
                </div>

                <div class="flex flex-col gap-2 mt-4">
                    <Label>"Event Category"</Label>
                    <Select value=category>
                        <option value="">Select a category</option>
                        <option value="Music">Music</option>
                        <option value="Art">Art</option>
                        <option value="Tech">Tech</option>
                    </Select>
                </div>
            </Card>
            <Card class="mt-4">
                <div class="flex flex-col gap-4">
                    <div class="flex items-center gap-2 mb-2">
                        <Icon icon=CALENDAR_DOT size="24px" weight=IconWeight::Regular />
                        <p>"Date & Time:"</p>
                    </div>

                    <div class="flex md:flex-row flex-col gap-4 grow w-full">
                        <div class="flex flex-col w-full">
                            <Label>"Date"</Label>
                            <DatePicker value=event_date />
                        </div>

                        <div class="flex flex-col w-full">
                            <Label>"Time"</Label>
                            <TimePicker class="grow w-full" value=event_time />
                        </div>
                    </div>
                </div>
            </Card>
            <Card class="mt-4">

                <div class="flex items-center gap-2 mb-2">
                    <Icon icon=MAP_PIN size="24px" weight=IconWeight::Regular />
                    <Label size=LabelSize::Large>"Location"</Label>
                </div>

                <div class="flex flex-col gap-2">
                    <Label>"Event Location"</Label>
                    <Input value=location placeholder="Where is the event happening?" />
                </div>

                <div class="flex flex-col gap-2 mt-4">
                    <Label>"Location Name"</Label>
                    <Input value=location_name placeholder="Location Name" />
                </div>

            </Card>
            <Card class="mt-4">
                <div class="flex items-center gap-2 mb-2">
                    <Icon icon=COINS size="24px" weight=IconWeight::Regular />
                    <Label size=LabelSize::Large>"Pricing & Tickets"</Label>
                </div>

                <div class="flex md:flex-row flex-col gap-4 grow w-full">
                    <div class="flex flex-col w-full">
                        <Label>"Date"</Label>
                        <SpinButton<f64> value=ticket_price step_page=1f64 placeholder="0.00" />
                    </div>

                    <div class="flex flex-col w-full">
                        <Label>"Time"</Label>
                        <SpinButton<i32> value=ticket_quantity step_page=10 placeholder="100" />
                    </div>
                </div>
            // <button
            // type="button"
            // on:click=move |_| spawn_local(
                // on_submit(event_name.get(), event_date.get(), location.get()),
                // )
                // class="bg-blue-500 text-white p-2 rounded"
                // >
                // "Create Event"
                // </button>
            </Card>
            <div class="flex md:flex-row flex-col gap-4 mt-4 justify-center-safe">
                <Button class="basis-xl max-h-10" appearance=ButtonAppearance::Primary>"Create Event"</Button>
                <Button class="basis-xs max-h-10" appearance=ButtonAppearance::Secondary>"Cancel"</Button>
            </div>
        </div>
    }
}

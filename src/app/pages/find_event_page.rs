use crate::app::components::navbar::NavbarComponent;
use leptos::prelude::*;
use leptos::{view, IntoView};

#[component]
pub fn FindEventPage() -> impl IntoView {
    view! {
        <div>
            <NavbarComponent/>
            <h1>"Find Event"</h1>
        </div>
    }
}

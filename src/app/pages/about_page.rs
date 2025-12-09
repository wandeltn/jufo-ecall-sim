use crate::app::components::navbar::NavbarComponent;
use leptos::prelude::*;
use leptos::{view, IntoView};

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <NavbarComponent/>
            <h1>"About"</h1>
        </div>
    }
}

use leptos::prelude::*;

#[component]
pub fn Event(#[prop(optional)] number: u32) -> impl IntoView {
    view! {
        <div class="border p-4 m-4 rounded shadow">
            <h2>"Event Component"</h2>
            <p>"This is the Event component content with number: "{number}</p>
        </div>
    }
}

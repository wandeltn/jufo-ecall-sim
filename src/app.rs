use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment, WildcardSegment,
};
use thaw::ConfigProvider;

pub mod backend;
pub mod components;
pub mod pages;

use crate::app::pages::{
    about_page::AboutPage, create_event_page::CreateEventPage, find_event_page::FindEventPage,
    home_page::HomePage,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/event-tickets-rust.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page

        <ConfigProvider>
            <Router>
                <main>
                    <Routes fallback=move || "Not found.">
                        <Route path=path!("") view=HomePage />
                        <Route path=StaticSegment("find") view=FindEventPage />
                        <Route path=path!("create") view=CreateEventPage />
                        <Route path=path!("about") view=AboutPage />
                        <Route path=WildcardSegment("any") view=NotFound />
                    </Routes>
                </main>
            </Router>
        </ConfigProvider>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}

use crate::app::components::navbar::NavbarComponent;
use crate::app::pages::i18n::Language;
use leptos::prelude::*;
use leptos_captcha::Captcha;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment, WildcardSegment,
};
use leptos_use::{
    use_color_mode_with_options, use_preferred_dark, ColorMode, UseColorModeOptions,
    UseColorModeReturn,
};
use thaw::{ConfigProvider, Theme};

pub mod backend;
pub mod components;
pub mod pages;

use crate::app::pages::{
    about_page::AboutPage, boat_emergency_details_page::BoatEmergencyDetailsPage,
    boat_emergency_page::BoatEmergencyPage, create_event_page::CreateEventPage,
    ecall_details_page::EcallDetailsPage, ecall_simulator_page::EcallSimulatorPage,
    find_event_page::FindEventPage, landing_page::LandingPage,
    lift_emergency_details_page::LiftEmergencyDetailsPage, lift_emergency_page::LiftEmergencyPage,
    simulators_page::SimulatorsPage, wheelchair_emergency_details_page::WheelchairEmergencyDetailsPage,
    wheelchair_emergency_page::WheelchairEmergencyPage,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let is_dark_preferred = use_preferred_dark();

    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));

    let theme = RwSignal::new(
        if is_dark_preferred.get() || mode.get() == ColorMode::Dark {
            set_mode.set(ColorMode::Dark);
            Theme::dark()
        } else {
            set_mode.set(ColorMode::Light);
            Theme::light()
        },
    );
    let is_pending = RwSignal::new(Option::None);
    let language = RwSignal::new(Language::German);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/event-tickets-rust.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page

        <ConfigProvider theme>
            <Router>
                <main>
                    <NavbarComponent theme=theme language=language />
                    <Routes fallback=move || "Not found.">
                        <Route path=path!("") view=move || view! { <LandingPage language /> } />
                        <Route
                            path=path!("simulators")
                            view=move || view! { <SimulatorsPage language /> }
                        />
                        <Route
                            path=path!("ecall")
                            view=move || view! { <EcallSimulatorPage language /> }
                        />
                        <Route path=StaticSegment("find") view=FindEventPage />
                        <Route path=path!("create") view=CreateEventPage />
                        <Route path=path!("about") view=move || view! { <AboutPage language /> } />
                        <Route path=path!("ecall-details") view=EcallDetailsPage />
                        <Route
                            path=path!("boat-emergency")
                            view=move || view! { <BoatEmergencyPage language /> }
                        />
                        <Route path=path!("boat-emergency-details") view=BoatEmergencyDetailsPage />
                        <Route
                            path=path!("wheelchair-emergency")
                            view=move || view! { <WheelchairEmergencyPage language /> }
                        />
                        <Route
                            path=path!("wheelchair-emergency-details")
                            view=WheelchairEmergencyDetailsPage
                        />
                        <Route
                            path=path!("lift-emergency")
                            view=move || view! { <LiftEmergencyPage language /> }
                        />
                        <Route path=path!("lift-emergency-details") view=LiftEmergencyDetailsPage />
                        <Route path=WildcardSegment("any") view=NotFound />
                    </Routes>

                    <Captcha is_pending />
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

#[server]
pub async fn get_pow() -> Result<String, ServerFnError> {
    use leptos_captcha::spow::pow::Pow;

    // I highly suggest, that you create a global static variable in your app
    // as an indicator if you are in DEV / DEBUG mode, or something like that.
    // You could pull it from the context, or where ever it makes sense for you.
    // In debug mode, the speed of the verification in the UI is a lot slower, and
    // you should just use the lowest difficulty of `10` during development.
    const DEV_MODE: bool = false;

    if DEV_MODE {
        Ok(Pow::with_difficulty(10, 10)?.to_string())
    } else {
        Ok(Pow::new(10)?.to_string())
    }
}

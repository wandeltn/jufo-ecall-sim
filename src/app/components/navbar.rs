use leptos::prelude::*;
use leptos_use::{use_color_mode_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn};
use phosphor_leptos::{Icon, MOON_STARS, SUN};
use thaw::{Button, ButtonAppearance, ConfigProvider, Theme};

#[component]
pub fn NavbarComponent(theme: RwSignal<Theme>) -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));

    view! {
        <ConfigProvider theme=theme>
            <nav class="sticky top-0 z-50 w-full border-b bg-card/95 backdrop-blur supports-[backdrop-filter]:bg-card/60">
                <div></div>
                <div class="hidden md:flex items-center gap-4">
                    <a
                        class="container mx-auto px-4 h-16 flex items-center justify-between"
                        href="/"
                    >
                        "Home"
                    </a>
                    <a class="font-medium" href="/find">
                        "Events"
                    </a>
                    <a class="" href="/create">
                        "Create Event"
                    </a>
                    <a class="" href="/about">
                        "About"
                    </a>
                    <Button
                        appearance=ButtonAppearance::Transparent
                        on_click={
                            let mode = mode.clone();
                            let set_mode = set_mode.clone();
                            move |_| {
                                if mode.get() == ColorMode::Dark {
                                    set_mode.set(ColorMode::Light);
                                    theme.set(Theme::light());
                                } else {
                                    set_mode.set(ColorMode::Dark);
                                    theme.set(Theme::dark());
                                }
                            }
                        }
                    >
                    {move || {
                        if mode.get() == ColorMode::Light { view!{<Icon icon=MOON_STARS size="32px" />} } else { view!{<Icon icon=SUN size="32px" />} }
                    }}
                    </Button>
                </div>
            </nav>
        </ConfigProvider>
    }
}

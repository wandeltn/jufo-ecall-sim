use crate::app::pages::i18n::Language;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_use::{use_color_mode_with_options, ColorMode, UseColorModeOptions, UseColorModeReturn};
use phosphor_leptos::{Icon, LIST, MOON_STARS, SUN};
use thaw::{Button, ButtonAppearance, ConfigProvider, Theme};

#[component]
pub fn NavbarComponent(theme: RwSignal<Theme>, language: RwSignal<Language>) -> impl IntoView {
    let UseColorModeReturn { mode, set_mode, .. } =
        use_color_mode_with_options(UseColorModeOptions::default().cookie_enabled(true));

    let menu_open = RwSignal::new(false);

    view! {
        <ConfigProvider theme=theme>
            <nav class="sticky top-0 z-50 w-full border-b bg-white dark:bg-slate-900 shadow-md">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        {} <div class="flex-shrink-0">
                            <A href="/">
                                <div class="text-2xl font-bold text-red-600 hover:text-red-700 transition">
                                    "Emergency Response Simulator"
                                </div>
                            </A>
                        </div> {}
                        <div class="hidden md:flex items-center space-x-1">
                            {} <div class="relative group">
                                <button class="px-3 py-2 rounded-md text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-red-50 dark:hover:bg-red-900 hover:text-red-600 transition duration-200 flex items-center gap-2">
                                    "Emergency Services"
                                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                                        <path
                                            fill-rule="evenodd"
                                            d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                                            clip-rule="evenodd"
                                        />
                                    </svg>
                                </button>
                                <div class="absolute left-0 mt-0 w-48 bg-white dark:bg-slate-800 rounded-md shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200">
                                    <div class="px-0 py-1">
                                        <A href="/simulators">
                                            <div class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 hover:text-gray-900 dark:hover:text-white font-semibold border-b border-gray-200 dark:border-gray-600">
                                                "🚀 All Simulators"
                                            </div>
                                        </A>
                                        <A href="/ecall">
                                            <div class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-red-50 dark:hover:bg-red-900 hover:text-red-600">
                                                "🚗 Auto e-call"
                                            </div>
                                        </A>
                                        <A href="/boat-emergency">
                                            <div class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-blue-50 dark:hover:bg-blue-900 hover:text-blue-600">
                                                "⛵ Marine"
                                            </div>
                                        </A>
                                        <A href="/wheelchair-emergency">
                                            <div class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-purple-50 dark:hover:bg-purple-900 hover:text-purple-600">
                                                "♿ Mobility Assistance"
                                            </div>
                                        </A>
                                        <A href="/lift-emergency">
                                            <div class="block px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-yellow-50 dark:hover:bg-yellow-900 hover:text-yellow-600">
                                                "🛗 Lift/Elevator"
                                            </div>
                                        </A>
                                    </div>
                                </div>
                            // Other links
                            </div> // <A href="/find">
                            // <div class="px-3 py-2 rounded-md text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-blue-50 dark:hover:bg-blue-900 hover:text-blue-600 transition duration-200">
                            // "📅 Events"
                            // </div>
                            // </A>
                            // <A href="/create">
                            // <div class="px-3 py-2 rounded-md text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-green-50 dark:hover:bg-green-900 hover:text-green-600 transition duration-200">
                            // "➕ Create Event"
                            // </div>
                            // </A>
                            <A href="/about">
                                <div class="px-3 py-2 rounded-md text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition duration-200">
                                    "ℹ️ About"
                                </div>
                            </A>
                        </div> {}
                        <div class="flex items-center space-x-4">
                            {} <div class="relative group">
                                <button class="px-3 py-2 rounded-md text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition duration-200">
                                    {move || match language.get() {
                                        Language::English => "EN",
                                        Language::German => "DE",
                                    }}
                                </button>
                                <div class="absolute right-0 mt-0 w-32 bg-white dark:bg-slate-800 rounded-md shadow-lg opacity-0 invisible group-hover:opacity-100 group-hover:visible transition-all duration-200 z-50">
                                    <div class="px-0 py-1">
                                        <button
                                            on:click=move |_| language.set(Language::English)
                                            class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700"
                                        >
                                            "English"
                                        </button>
                                        <button
                                            on:click=move |_| language.set(Language::German)
                                            class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700"
                                        >
                                            "Deutsch"
                                        </button>
                                    </div>
                                </div>
                            </div>
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
                                class="hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg p-2 transition"
                            >
                                {move || {
                                    if mode.get() == ColorMode::Light {
                                        view! { <Icon icon=MOON_STARS size="24px" /> }
                                    } else {
                                        view! { <Icon icon=SUN size="24px" /> }
                                    }
                                }}
                            </Button>
                            {}
                            <button
                                on:click=move |_| {
                                    menu_open.set(!menu_open.get());
                                }
                                class="md:hidden inline-flex items-center justify-center p-2 rounded-md text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition"
                            >
                                <Icon icon=LIST size="24px" />
                            </button>
                        </div>
                    </div>
                </div>

                {}
                <Show when=move || menu_open.get()>
                    <div class="md:hidden bg-white dark:bg-slate-800 border-t border-gray-200 dark:border-gray-700">
                        <div class="px-2 pt-2 pb-3 space-y-1">
                            <A href="/simulators">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 font-semibold">
                                    "🚀 All Simulators"
                                </div>
                            </A>
                            <div class="border-t border-gray-200 dark:border-gray-600 my-1"></div>
                            <A href="/ecall">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-red-50 dark:hover:bg-red-900 hover:text-red-600">
                                    "🚗 Auto e-call"
                                </div>
                            </A>
                            <A href="/boat-emergency">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-blue-50 dark:hover:bg-blue-900 hover:text-blue-600">
                                    "⛵ Marine"
                                </div>
                            </A>
                            <A href="/wheelchair-emergency">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-purple-50 dark:hover:bg-purple-900 hover:text-purple-600">
                                    "♿ Mobility Assistance"
                                </div>
                            </A>
                            <A href="/lift-emergency">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-yellow-50 dark:hover:bg-yellow-900 hover:text-yellow-600">
                                    "🛗 Lift/Elevator"
                                </div>
                            </A>
                            <div class="border-t border-gray-200 dark:border-gray-700 my-2"></div>
                            <A href="/find">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-blue-50 dark:hover:bg-blue-900 hover:text-blue-600">
                                    "📅 Events"
                                </div>
                            </A>
                            <A href="/create">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-green-50 dark:hover:bg-green-900 hover:text-green-600">
                                    "➕ Create Event"
                                </div>
                            </A>
                            <A href="/about">
                                <div class="block px-3 py-2 rounded-md text-base font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700">
                                    "ℹ️ About"
                                </div>
                            </A>
                        </div>
                    </div>
                </Show>
            </nav>
        </ConfigProvider>
    }
}

use crate::app::pages::i18n::{Language, Translations};
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_router::components::A;

#[component]
pub fn LandingPage(language: RwSignal<Language>) -> impl IntoView {
    let t = move |key: &str| Translations::new(language.get()).t(key);
    view! {
        <div class="min-h-screen bg-gradient-to-br from-red-50 via-white to-blue-50">
            {} <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
                <div class="text-center mb-16">
                    <h1 class="text-5xl md:text-6xl font-bold text-gray-900 mb-6">
                        {move || t("landing.hero_title")}
                    </h1>
                    <p class="text-xl md:text-2xl text-gray-600 mb-8 max-w-3xl mx-auto">
                        {move || t("landing.hero_subtitle")}
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <A href="/simulators">
                            <div class="px-8 py-3 bg-red-600 hover:bg-red-700 text-white font-bold rounded-lg transition duration-200 cursor-pointer">
                                {move || t("landing.launch_simulators")}
                            </div>
                        </A>
                        <A href="/about">
                            <div class="px-8 py-3 bg-gray-200 hover:bg-gray-300 text-gray-800 font-bold rounded-lg transition duration-200 cursor-pointer">
                                {move || t("landing.learn_more")}
                            </div>
                        </A>
                    </div>
                </div>
            </div> {} <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    {move || t("landing.about_title")}
                </h2>
                <div class="grid md:grid-cols-2 gap-8 mb-16">
                    <div class="bg-white rounded-lg shadow-lg p-8 border-l-4 border-red-600">
                        <h3 class="text-2xl font-bold text-gray-900 mb-4">
                            {move || t("landing.what_is_this")}
                        </h3>
                        <p class="text-gray-700 leading-relaxed">
                            {move || t("landing.what_is_this_desc")}
                        </p>
                    </div>

                    <div class="bg-white rounded-lg shadow-lg p-8 border-l-4 border-blue-600">
                        <h3 class="text-2xl font-bold text-gray-900 mb-4">
                            {move || t("landing.key_features")}
                        </h3>
                        <ul class="text-gray-700 space-y-3">
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3">v</span>
                                <span>{move || t("landing.feature_presets")}</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3">v</span>
                                <span>{move || t("landing.feature_geolocation")}</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3">v</span>
                                <span>{move || t("landing.feature_prefill")}</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3">v</span>
                                <span>{move || t("landing.feature_standards")}</span>
                            </li>
                        </ul>
                    </div>
                </div>

                {}
                <h2 class="text-3xl font-bold text-center text-gray-900 mb-12">
                    {move || t("landing.simulators_title")}
                </h2>
                <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
                    {} <A href="/ecall">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition duration-200 cursor-pointer h-full border-t-4 border-red-600">
                            <div class="text-4xl mb-3">"🚗"</div>
                            <h3 class="text-xl font-bold text-gray-900 mb-2">
                                {move || t("landing.auto_ecall_title")}
                            </h3>
                            <p class="text-gray-600 text-sm">
                                {move || t("landing.auto_ecall_desc")}
                            </p>
                            <div class="mt-4 text-red-600 font-semibold text-sm">
                                "→ Launch Simulator"
                            </div>
                        </div>
                    </A> {} <A href="/boat-emergency">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition duration-200 cursor-pointer h-full border-t-4 border-blue-600">
                            <div class="text-4xl mb-3">"⛵"</div>
                            <h3 class="text-xl font-bold text-gray-900 mb-2">
                                {move || t("landing.marine_title")}
                            </h3>
                            <p class="text-gray-600 text-sm">{move || t("landing.marine_desc")}</p>
                            <div class="mt-4 text-blue-600 font-semibold text-sm">
                                "→ Launch Simulator"
                            </div>
                        </div>
                    </A> {} <A href="/wheelchair-emergency">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition duration-200 cursor-pointer h-full border-t-4 border-purple-600">
                            <div class="text-4xl mb-3">"♿"</div>
                            <h3 class="text-xl font-bold text-gray-900 mb-2">
                                {move || t("landing.mobility_title")}
                            </h3>
                            <p class="text-gray-600 text-sm">
                                {move || t("landing.mobility_desc")}
                            </p>
                            <div class="mt-4 text-purple-600 font-semibold text-sm">
                                "→ Launch Simulator"
                            </div>
                        </div>
                    </A> {} <A href="/lift-emergency">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl transition duration-200 cursor-pointer h-full border-t-4 border-yellow-600">
                            <div class="text-4xl mb-3">"🛗"</div>
                            <h3 class="text-xl font-bold text-gray-900 mb-2">
                                {move || t("landing.lift_title")}
                            </h3>
                            <p class="text-gray-600 text-sm">{move || t("landing.lift_desc")}</p>
                            <div class="mt-4 text-yellow-600 font-semibold text-sm">
                                "→ Launch Simulator"
                            </div>
                        </div>
                    </A>
                </div>
            </div> {} <div class="bg-white py-16 border-y">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                        {move || t("landing.how_it_works")}
                    </h2>
                    <div class="grid md:grid-cols-4 gap-8">
                        <div class="text-center">
                            <div class="bg-red-100 rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-4 text-2xl font-bold text-red-600">
                                "1"
                            </div>
                            <h3 class="text-lg font-bold text-gray-900 mb-2">
                                {move || t("landing.step_1_select")}
                            </h3>
                            <p class="text-gray-600 text-sm">{move || t("landing.step_1_desc")}</p>
                        </div>
                        <div class="text-center">
                            <div class="bg-blue-100 rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-4 text-2xl font-bold text-blue-600">
                                "2"
                            </div>
                            <h3 class="text-lg font-bold text-gray-900 mb-2">
                                {move || t("landing.step_2_enter")}
                            </h3>
                            <p class="text-gray-600 text-sm">{move || t("landing.step_2_desc")}</p>
                        </div>
                        <div class="text-center">
                            <div class="bg-green-100 rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-4 text-2xl font-bold text-green-600">
                                "3"
                            </div>
                            <h3 class="text-lg font-bold text-gray-900 mb-2">
                                {move || t("landing.step_3_review")}
                            </h3>
                            <p class="text-gray-600 text-sm">{move || t("landing.step_3_desc")}</p>
                        </div>
                        <div class="text-center">
                            <div class="bg-purple-100 rounded-full w-16 h-16 flex items-center justify-center mx-auto mb-4 text-2xl font-bold text-purple-600">
                                "4"
                            </div>
                            <h3 class="text-lg font-bold text-gray-900 mb-2">
                                {move || t("landing.step_4_submit")}
                            </h3>
                            <p class="text-gray-600 text-sm">{move || t("landing.step_4_desc")}</p>
                        </div>
                    </div>
                </div>
            </div> {} <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-16">
                <h2 class="text-4xl font-bold text-center text-gray-900 mb-12">
                    {move || t("landing.tech_title")}
                </h2>
                <div class="grid md:grid-cols-3 gap-8">
                    <div class="bg-gray-50 rounded-lg p-6">
                        <h3 class="text-xl font-bold text-gray-900 mb-3">
                            {move || t("landing.tech_framework")}
                        </h3>
                        <p class="text-gray-700">{move || t("landing.tech_framework_desc")}</p>
                    </div>
                    <div class="bg-gray-50 rounded-lg p-6">
                        <h3 class="text-xl font-bold text-gray-900 mb-3">
                            {move || t("landing.tech_geolocation")}
                        </h3>
                        <p class="text-gray-700">{move || t("landing.tech_geolocation_desc")}</p>
                    </div>
                    <div class="bg-gray-50 rounded-lg p-6">
                        <h3 class="text-xl font-bold text-gray-900 mb-3">
                            {move || t("landing.tech_standards")}
                        </h3>
                        <p class="text-gray-700">{move || t("landing.tech_standards_desc")}</p>
                    </div>
                </div>
            </div> {} <div class="bg-gradient-to-r from-red-600 to-red-700 py-16">
                <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h2 class="text-4xl font-bold text-white mb-6">
                        {move || t("landing.cta_title")}
                    </h2>
                    <p class="text-xl text-red-100 mb-8">{move || t("landing.cta_subtitle")}</p>
                    <A href="/simulators">
                        <div class="inline-block px-10 py-4 bg-white text-red-600 font-bold rounded-lg hover:bg-gray-100 transition duration-200 cursor-pointer text-lg">
                            {move || t("landing.cta_button")}
                        </div>
                    </A>
                </div>
            </div>
        </div>
    }
}

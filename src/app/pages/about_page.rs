use crate::app::pages::i18n::{Language, Translations};
use leptos::prelude::*;
use leptos::{view, IntoView};

#[component]
pub fn AboutPage(language: RwSignal<Language>) -> impl IntoView {
    let t = move |key: &str| Translations::new(language.get()).t(key);
    view! {
        <div class="min-h-screen bg-gradient-to-br from-red-50 via-white to-blue-50 py-16 px-4">
            <div class="max-w-3xl mx-auto bg-white rounded-lg shadow-lg p-10 border-l-4 border-red-600">
                <h1 class="text-4xl font-bold text-gray-900 mb-6">{move || t("about.title")}</h1>
                <p class="text-lg text-gray-700 mb-8">{move || t("about.intro")}</p>
                <div class="mb-8">
                    <h2 class="text-2xl font-bold text-gray-900 mb-3">
                        {move || t("about.key_features")}
                    </h2>
                    <ul class="text-gray-700 space-y-3">
                        <li class="flex items-start">
                            <span class="text-green-500 font-bold mr-3">v</span>
                            <span>{move || t("about.feature1")}</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 font-bold mr-3">v</span>
                            <span>{move || t("about.feature2")}</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 font-bold mr-3">v</span>
                            <span>{move || t("about.feature3")}</span>
                        </li>
                        <li class="flex items-start">
                            <span class="text-green-500 font-bold mr-3">v</span>
                            <span>{move || t("about.feature4")}</span>
                        </li>
                    </ul>
                </div>
                <div class="mb-8">
                    <h2 class="text-2xl font-bold text-gray-900 mb-3">
                        {move || t("about.how_it_works")}
                    </h2>
                    <ol class="list-decimal list-inside text-gray-700 space-y-2">
                        <li>
                            <strong>{move || t("about.step1_title")}</strong>
                            " "
                            {move || t("about.step1_desc")}
                        </li>
                        <li>
                            <strong>{move || t("about.step2_title")}</strong>
                            " "
                            {move || t("about.step2_desc")}
                        </li>
                        <li>
                            <strong>{move || t("about.step3_title")}</strong>
                            " "
                            {move || t("about.step3_desc")}
                        </li>
                        <li>
                            <strong>{move || t("about.step4_title")}</strong>
                            " "
                            {move || t("about.step4_desc")}
                        </li>
                    </ol>
                </div>
                <div class="mb-8">
                    <h2 class="text-2xl font-bold text-gray-900 mb-3">
                        {move || t("about.technology")}
                    </h2>
                    <ul class="text-gray-700 space-y-2">
                        <li>
                            <strong>{move || t("about.tech_leptos")}</strong>
                            " "
                            {move || t("about.tech_leptos_desc")}
                        </li>
                        <li>
                            <strong>{move || t("about.tech_geolocation")}</strong>
                            " "
                            {move || t("about.tech_geolocation_desc")}
                        </li>
                        <li>
                            <strong>{move || t("about.tech_standards")}</strong>
                            " "
                            {move || t("about.tech_standards_desc")}
                        </li>
                    </ul>
                </div>
                <div class="mb-8">
                    <h2 class="text-2xl font-bold text-gray-900 mb-3">
                        {move || t("about.goals")}
                    </h2>
                    <ul class="text-gray-700 space-y-2">
                        <li>{move || t("about.goal1")}</li>
                        <li>{move || t("about.goal2")}</li>
                        <li>{move || t("about.goal3")}</li>
                    </ul>
                </div>
                <div class="mt-10 text-center">
                    <a
                        href="/simulators"
                        class="inline-block px-8 py-3 bg-red-600 hover:bg-red-700 text-white font-bold rounded-lg transition duration-200 cursor-pointer text-lg"
                    >
                        {move || t("about.explore_btn")}
                    </a>
                </div>
            </div>
        </div>
    }
}

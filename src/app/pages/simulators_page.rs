use crate::app::pages::i18n::{Language, Translations};
use leptos::prelude::*;
use leptos::{view, IntoView};
use leptos_router::components::A;

#[component]
pub fn SimulatorsPage(language: RwSignal<Language>) -> impl IntoView {
    let t = move |key: &str| Translations::new(language.get()).t(key);
    view! {
        <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-6xl mx-auto">
                <div class="mb-12">
                    <h1 class="text-5xl font-bold text-gray-900 mb-4">
                        {move || t("simulators.title")}
                    </h1>
                    <p class="text-xl text-gray-600">{move || t("simulators.subtitle")}</p>
                </div>

                {}
                <div class="grid md:grid-cols-2 lg:grid-cols-4 gap-6 mb-12">
                    {} <A href="/ecall">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl hover:scale-105 transition duration-200 cursor-pointer h-full border-t-4 border-red-600">
                            <div class="text-5xl mb-4">"🚗"</div>
                            <h2 class="text-2xl font-bold text-gray-900 mb-3">
                                {move || t("simulators.auto_ecall_title")}
                            </h2>
                            <p class="text-gray-600 mb-4">
                                {move || t("simulators.auto_ecall_desc")}
                            </p>
                            <ul class="text-sm text-gray-700 space-y-2 mb-4">
                                <li class="flex items-center">
                                    <span class="text-red-500 mr-2">-</span>
                                    {move || t("simulators.auto_ecall_feature1")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-red-500 mr-2">-</span>
                                    {move || t("simulators.auto_ecall_feature2")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-red-500 mr-2">-</span>
                                    {move || t("simulators.auto_ecall_feature3")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-red-500 mr-2">-</span>
                                    {move || t("simulators.auto_ecall_feature4")}
                                </li>
                            </ul>
                            <div class="bg-red-50 text-red-600 font-semibold py-2 px-3 rounded text-center">
                                {move || t("simulators.launch")}
                            </div>
                        </div>
                    </A> {} <A href="/boat-emergency">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl hover:scale-105 transition duration-200 cursor-pointer h-full border-t-4 border-blue-600">
                            <div class="text-5xl mb-4">"⛵"</div>
                            <h2 class="text-2xl font-bold text-gray-900 mb-3">
                                {move || t("simulators.marine_title")}
                            </h2>
                            <p class="text-gray-600 mb-4">{move || t("simulators.marine_desc")}</p>
                            <ul class="text-sm text-gray-700 space-y-2 mb-4">
                                <li class="flex items-center">
                                    <span class="text-blue-500 mr-2">-</span>
                                    {move || t("simulators.marine_feature1")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-blue-500 mr-2">-</span>
                                    {move || t("simulators.marine_feature2")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-blue-500 mr-2">-</span>
                                    {move || t("simulators.marine_feature3")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-blue-500 mr-2">-</span>
                                    {move || t("simulators.marine_feature4")}
                                </li>
                            </ul>
                            <div class="bg-blue-50 text-blue-600 font-semibold py-2 px-3 rounded text-center">
                                {move || t("simulators.launch")}
                            </div>
                        </div>
                    </A> {} <A href="/wheelchair-emergency">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl hover:scale-105 transition duration-200 cursor-pointer h-full border-t-4 border-purple-600">
                            <div class="text-5xl mb-4">"♿"</div>
                            <h2 class="text-2xl font-bold text-gray-900 mb-3">
                                {move || t("simulators.mobility_title")}
                            </h2>
                            <p class="text-gray-600 mb-4">
                                {move || t("simulators.mobility_desc")}
                            </p>
                            <ul class="text-sm text-gray-700 space-y-2 mb-4">
                                <li class="flex items-center">
                                    <span class="text-purple-500 mr-2">-</span>
                                    {move || t("simulators.mobility_feature1")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-purple-500 mr-2">-</span>
                                    {move || t("simulators.mobility_feature2")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-purple-500 mr-2">-</span>
                                    {move || t("simulators.mobility_feature3")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-purple-500 mr-2">-</span>
                                    {move || t("simulators.mobility_feature4")}
                                </li>
                            </ul>
                            <div class="bg-purple-50 text-purple-600 font-semibold py-2 px-3 rounded text-center">
                                {move || t("simulators.launch")}
                            </div>
                        </div>
                    </A> {} <A href="/lift-emergency">
                        <div class="bg-white rounded-lg shadow-lg p-6 hover:shadow-xl hover:scale-105 transition duration-200 cursor-pointer h-full border-t-4 border-yellow-600">
                            <div class="text-5xl mb-4">"🛗"</div>
                            <h2 class="text-2xl font-bold text-gray-900 mb-3">
                                {move || t("simulators.lift_title")}
                            </h2>
                            <p class="text-gray-600 mb-4">{move || t("simulators.lift_desc")}</p>
                            <ul class="text-sm text-gray-700 space-y-2 mb-4">
                                <li class="flex items-center">
                                    <span class="text-yellow-600 mr-2">-</span>
                                    {move || t("simulators.lift_feature1")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-yellow-600 mr-2">-</span>
                                    {move || t("simulators.lift_feature2")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-yellow-600 mr-2">-</span>
                                    {move || t("simulators.lift_feature3")}
                                </li>
                                <li class="flex items-center">
                                    <span class="text-yellow-600 mr-2">-</span>
                                    {move || t("simulators.lift_feature4")}
                                </li>
                            </ul>
                            <div class="bg-yellow-50 text-yellow-600 font-semibold py-2 px-3 rounded text-center">
                                {move || t("simulators.launch")}
                            </div>
                        </div>
                    </A>
                </div>

                {}
                <div class="grid md:grid-cols-2 gap-6 mt-16">
                    <div class="bg-white rounded-lg shadow p-8">
                        <h3 class="text-2xl font-bold text-gray-900 mb-4">
                            {move || t("simulators.common_features")}
                        </h3>
                        <ul class="space-y-3 text-gray-700">
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3 text-lg">v</span>
                                <div>
                                    <strong>{move || t("simulators.feature_realistic")}</strong>
                                    " "
                                    {move || t("simulators.feature_realistic_desc")}
                                </div>
                            </li>
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3 text-lg">v</span>
                                <div>
                                    <strong>{move || t("simulators.feature_auto_geo")}</strong>
                                    " "
                                    {move || t("simulators.feature_auto_geo_desc")}
                                </div>
                            </li>
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3 text-lg">v</span>
                                <div>
                                    <strong>{move || t("simulators.feature_prefill")}</strong>
                                    " "
                                    {move || t("simulators.feature_prefill_desc")}
                                </div>
                            </li>
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3 text-lg">v</span>
                                <div>
                                    <strong>{move || t("simulators.feature_manual")}</strong>
                                    " "
                                    {move || t("simulators.feature_manual_desc")}
                                </div>
                            </li>
                            <li class="flex items-start">
                                <span class="text-green-500 font-bold mr-3 text-lg">v</span>
                                <div>
                                    <strong>{move || t("simulators.feature_summary")}</strong>
                                    " "
                                    {move || t("simulators.feature_summary_desc")}
                                </div>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-white rounded-lg shadow p-8">
                        <h3 class="text-2xl font-bold text-gray-900 mb-4">
                            {move || t("simulators.getting_started")}
                        </h3>
                        <ol class="space-y-4 text-gray-700">
                            <li class="flex">
                                <span class="text-blue-600 font-bold mr-4 text-lg">1.</span>
                                <div>
                                    <strong>{move || t("simulators.step1_title")}</strong>
                                    " "
                                    {move || t("simulators.step1_desc")}
                                </div>
                            </li>
                            <li class="flex">
                                <span class="text-blue-600 font-bold mr-4 text-lg">2.</span>
                                <div>
                                    <strong>{move || t("simulators.step2_title")}</strong>
                                    " "
                                    {move || t("simulators.step2_desc")}
                                </div>
                            </li>
                            <li class="flex">
                                <span class="text-blue-600 font-bold mr-4 text-lg">3.</span>
                                <div>
                                    <strong>{move || t("simulators.step3_title")}</strong>
                                    " "
                                    {move || t("simulators.step3_desc")}
                                </div>
                            </li>
                            <li class="flex">
                                <span class="text-blue-600 font-bold mr-4 text-lg">4.</span>
                                <div>
                                    <strong>{move || t("simulators.step4_title")}</strong>
                                    " "
                                    {move || t("simulators.step4_desc")}
                                </div>
                            </li>
                        </ol>
                    </div>
                </div>

                {}
                <div class="mt-12 text-center">
                    <A href="/">
                        <div class="inline-block text-gray-600 hover:text-gray-900 font-semibold">
                            {move || t("simulators.back_home")}
                        </div>
                    </A>
                </div>
            </div>
        </div>
    }
}

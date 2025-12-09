use leptos::prelude::*;

#[component]
pub fn NavbarComponent() -> impl IntoView {
    view! {
        <nav class="sticky top-0 z-50 w-full border-b bg-card/95 backdrop-blur supports-[backdrop-filter]:bg-card/60">
            <div>
                <a class="container mx-auto px-4 h-16 flex items-center justify-between" href="/">"Home"</a>
            </div>
            <div class="hidden md:flex items-center gap-4">
                <a class="font-medium" href="/find">"Events"</a>
                <a class="" href="/create">"Create Event"</a>
                <a class="" href="/about">"About"</a>
            </div>
        </nav>
    }
}

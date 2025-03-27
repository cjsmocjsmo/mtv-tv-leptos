#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVComedyPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvfubarseapage">
                <img src="http://10.0.4.41:7777/tvstatic/fubar.webp" alt="Comedy" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
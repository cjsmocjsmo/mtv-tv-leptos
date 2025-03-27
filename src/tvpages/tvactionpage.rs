#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVActionPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvcontinentalseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/continental.webp" alt="Continental" />
            </a>
            <a href="/tvshogunseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/shogun.webp" alt="Showgun" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
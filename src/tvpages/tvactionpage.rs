#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVActionPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvcontinentalseapage">
                <img src="http://10.0.4.41:7777/tvstatic/continental.webp" alt="Continental" />
            </a>
            <a href="/tvshogunseapage">
                <img src="http://10.0.4.41:7777/tvstatic/shogun.webp" alt="Showgun" />
            </a>
            <a href="/tvmoblandpage">
                <img src="http://10.0.4.41:7777/tvstatic/mobland.webp" alt="Mobland" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
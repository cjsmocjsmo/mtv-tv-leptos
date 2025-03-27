#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVMCUPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvsecretinvasionseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/secret_invasion.webp" alt="Secret Invation" />
            </a>
            <a href="/tviamgrootseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/iamgroot.webp" alt="I Am Groot" />
            </a>
            <a href="/tvlokiseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/loki.webp" alt="Loki" />
            </a>
            <a href="/tvmoonknightseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/moonknight.webp" alt="MoonKnight" />
            </a>
            <a href="/tvshehulkseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/shehulk.webp" alt="SheHulk" />
            </a>
            <a href="/tvhawkeyeseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/Hawkeye.webp" alt="Hawkeye" />
            </a>
            <a href="/tvfalconwintersoldierseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/falconwintersoldier.webp" alt="Falcon And The Winter Soldier" />
            </a>
            <a href="/tvwandavisionseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/wandavision.webp" alt="Wandavision" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
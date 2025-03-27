#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarWarsPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvskeletoncrewseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/skeletoncrew.webp" alt="skeleton crew" />
            </a>
            <a href="/tvahsokaseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/ahsoka.webp" alt="ahsoka" />
            </a>
            <a href="/tvandorseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/andor.webp" alt="andor" />
            </a>
            <a href="/tvbadbatchseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/badbatch.webp" alt="bad batch" />
            </a>
            <a href="/tvbobafettseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/bobafett.webp" alt="boba fett" />
            </a>
            <a href="/tvmandalorianseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/mandalorian.webp" alt="mandalorian" />
            </a>
            <a href="/tvtalesofthejediseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/talesofthejedi.webp" alt="tales of the jedi" />
            </a>
            <a href="/tvobiwanseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/obiwan.webp" alt="obiwan" />
            </a>
            <a href="/tvvisionsseapage">
                <img src="http://10.0.4.41:7777/tvthumbnails/visions.webp" alt="visions" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
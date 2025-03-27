#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarWarsPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvskeletoncrewseapage">
                <img src="http://10.0.4.41:7777/tvstatic/skeletoncrew.webp" alt="skeleton crew" />
            </a>
            <a href="/tvahsokaseapage">
                <img src="http://10.0.4.41:7777/tvstatic/ahsoka.webp" alt="ahsoka" />
            </a>
            <a href="/tvandorseapage">
                <img src="http://10.0.4.41:7777/tvstatic/andor.webp" alt="andor" />
            </a>
            <a href="/tvbadbatchseapage">
                <img src="http://10.0.4.41:7777/tvstatic/badbatch.webp" alt="bad batch" />
            </a>
            <a href="/tvbobafettseapage">
                <img src="http://10.0.4.41:7777/tvstatic/bobafett.webp" alt="boba fett" />
            </a>
            <a href="/tvmandalorianseapage">
                <img src="http://10.0.4.41:7777/tvstatic/mandalorian.webp" alt="mandalorian" />
            </a>
            <a href="/tvtalesofthejediseapage">
                <img src="http://10.0.4.41:7777/tvstatic/talesofthejedi.webp" alt="tales of the jedi" />
            </a>
            <a href="/tvobiwanseapage">
                <img src="http://10.0.4.41:7777/tvstatic/obiwan.webp" alt="obiwan" />
            </a>
            <a href="/tvvisionsseapage">
                <img src="http://10.0.4.41:7777/tvstatic/visions.webp" alt="visions" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
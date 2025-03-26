#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarWarsPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvskeletoncrewseapage">
                <img src="http://10.0.4.41:9095/skeletoncrew.webp" alt="skeleton crew" />
            </a>
            <a href="/tvahsokaseapage">
                <img src="http://10.0.4.41:9095/ahsoka.webp" alt="ahsoka" />
            </a>
            <a href="/tvandorseapage">
                <img src="http://10.0.4.41:9095/andor.webp" alt="andor" />
            </a>
            <a href="/tvbadbatchseapage">
                <img src="http://10.0.4.41:9095/badbatch.webp" alt="bad batch" />
            </a>
            <a href="/tvbobafettseapage">
                <img src="http://10.0.4.41:9095/bobafett.webp" alt="boba fett" />
            </a>
            <a href="/tvmandalorianseapage">
                <img src="http://10.0.4.41:9095/mandalorian.webp" alt="mandalorian" />
            </a>
            <a href="/tvtalesofthejediseapage">
                <img src="http://10.0.4.41:9095/talesofthejedi.webp" alt="tales of the jedi" />
            </a>
            <a href="/tvobiwanseapage">
                <img src="http://10.0.4.41:9095/obiwan.webp" alt="obiwan" />
            </a>
            <a href="/tvvisionsseapage">
                <img src="http://10.0.4.41:9095/visions.webp" alt="visions" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
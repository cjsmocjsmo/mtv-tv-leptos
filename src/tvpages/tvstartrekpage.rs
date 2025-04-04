#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVStarTrekPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvvoyagerseapage">
                <img src="http://10.0.4.41:7777/tvstatic/voyager.webp" alt="voyager" />
            </a>
            <a href="/tvsttvseapage">
                <img src="http://10.0.4.41:7777/tvstatic/sttv.webp" alt="STTV" />
            </a>
            <a href="/tventerpriseseapage">
                <img src="http://10.0.4.41:7777/tvstatic/enterprise.webp" alt="enterprise" />
            </a>
            <a href="/tvnexgenseapage">
                <img src="http://10.0.4.41:7777/tvstatic/nextgen.webp" alt="next generation" />
            </a>
            <a href="tvdiscoveryseapage">
                <img src="http://10.0.4.41:7777/tvstatic/discovery.webp" alt="discovery" />
            </a>
            <a href="/tvpicardseapage">
                <img src="http://10.0.4.41:7777/tvstatic/picard.webp" alt="picard" />
            </a>
            <a href="/tvlowerdecksseapage">
                <img src="http://10.0.4.41:7777/tvstatic/lowerdecks.webp" alt="lower decks" />
            </a>
            <a href="/tvprodigyseapage">
                <img src="http://10.0.4.41:7777/tvstatic/prodigy.webp" alt="prodigy" />
            </a>
            <a href="tvstrangenewworldsseapage">
                <img src="http://10.0.4.41:7777/tvstatic/strangenewworlds.webp" alt="strange new worlds" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
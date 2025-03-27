#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVFantasyPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvhouseofthedragonseapage">
                <img src="http://10.0.4.41:7777/tvstatic/houseofthedragon.webp" alt="House of the Dragon" />
            </a>
            <a href="/tvthelordoftheringsringsofpowerseapage">
                <img src="http://10.0.4.41:7777/tvstatic/thelordoftheringsringsofpower.webp" alt="The Rings of Power" />
            </a>
            <a href="/tvwheeloftimeseapage">
                <img src="http://10.0.4.41:7777/tvstatic/wheeloftime.webp" alt="The Wheel Of Time" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
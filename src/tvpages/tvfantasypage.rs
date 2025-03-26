#![allow(non_snake_case)]
use leptos::prelude::*;

#[component]
pub fn TVFantasyPage() -> impl IntoView {
    view! {
        <div class="tvAss">
            <a href="/tvhouseofthedragonseapage">
                <img src="http://10.0.4.41:9095/houseofthedragon.webp" alt="House of the Dragon" />
            </a>
            <a href="/tvthelordoftheringsringsofpowerseapage">
                <img src="http://10.0.4.41:9095/thelordoftheringsringsofpower.webp" alt="The Rings of Power" />
            </a>
            <a href="/tvwheeloftimeseapage">
                <img src="http://10.0.4.41:9095/wheeloftime.webp" alt="The Wheel Of Time" />
            </a>
        </div>
        <span class="spacerSpan"></span>
    }
}
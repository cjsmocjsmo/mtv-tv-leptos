use leptos::prelude::*;

#[component]
pub fn TVShowsListPage() -> impl IntoView {
    view! {
        <div class="tvshowsSection">
            <div class="tvshowsSectionDiv">
                <a class="tvshowsSectionDivItem" href="/tvactionpage">"Action"</a>
                <a class="tvshowsSectionDivItem" href="/tvcomedypage">"Comedy"</a>
                <a class="tvshowsSectionDivItem" href="/tvfantasypage">"Fantasy"</a>
                <a class="tvshowsSectionDivItem" href="/tvmcupage">"MCU"</a>
                <a class="tvshowsSectionDivItem" href="/tvscience">"Science"</a>
                <a class="tvshowsSectionDivItem" href="/tvscifi">"SciFi"</a>
                <a class="tvshowsSectionDivItem" href="/tvstartrek">"Star Trek"</a>
                <a class="tvshowsSectionDivItem" href="/tvstarwars">"Star Wars"</a>
                <a class="tvshowsSectionDivItem" href="/tvwesterns">"Westerns"</a>
            </div>
            <span class="spacerSpan"></span>
        </div>
    }
}

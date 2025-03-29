use leptos::ev::SubmitEvent;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::prelude::*;
// use leptos::html::HtmlElement;
use leptos::web_sys;
use reqwest::Error;
use web_sys::HtmlInputElement;

async fn fetch_results(search_query: &str) -> Result<Vec<String>, Error> {
    // fetch search results from the server
    let response = reqwest::get(&format!("http://10.0.4.41:7777/tvsearch/{}", search_query)).await?;
    let results = response.json::<Vec<String>>().await?;
    Ok(results)
}

#[component]
pub fn SearchPage() -> impl IntoView {
    let search_query = signal(String::new());

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let input: HtmlInputElement = event.target().unwrap().dyn_into().expect("Failed to cast to HtmlInputElement");
        let value = input.value();
        search_query.1.set(value.clone());

        spawn_local(handle_search(value));
    };

    async fn handle_search(value: String) {
        let results = fetch_results(&value).await.unwrap();
        update_search_results(results);
    }

    fn update_search_results(results: Vec<String>) {
        if let Some(search_results) = document().query_selector(".searchResults").unwrap() {
            search_results.set_inner_html("");
            for result in results {
                if let Ok(search_result_div) = document().create_element("div") {
                    search_result_div.set_class_name("searchResultDiv");
                    if let Ok(img) = document().create_element("img") {
                        if img.set_attribute("src", &result).is_ok() && img.set_attribute("alt", &result).is_ok(){
                            if search_result_div.append_child(&img).is_ok(){
                                search_results.append_child(&search_result_div).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    view! {
        <div class="searchDiv">
            <div class="searchInnerDiv">
                <form action="" on:submit={on_submit}>
                    <input class="searchInput" type="text" placeholder="TVShow..." />
                    <input class="searchInput" type="text" placeholder="Season..." />
                    <button class="searchButton" type="submit" >Submit</button>
                </form>
            </div>
            <div class="searchResults">
                <div class="searchResultDiv">
                    <img src="https://apod.nasa.gov/apod/image/2503/LunarEclipseColors_Jin_960.jpg" alt="suppose to be a pic" />
                </div>
            </div>
            <span class="spacerSpan"></span>
        </div>
    }
}
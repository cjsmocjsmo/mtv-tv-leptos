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
        let search_results = document().query_selector(".searchResults").unwrap().unwrap();
        search_results.set_inner_html("");
        for result in results {
            let search_result_div = document().create_element("div").unwrap();
            search_result_div.set_class_name("searchResultDiv");
            let img = document().create_element("img").unwrap();
            img.set_attribute("src", &result).unwrap();
            img.set_attribute("alt", &result).unwrap();
            search_result_div.append_child(&img).unwrap();
            let span = document().create_element("span").unwrap();
            span.set_text_content(Some(&result));
            search_result_div.append_child(&span).unwrap();
            search_results.append_child(&search_result_div).unwrap();
        }
    }

    view! {
        <div class="searchDiv">
            <div class="searchInnerDiv">
                <form action="" on:submit={on_submit}>
                    <input class="searchInput" type="text" placeholder="Search..." />
                    <button class="search-button" type="submit" >Submit</button>
                </form>
            </div>
            <span class="spacerSpan"></span>
            <span class="spacerSpan"></span>
            <div class="searchResults">
                <div class="searchResultDiv">
                    <img src="https://via.placeholder.com/150" alt="suppose to be a pic" />
                    <span>Movie Title</span>
                </div>
            </div>
        </div>
    }
}
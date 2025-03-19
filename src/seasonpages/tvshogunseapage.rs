#![allow(non_snake_case)]
use leptos::{prelude::*, task::spawn_local};
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct Episode {
    TvId: String,
    Episode: String,
}

#[component]
pub fn TVShogunSeaPage() -> impl IntoView {
    let (episodes, set_episodes) = signal(Vec::new());

    spawn_local(async move {
        match fetch_episodes().await {
            Ok(mut data) => {
                data.sort_by(|a, b| a.Episode.cmp(&b.Episode));
                log::info!("Fetched and sorted episodes data: {:?}", data); // Debugging log
                set_episodes.set(data);
            },
            Err(err) => log::error!("Error fetching episodes data: {:?}", err),
        }
    });

    view! {
        <div class="seaMainDiv">
            <h1 class="seaH1">Shogun</h1>
            <div class="seaInnerDiv">
                <h3 class="seaH3">Season 1</h3>
                <div class="seaBtnGrp">
                {
                    let episodes_list = move || episodes.get().clone(); move || episodes_list().iter().map(|episode| {
                        let tv_id = episode.TvId.clone();
                        let episode_clone = episode.Episode.clone();
                        view! {
                            <button class="seaBtn" on:click=move |_| {
                                let tv_id = tv_id.clone();
                                spawn_local(async move {
                                    if let Err(err) = send_get_request(&tv_id).await {
                                        log::error!("Error sending GET request: {:?}", err);
                                    }
                                });
                            }>{episode_clone}</button>
                        }
                    }).collect_view()
                }
                </div>
            </div>
        </div>
    }
}

async fn fetch_episodes() -> Result<Vec<Episode>, Error> {
    let response = reqwest::get("http://10.0.4.41:7777/shogun").await?;
    let episodes: Vec<Episode> = response.json().await?;
    Ok(episodes)
}

async fn send_get_request(tv_id: &str) -> Result<(), Error> {
    let url = format!("http://10.0.4.41:7777/set_player_tv_id/{}", tv_id);
    reqwest::get(&url).await?;
    Ok(())
}
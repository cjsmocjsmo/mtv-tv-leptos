use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos::task::spawn_local;
use leptos_router::{components::*, path};



mod tvshowspage;
use crate::tvshowspage::TVShowsListPage;

mod tvpages {
    pub mod tvactionpage;
    pub mod tvcomedypage;
    pub mod tvfantasypage;
    pub mod tvmcupage;
    pub mod tvsciencepage;
    pub mod tvscifipage;
    pub mod tvstartrekpage;
    pub mod tvstarwarspage;
    pub mod tvwesternspage;
}

use crate::tvpages::tvactionpage::TVActionPage;
use crate::tvpages::tvcomedypage::TVComedyPage;
use crate::tvpages::tvfantasypage::TVFantasyPage;
use crate::tvpages::tvmcupage::TVMCUPage;
use crate::tvpages::tvsciencepage::TVSciencePage;
use crate::tvpages::tvscifipage::TVSciFiPage;
use crate::tvpages::tvstartrekpage::TVStarTrekPage;
use crate::tvpages::tvstarwarspage::TVStarWarsPage;
use crate::tvpages::tvwesternspage::TVWesternsPage;

mod seasonpages {
    pub mod tv1923seapage;
    pub mod tvahsokaseapage;
    pub mod tvalteredcarbonseapage;
    pub mod tvandorseapage;
    pub mod tvbadbatchseapage;
    pub mod tvbobafettseapage;
    pub mod tvcontinentalseapage;
    pub mod tvcowboybebopseapage;
    pub mod tvdiscoveryseapage;
    pub mod tventerpriseseapage;
    pub mod tvfalconwintersoldierseapage;
    pub mod tvfalloutseapage;
    pub mod tvfubarseapage;
    pub mod tvforallmankindseapage;
    pub mod tvfoundationseapage;
    pub mod tvhaloseapage;
    pub mod tvhawkeyeseapage;
    pub mod tvhouseofthedragonseapage;
    pub mod tviamgrootseapage;
    pub mod tvlastofusseapage;
    pub mod tvlokiseapage;
    pub mod tvlostinspaceseapage;
    pub mod tvlowerdecksseapage;
    pub mod tvmandalorianseapage;
    pub mod tvmonarchlegacyofmonstersseapage;
    pub mod tvmoonknightseapage;
    pub mod tvnexgenseapage;
    pub mod tvnightskyseapage;
    pub mod tvobiwanseapage;
    pub mod tvorvilleseapage;
    pub mod tvpicardseapage;
    pub mod tvprehistoricplanetseapage;
    pub mod tvprodigyseapage;
    pub mod tvraisedbywolvesseapage;
    pub mod tvsecretinvasionseapage;
    pub mod tvshehulkseapage;
    pub mod tvshogunseapage;
    pub mod tvsiloseapage;
    pub mod tvskeletoncrewseapage;
    pub mod tvstrangenewworldsseapage;
    pub mod tvsttvseapage;
    pub mod tvtalesofthejediseapage;
    pub mod tvthelordoftheringsringsofpowerseapage;
    pub mod tvvisionsseapage;
    pub mod tvvoyagerseapage;
    pub mod tvwandavisionseapage;
    pub mod tvwheeloftimeseapage; 
}

use crate::seasonpages::tv1923seapage::TV1923SeaPage;
use crate::seasonpages::tvahsokaseapage::TVAhsokaSeaPage;
use crate::seasonpages::tvalteredcarbonseapage::TVAlteredCarbonSeaPage;
use crate::seasonpages::tvandorseapage::TVAndorSeaPage;
use crate::seasonpages::tvbadbatchseapage::TVBadBatchSeaPage;
use crate::seasonpages::tvbobafettseapage::TVBobaFettSeaPage;
use crate::seasonpages::tvcontinentalseapage::TVContinentalSeaPage;
use crate::seasonpages::tvcowboybebopseapage::TVCowboyBebopSeaPage;
use crate::seasonpages::tvdiscoveryseapage::TVDiscoverySeaPage;
use crate::seasonpages::tventerpriseseapage::TVEnterpriseSeaPage;
use crate::seasonpages::tvfalconwintersoldierseapage::TVFalconWinterSoldierSeaPage;
use crate::seasonpages::tvfalloutseapage::TVFallOutSeaPage;
use crate::seasonpages::tvfubarseapage::TVFubarSeaPage;
use crate::seasonpages::tvforallmankindseapage::TVForAllManKindSeaPage;
use crate::seasonpages::tvfoundationseapage::TVFoundationSeaPage;
use crate::seasonpages::tvhaloseapage::TVHaloSeaPage;
use crate::seasonpages::tvhawkeyeseapage::TVHawkeyeSeaPage;
use crate::seasonpages::tvhouseofthedragonseapage::TVHouseOfTheDragonSeaPage;
use crate::seasonpages::tviamgrootseapage::TVIAmGrootSeaPage;
use crate::seasonpages::tvlastofusseapage::TVLastOfUsSeaPage;
use crate::seasonpages::tvlokiseapage::TVLokiSeaPage;
use crate::seasonpages::tvlostinspaceseapage::TVLostInSpaceSeaPage;
use crate::seasonpages::tvlowerdecksseapage::TVLowerDecksSeaPage;
use crate::seasonpages::tvmandalorianseapage::TVMandalorianSeaPage;
use crate::seasonpages::tvmonarchlegacyofmonstersseapage::TVMonarchLegacyOfMonstersSeaPage;
use crate::seasonpages::tvmoonknightseapage::TVMoonKnightSeaPage;
use crate::seasonpages::tvnexgenseapage::TVNexGenSeaPage;
use crate::seasonpages::tvnightskyseapage::TVNightSkySeaPage;
use crate::seasonpages::tvobiwanseapage::TVObiWanSeaPage;
use crate::seasonpages::tvorvilleseapage::TVOrvilleSeaPage;
use crate::seasonpages::tvpicardseapage::TVPicardSeaPage;
use crate::seasonpages::tvprehistoricplanetseapage::TVPreHistoricPlanetSeaPage;
use crate::seasonpages::tvprodigyseapage::TVProdigySeaPage;
use crate::seasonpages::tvraisedbywolvesseapage::TVRaisedByWolvesSeaPage;
use crate::seasonpages::tvsecretinvasionseapage::TVSecretInvasionSeaPage;
use crate::seasonpages::tvshehulkseapage::TVSheHulkSeaPage;
use crate::seasonpages::tvshogunseapage::TVShogunSeaPage;
use crate::seasonpages::tvsiloseapage::TVSiloSeaPage;
use crate::seasonpages::tvskeletoncrewseapage::TVSkeletonCrewSeaPage;
use crate::seasonpages::tvstrangenewworldsseapage::TVStrangeNewWorldsSeaPage;
use crate::seasonpages::tvsttvseapage::TVSTTVSeaPage;
use crate::seasonpages::tvtalesofthejediseapage::TVTalesOfTheJediSeaPage;
use crate::seasonpages::tvthelordoftheringsringsofpowerseapage::TVTheLordOfTheRingsRingsOfPowerSeaPagePage;
use crate::seasonpages::tvvisionsseapage::TVVisionsSeaPage;
use crate::seasonpages::tvvoyagerseapage::TVVoyagerSeaPage;
use crate::seasonpages::tvwandavisionseapage::TVWandaVisionSeaPage;
use crate::seasonpages::tvwheeloftimeseapage::TVWheelOfTimeSeaPage;


fn main() {
	console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <NavBar />
            <Header />
            <main>
                <Routes fallback=|| "Not Found.">
                    <Route path=path!("/") view=tvshowspage::TVShowsPage />
                    <Route path=path!("/tvactionpage") view=TVActionPage />
                    <Route path=path!("/tvcomedypage") view=TVComedyPage />
                    <Route path=path!("/tvfantasypage") view=TVFantasyPage />
                    <Route path=path!("/tvmcupage") view=TVMCUPage />
                    <Route path=path!("/tvscience") view=TVSciencePage />
                    <Route path=path!("/tvscifi") view=TVSciFiPage />
                    <Route path=path!("/tvstartrek") view=TVStarTrekPage />
                    <Route path=path!("/tvstarwars") view=TVStarWarsPage />
                    <Route path=path!("/tvwesterns") view=TVWesternsPage />

                    <Route path=path!("/tv1923seapage") view=TV1923SeaPage />
                    <Route path=path!("/tvahsokaseapage") view=TVAhsokaSeaPage />
                    <Route path=path!("/tvalteredcarbonseapage") view=TVAlteredCarbonSeaPage />
                    <Route path=path!("/tvandorseapage") view=TVAndorSeaPage />
                    <Route path=path!("/tvbadbatchseapage") view=TVBadBatchSeaPage />
                    <Route path=path!("/tvbobafettseapage") view=TVBobaFettSeaPage />
                    <Route path=path!("/tvcontinentalseapage") view=TVContinentalSeaPage />
                    <Route path=path!("/tvcowboybebopseapage") view=TVCowboyBebopSeaPage />
                    <Route path=path!("/tvdiscoveryseapage") view=TVDiscoverySeaPage />
                    <Route path=path!("/tventerpriseseapage") view=TVEnterpriseSeaPage />
                    <Route path=path!("/tvfalconwintersoldierseapage") view=TVFalconWinterSoldierSeaPage />
                    <Route path=path!("/tvfalloutseapage") view=TVFallOutSeaPage />
                    <Route path=path!("/tvfubarseapage") view=TVFubarSeaPage />
                    <Route path=path!("/tvforallmankindseapage") view=TVForAllManKindSeaPage />
                    <Route path=path!("/tvfoundationseapage") view=TVFoundationSeaPage />
                    <Route path=path!("/tvhaloseapage") view=TVHaloSeaPage />
                    <Route path=path!("/tvhawkeyeseapage") view=TVHawkeyeSeaPage />
                    <Route path=path!("/tvhouseofthedragonseapage") view=TVHouseOfTheDragonSeaPage />
                    <Route path=path!("/tviamgrootseapage") view=TVIAmGrootSeaPage />
                    <Route path=path!("/tvlastofusseapage") view=TVLastOfUsSeaPage />
                    <Route path=path!("/tvlokiseapage") view=TVLokiSeaPage />
                    <Route path=path!("/tvlostinspaceseapage") view=TVLostInSpaceSeaPage />
                    <Route path=path!("/tvlowerdecksseapage") view=TVLowerDecksSeaPage />
                    <Route path=path!("/tvmandalorianseapage") view=TVMandalorianSeaPage />
                    <Route path=path!("/tvmonarchlegacyofmonstersseapage") view=TVMonarchLegacyOfMonstersSeaPage />
                    <Route path=path!("/tvmoonknightseapage") view=TVMoonKnightSeaPage />
                    <Route path=path!("/tvnexgenseapage") view=TVNexGenSeaPage />
                    <Route path=path!("/tvnightskyseapage") view=TVNightSkySeaPage />
                    <Route path=path!("/tvobiwanseapage") view=TVObiWanSeaPage />
                    <Route path=path!("/tvorvilleseapage") view=TVOrvilleSeaPage />
                    <Route path=path!("/tvpicardseapage") view=TVPicardSeaPage />
                    <Route path=path!("/tvprehistoricplanetseapage") view=TVPreHistoricPlanetSeaPage />
                    <Route path=path!("/tvprodigyseapage") view=TVProdigySeaPage />
                    <Route path=path!("/tvraisedbywolvesseapage") view=TVRaisedByWolvesSeaPage />
                    <Route path=path!("/tvsecretinvasionseapage") view=TVSecretInvasionSeaPage />
                    <Route path=path!("/tvshehulkseapage") view=TVSheHulkSeaPage />
                    <Route path=path!("/tvshogunseapage") view=TVShogunSeaPage />
                    <Route path=path!("/tvsiloseapage") view=TVSiloSeaPage />
                    <Route path=path!("/tvskeletoncrewseapage") view=TVSkeletonCrewSeaPage />
                    <Route path=path!("/tvstrangenewworldsseapage") view=TVStrangeNewWorldsSeaPage />
                    <Route path=path!("/tvsttvseapage") view=TVSTTVSeaPage />
                    <Route path=path!("/tvtalesofthejediseapage") view=TVTalesOfTheJediSeaPage />
                    <Route path=path!("/tvthelordoftheringsringsofpowerseapage") view=TVTheLordOfTheRingsRingsOfPowerSeaPagePage />
                    <Route path=path!("/tvvisionsseapage") view=TVVisionsSeaPage />
                    <Route path=path!("/tvvoyagerseapage") view=TVVoyagerSeaPage />
                    <Route path=path!("/tvwandavisionseapage") view=TVWandaVisionSeaPage />
                    <Route path=path!("/tvwheeloftimeseapage") view=TVWheelOfTimeSeaPage />
                </Routes>
                
            </main>
            // <span></span>
            <PlayerControls />
        </Router>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header>
            <h1 class="header">"MTV"</h1>
        </header>
    }
}

#[component]
fn NavBar() -> impl IntoView {
    view! {
        <nav>
            <a href="/" class="navItem">"Movies"</a>
            <a href="/tvshows" class="navItem">"TV Shows"</a>
            <a href="/search" class="navItem">"Search"</a>
        </nav>
    }
}

#[component]
fn PlayerControls() -> impl IntoView {
    view! {
        <footer class="playerControls">
            <button 
                class="playerButton"
                on:click=move |_| {
                    spawn_local(async move {
                        if let Err(err) = send_previous().await {
                            log::error!("Error sending GET request: {:?}", err);
                        }
                    });
                }
            >
                "Previous"
            </button>
            <button 
                class="playerButton"
                on:click=move |_| {
                    spawn_local(async move {
                        if let Err(err) = send_play().await {
                            log::error!("Error sending GET request: {:?}", err);
                        }
                    });
                }
            >
                "Play"
            </button>
            <button 
                class="playerButton"
                on:click=move |_| {
                    spawn_local(async move {
                        if let Err(err) = send_pause().await {
                            log::error!("Error sending GET request: {:?}", err);
                        }
                    });
                }
            >
                "Pause"
            </button>
            <button 
                class="playerButton"
                on:click=move |_| {
                    spawn_local(async move {
                        if let Err(err) = send_next().await {
                            log::error!("Error sending GET request: {:?}", err);
                        }
                    });
                }
            >
                "Next"
            </button>
        </footer>
    }
}

async fn send_previous() -> Result<(), Error> {
    let url = format!("http://10.0.4.41:7777/previous");
    reqwest::get(&url).await?;
    Ok(())
}

async fn send_play() -> Result<(), Error> {
    let url = format!("http://10.0.4.41:7777/play");
    reqwest::get(&url).await?;
    Ok(())
}

async fn send_pause() -> Result<(), Error> {
    let url = format!("http://10.0.4.41:7777/pause");
    reqwest::get(&url).await?;
    Ok(())
}

async fn send_next() -> Result<(), Error> {
    let url = format!("http://10.0.4.41:7777/next");
    reqwest::get(&url).await?;
    Ok(())
}

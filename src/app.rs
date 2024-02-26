use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/emmuoviti-app.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Poi {
    id: i32,
    name: String,
    category_id: i32,
    address_id: i32,
    latitude: f64,
    longitude: f64,
    slug: String,
    created_at: String,
    updated_at: String,
}

async fn fetch_pois(page: i32, per_page: i32) -> Result<Vec<Poi>, String> {
    let response = reqwest::get(format!("http://127.0.0.1:8080/pois?page={}", page)).await;
    match response {
        Ok(resp) => {
            match resp.json::<Vec<Poi>>().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Error while parsing pois {}", e))
            }
        }
        Err(e) => Err(format!("Error with API while fetching pois {}", e))
    }
}

#[component]
fn DisplayPois(pois: Vec<Poi>) -> impl IntoView {
    view! {
        <ul>
            {pois.iter().map(|poi| {
                view! {
                    <li>{&poi.name}</li>
                }
            }).collect_view()}
        </ul>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (page, set_page) = create_signal(0);
    let on_click = move |_| set_page.update(|page| *page += 1);

    let pois = create_local_resource(page, |p| async move {
        logging::log!("loading data from API");
        fetch_pois(p, 10).await
    } );

    let view_pois = move || {
        match pois.get() {
            Some(Ok(pois)) => {
                view! {
                    <DisplayPois pois=pois />
                }
            }
            Some(Err(e)) => {
                view! {
                    <DisplayPois pois=[].into() />
                }
            }
            None => {
                view! {
                    <DisplayPois pois=[].into() />
                }
            }
        }
    };


    view! {
        <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=" crossorigin="" />
        <link rel="stylesheet" src="https://unpkg.com/leaflet-draw@1.0.4/dist/leaflet.draw.css" crossorigin="" />
        <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=" crossorigin=""></script>
        <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=" crossorigin=""></script>
        <script src="https://unpkg.com/leaflet-draw@1.0.4/dist/leaflet.draw.js" crossorigin=""></script>

        <div id="map" style="height: 60vh"></div>

        {view_pois}

        <script>
            var osmBase = L.tileLayer("http://{s}.tile.osm.org/{z}/{x}/{y}.png");

            var map = L.map("map", {
                center: [45.275148,  9.113059],
                zoom: 13,
                layers: [osmBase],
                drawControl: true
            });

            var drawnItems = new L.FeatureGroup();
             map.addLayer(drawnItems);
             var drawControl = new L.Control.Draw({
                 edit: {
                     featureGroup: drawnItems
                 }
             });
             map.addControl(drawControl);
        </script>
        <button on:click=on_click>"Next: " {page}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

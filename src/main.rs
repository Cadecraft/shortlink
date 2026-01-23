use axum::{
    Router,
    extract::Path,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use reqwest;
use std::collections::hash_map::HashMap;
use std::env;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    assert_env();

    println!("Fetching initial data...");
    let data = fetch_data()
        .await
        .expect("Could not fetch initial data from DATA_URL");

    let state = AppState {
        data: Arc::new(Mutex::new(data)),
    };

    let app = Router::new()
        .route("/", get(get_root))
        .route("/{short}", get(get_full_link))
        .route("/refresh", post(post_refresh))
        .with_state(state);

    let addr = format!(
        "{}:{}",
        env::var("HOST").unwrap(),
        env::var("PORT").unwrap()
    );
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

/// Root renders an HTML file showing all available links
async fn get_root(State(state): State<AppState>) -> String {
    // TODO: get full link
    String::from("<h1>linkshort<h1><span>Public link directory coming soon!</span>")
}

async fn get_full_link(Path(short): Path<String>, State(state): State<AppState>) -> String {
    // TODO: get full link from state
    String::from("HI")
}

/// Refreshes the data
async fn post_refresh(State(state): State<AppState>) -> StatusCode {
    match fetch_data().await {
        Ok(res) => {
            // TODO: populate
            StatusCode::OK
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn fetch_data() -> reqwest::Result<HashMap<String, String>> {
    let data_url = env::var("DATA_URL").unwrap();
    let body = reqwest::get(data_url).await?.text().await?;
    let res = HashMap::new();
    // TODO: parse body and apply to the data
    Ok(res)
}

fn assert_env() {
    let needed = ["HOST", "PORT", "DATA_URL"];
    for var in needed {
        env::var(var).expect(&format!("{} must be provided", var));
    }
}

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<HashMap<String, String>>>,
}

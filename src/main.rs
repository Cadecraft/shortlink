use axum::{
    Router,
    extract::Path,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    routing::{get, post},
};
use dashmap::DashMap;
use std::collections::hash_map::HashMap;
use std::env;
use std::sync::Arc;

mod fetch_data;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    assert_env();

    println!("Fetching initial data...");
    let data = fetch_data::fetch()
        .await
        .expect("Could not fetch initial data from DATA_URL");

    let initial_cache = DashMap::new();
    overwrite_dashmap(&initial_cache, &data);

    let state = AppState {
        data: Arc::new(initial_cache),
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

async fn get_root() -> String {
    // TODO: in the future, server-side render HTML showing all available public links
    String::from(
        "Welcome to shortlink (https://github.com/Cadecraft/shortlink)\n\nPublic link directory coming soon!",
    )
}

async fn get_full_link(
    Path(short): Path<String>,
    State(state): State<AppState>,
) -> (StatusCode, HeaderMap) {
    match state.data.get(&short) {
        Some(full_url) => {
            let mut headers = HeaderMap::new();
            headers.insert(header::LOCATION, full_url.parse().unwrap());
            (StatusCode::MOVED_PERMANENTLY, headers)
        }
        None => (StatusCode::NOT_FOUND, HeaderMap::new()),
    }
}

/// Refreshes the data
async fn post_refresh(State(state): State<AppState>) -> StatusCode {
    match fetch_data::fetch().await {
        Some(res) => {
            overwrite_dashmap(&state.data, &res);
            StatusCode::OK
        }
        None => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn assert_env() {
    let needed = ["HOST", "PORT", "DATA_URL"];
    for var in needed {
        env::var(var).unwrap_or_else(|_| panic!("{} must be provided", var));
    }
}

fn overwrite_dashmap(data: &DashMap<String, String>, new_values: &HashMap<String, String>) {
    data.clear();
    new_values.iter().for_each(|(k, v)| {
        data.insert(k.to_string(), v.to_string());
    });
}

#[derive(Clone)]
struct AppState {
    data: Arc<DashMap<String, String>>,
}

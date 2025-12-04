// src/state.rs
#[derive(Clone)]
pub struct AppState {
    pub tmdb_api_key: String,
    pub client: reqwest::Client,
}
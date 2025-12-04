use axum::{ extract::{ Path, Query, State }, Json };
use crate::models::{ TmdbResponse, VideoResponse, PageQuery, SearchQuery };
use crate::state::AppState;

pub async fn root() -> &'static str {
    "Netflix Backend is Online"
}

pub async fn get_trending_movies(
    State(state): State<AppState>,
    Query(params): Query<PageQuery>
) -> Json<TmdbResponse> {
    let page = params.page.unwrap_or(1);
    let url = format!(
        "https://api.themoviedb.org/3/trending/all/week?api_key={}&page={}",
        state.tmdb_api_key,
        page
    );
    let res = state.client.get(&url).send().await.unwrap().json::<TmdbResponse>().await.unwrap();
    Json(res)
}

pub async fn search_content(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>
) -> Json<TmdbResponse> {
    let page = params.page.unwrap_or(1);
    let url = format!(
        "https://api.themoviedb.org/3/search/multi?api_key={}&query={}&page={}&include_adult=false",
        state.tmdb_api_key,
        params.query,
        page
    );
    let res = state.client.get(&url).send().await.unwrap().json::<TmdbResponse>().await.unwrap();
    Json(res)
}

pub async fn get_movie_videos(
    State(state): State<AppState>,
    Path(id): Path<i32>
) -> Json<VideoResponse> {
    let url = format!(
        "https://api.themoviedb.org/3/movie/{}/videos?api_key={}",
        id,
        state.tmdb_api_key
    );
    let res = state.client.get(&url).send().await.unwrap().json::<VideoResponse>().await.unwrap();
    Json(res)
}

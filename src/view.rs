use axum::{extract::Query, response::IntoResponse, Json};
use chrono::{Datelike, Utc};

use super::request::Params;
use super::response::Response;

// Root view
pub async fn root(Query(params): Query<Params>) -> impl IntoResponse {
    let date = Utc::now();

    let response = sunrise::sunrise_sunset(
        params.latitude.unwrap_or(51.5072),
        params.longitude.unwrap_or(0.1276),
        params.year.unwrap_or_else(|| date.year()),
        params.month.unwrap_or_else(|| date.month()),
        params.day.unwrap_or_else(|| date.day()),
    );

    Json(Response::from(response))
}

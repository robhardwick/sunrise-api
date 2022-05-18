use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    sunrise: String,
    sunset: String,
}

impl From<(i64, i64)> for Response {
    fn from((sunrise, sunset): (i64, i64)) -> Self {
        let sunrise = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(sunrise, 0), Utc);
        let sunset = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(sunset, 0), Utc);

        Self {
            sunrise: sunrise.to_rfc3339(),
            sunset: sunset.to_rfc3339(),
        }
    }
}

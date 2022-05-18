use serde::{de, Deserialize, Deserializer};
use std::{fmt, str::FromStr};

#[derive(Deserialize)]
pub struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub latitude: Option<f64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub longitude: Option<f64>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub year: Option<i32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub month: Option<u32>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub day: Option<u32>,
}

/// Serde deserialization decorator to map empty Strings to None,
pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

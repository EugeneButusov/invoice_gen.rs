use chrono::{ DateTime, FixedOffset};
use serde::{self, de, Deserialize, Deserializer};

pub mod date_serialize {
    use chrono::{Date, Utc};
    use serde::{self, Serializer};

    pub fn serialize<S>(date: &Date<Utc>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(&date.format("%B %d, %Y").to_string())
    }
}

pub fn date_deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(s).map_err(de::Error::custom)
}

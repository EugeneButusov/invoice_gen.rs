use super::ClockifySettings;
use crate::util::date_deserialize;
use chrono::{DateTime, Duration, FixedOffset, SecondsFormat};
use std::ops::Add;

#[derive(Deserialize, Debug)]
struct TimeInterval {
    #[serde(deserialize_with = "date_deserialize")]
    end: DateTime<FixedOffset>,
    #[serde(deserialize_with = "date_deserialize")]
    start: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug)]
struct TimeEntry {
    #[serde(rename = "timeInterval")]
    time_interval: TimeInterval,
}

pub struct Client {
    settings: ClockifySettings,
}

impl Client {
    pub fn new(settings: ClockifySettings) -> Client {
        Client { settings }
    }

    pub fn get_duration_for_period(
        &self,
        user_id: &String,
        from: &DateTime<FixedOffset>,
        to: &DateTime<FixedOffset>,
    ) -> Duration {
        let url = format!(
            // TODO: now selection capped up to 5000 entries, need to make looped extractions
            "https://api.clockify.me/api/v1/workspaces/{}/user/{}/time-entries?start={}&end={}&page-size=5000",
            self.settings.workspace_id,
            user_id,
            from.to_rfc3339_opts(SecondsFormat::Millis, true),
            to.to_rfc3339_opts(SecondsFormat::Millis, true)
        );

        let client = reqwest::blocking::Client::new();

        let response = client
            .get(url)
            .header("X-Api-Key", &self.settings.api_key)
            .send();

        let time_entries: Vec<TimeEntry> = match response {
            Ok(response) => match response.json::<Vec<TimeEntry>>() {
                Ok(data) => Some(data),
                Err(parse_error) => {
                    print!("Error occurred parsing time entry data: {}", parse_error);
                    None
                }
            },
            Err(reqwest_error) => {
                print!("Error occurred during http request: {}", reqwest_error);
                None
            }
        }
        .unwrap();

        let mut result = Duration::zero();
        for entry in time_entries {
            let duration = entry
                .time_interval
                .end
                .signed_duration_since(entry.time_interval.start);
            result = result.add(duration);
        }

        result
    }
}

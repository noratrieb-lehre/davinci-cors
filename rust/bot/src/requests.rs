use crate::error::BotResult;
use dto::{Class, Event, GetEventQueryParams, Timetable};
use reqwest::header::HeaderMap;
use reqwest::Client;
use tracing::{debug, info};

const BASE_URL: &str = "http://localhost:8080/api";

pub struct CorsClient {
    client: reqwest::Client,
}

impl CorsClient {
    pub fn from_token(token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token)
                .parse()
                .expect("Authorization header invalid"),
        );
        Self {
            client: Client::builder()
                .default_headers(headers)
                .build()
                .expect("Could not create client"),
        }
    }

    pub async fn get_events(
        &self,
        guild_id: u64,
        before: Option<i64>,
        after: Option<i64>,
    ) -> BotResult<Vec<Event>> {
        let class_id = self.get_class(guild_id).await?.id;

        let params = serde_url_params::to_string(&GetEventQueryParams { before, after })
            .expect("Query params be valid");

        let res = self
            .client
            .get(format!(
                "{}/classes/{}/events?{}",
                BASE_URL, class_id, params
            ))
            .send()
            .await?;

        debug!(status = %res.status());
        debug!(params = %params);

        let events = res.json().await?;
        Ok(events)
    }

    pub async fn get_timetable(&self, guild_id: u64) -> BotResult<Option<Timetable>> {
        let class_id = self.get_class(guild_id).await?.id;

        let res = self
            .client
            .get(format!("{}/classes/{}/timetable", BASE_URL, class_id))
            .send()
            .await?;

        if res.status() == 404 {
            Ok(None)
        } else {
            let timetable = res.json().await?;
            Ok(Some(timetable))
        }
    }

    async fn get_class(&self, guild_id: u64) -> BotResult<Class> {
        let rse = self
            .client
            .get(format!("{}/classes/discord/{}", BASE_URL, guild_id))
            .send()
            .await?;
        debug!(status = %rse.status());
        let class = rse.json::<dto::Class>().await?;
        Ok(class)
    }
}

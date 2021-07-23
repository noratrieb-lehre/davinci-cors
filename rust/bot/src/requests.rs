use crate::error::{BotError, BotResult};
use dto::{Class, Event};
use reqwest::header::HeaderMap;
use reqwest::Client;

const BASE_URL: &str = "http://localhost:8080/api";

pub struct CorsClient {
    client: reqwest::Client,
}

impl CorsClient {
    pub fn from_token(token: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Token", format!("Bearer {}", token).parse().unwrap());
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
        before: Option<u64>,
        after: Option<u64>,
    ) -> BotResult<Vec<Event>> {
        let class_id = self.get_class(guild_id).await?.id;

        let params = match (before, after) {
            (Some(before), Some(after)) => format!("&before={}&after={}", before, after),
            (Some(before), None) => format!("&before={}", before),
            (None, Some(after)) => format!("&after={}", after),
            _ => "".to_string(),
        };

        let body = self
            .client
            .get(format!(
                "{}/classes/{}/events{}",
                BASE_URL, class_id, params
            ))
            .send()
            .await?
            .json::<Vec<Event>>()
            .await?;

        Ok(body)
    }

    async fn get_class(&self, guild_id: u64) -> BotResult<Class> {
        let body = self
            .client
            .get(format!("{}/classes/discord/{}", BASE_URL, guild_id))
            .send()
            .await?;
        println!("{}", body.status());
        println!("{}", body.text().await?);
        //let class = body.json::<dto::Class>().await?;
        println!("Loaded classid");
        Err(BotError::Other("pain"))
        //Ok(class)
    }
}

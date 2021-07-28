use crate::error::BotResult;
use dto::{Class, Event, GetEventQueryParams, NotificationRes, Timetable};
use reqwest::header::HeaderMap;
use reqwest::{Client, StatusCode};
use serenity::model::id::UserId;
use tracing::debug;
use uuid::Uuid;

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

    pub async fn get_notifications(&self, old_timestamp: i64) -> BotResult<NotificationRes> {
        debug!(after = %old_timestamp, "Getting notifications");

        let res = self
            .client
            .get(format!(
                "{}/bot/notifications?since={}",
                BASE_URL, old_timestamp
            ))
            .send()
            .await?;
        debug!(res = %res.status(), "Get notification response status");

        let data = res.json::<NotificationRes>().await?;
        Ok(data)
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

        debug!(status = %res.status(), "Get events status");
        debug!(params = %params, "Get events sent params");

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

    pub async fn edit_guild_settings(&self, guild: &dto::Guild) -> BotResult<()> {
        let res = self
            .client
            .put(format!("{}/bot/guilds", BASE_URL))
            .json(guild)
            .send()
            .await?;

        debug!(status = %res.status());
        res.error_for_status()?;
        Ok(())
    }

    pub async fn get_guild(&self, guild_id: u64) -> BotResult<Option<dto::Guild>> {
        let res = self
            .client
            .get(format!("{}/bot/guilds/{}", BASE_URL, guild_id))
            .send()
            .await?;

        debug!(status = %res.status());
        if let StatusCode::NOT_FOUND = res.status() {
            return Ok(None);
        }

        Ok(Some(res.json().await?))
    }

    pub async fn get_member(&self, id: UserId, class_id: Uuid) -> BotResult<Option<dto::Member>> {
        debug!("gettings member...");
        let res = self
            .client
            .get(format!("{}/users/discord/{}", BASE_URL, id.0))
            .send()
            .await?;

        debug!(status = %res.status(), "Get user");
        if let StatusCode::NOT_FOUND = res.status() {
            return Ok(None);
        }

        let user = res.json::<dto::User>().await?;
        let res2 = self
            .client
            .get(format!(
                "{}/classes/{}/members/{}",
                BASE_URL, class_id, user.id
            ))
            .send()
            .await?;

        debug!(status = %res2.status(), "Get member");
        if let StatusCode::NOT_FOUND = res2.status() {
            return Ok(None);
        }
        let member = res2.json::<dto::Member>().await?;

        Ok(Some(member))
    }

    pub async fn get_class(&self, guild_id: u64) -> BotResult<Class> {
        let res = self
            .client
            .get(format!("{}/classes/discord/{}", BASE_URL, guild_id))
            .send()
            .await?;
        debug!(status = %res.status());
        let class = res.json::<dto::Class>().await?;
        Ok(class)
    }
}

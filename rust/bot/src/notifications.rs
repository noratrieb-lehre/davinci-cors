use crate::error::{BotError, BotResult};
use crate::requests::CorsClient;
use dto::Notification;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::ChannelId;
use serenity::prelude::{Context, RwLock, TypeMap};
use serenity::CacheAndHttp;
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

const LAST_NOTIFICATION_PATH: &str = "last_notifications.txt";

pub async fn start_timer(arc: Arc<CacheAndHttp>, client: Arc<CorsClient>) {
    let mut interval = tokio::time::interval(Duration::from_secs(60 * 10));

    loop {
        interval.tick().await;
        send_notifications(&arc, &client).await
    }
}

async fn send_notifications(http: &CacheAndHttp, client: &CorsClient) -> BotResult<()> {
    let last = match std::fs::read_to_string(LAST_NOTIFICATION_PATH) {
        Ok(str) => match str.parse::<i64>() {
            Ok(n) => chrono::NaiveDateTime::from_timestamp(n / 1000, 0),
            Err(_) => chrono::Utc::now().naive_utc(),
        },
        Err(_) => chrono::Utc::now().naive_utc(),
    };

    let notifications = client.get_notifications(last.timestamp_millis()).await?;

    std::fs::write(LAST_NOTIFICATION_PATH, notifications.time.to_string())
        .map_err(|_| BotError::Other("could not write notification file"))?;

    let notifications = notifications.notifications;

    let sent_messages = notifications
        .into_iter()
        .map(|notification| {
            ChannelId(
                notification
                    .channel
                    .parse()
                    .expect("Valid snowflake from api"),
            )
            .send_message(&http.http, |msg| {
                msg.embed(move |embed| notification_embed(embed, &notification))
            })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(sent_messages)
        .await
        .iter()
        .filter(|result| result.is_err())
        .for_each(|err| debug!(?err, "Error when sending notification"));

    Ok(())
}

fn notification_embed<'a>(embed: &'a mut CreateEmbed, notif: &Notification) -> &'a mut CreateEmbed {
    embed
        .title(format!("Benachrichtigung für {}", notif.event.name))
        .field(&notif.event.start, &notif.event.description, false)
}

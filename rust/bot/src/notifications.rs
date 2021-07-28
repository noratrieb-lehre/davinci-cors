use crate::commands::format_datetime;
use crate::error::{BotError, BotResult};
use crate::requests::CorsClient;
use dto::Notification;
use serenity::builder::CreateEmbed;
use serenity::model::id::RoleId;
use serenity::model::prelude::{ChannelId, Mentionable};
use serenity::CacheAndHttp;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, warn};

const LAST_NOTIFICATION_PATH: &str = "last_notifications.txt";

pub async fn start_timer(arc: Arc<CacheAndHttp>, client: Arc<CorsClient>) {
    tokio::time::sleep(Duration::from_secs(10)).await;

    let mut interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        if let Err(why) = send_notifications(&arc, &client).await {
            error!(%why, "Error while sending notifications")
        }
    }
}

async fn send_notifications(http: &CacheAndHttp, client: &CorsClient) -> BotResult<()> {
    let last = match std::fs::read_to_string(LAST_NOTIFICATION_PATH) {
        Ok(str) => match str.parse::<i64>() {
            Ok(n) => chrono::NaiveDateTime::from_timestamp(n / 1000, 0),
            Err(_) => {
                warn!("Invalid notification file");
                chrono::Utc::now().naive_utc()
            }
        },
        Err(_) => {
            warn!("Could not find notification file");
            chrono::Utc::now().naive_utc()
        }
    };

    let notifications = client.get_notifications(last.timestamp_millis()).await?;

    std::fs::write(LAST_NOTIFICATION_PATH, notifications.time.to_string())
        .map_err(|_| BotError::Other("could not write notification file"))?;

    let notifications = notifications.notifications;

    let pings = notifications
        .clone() // todo oh
        .into_iter()
        .filter(|notif| notif.everyone_ping || notif.role_ping.is_some())
        .map(|notification| {
            let channel = ChannelId(
                notification
                    .channel
                    .parse()
                    .expect("Valid snowflake from api"),
            );
            let ping = match (&notification.role_ping, notification.everyone_ping) {
                (Some(role), true) => format!(
                    "{} @everyone\n",
                    RoleId(role.parse().expect("invalid role id")).mention()
                ),
                (Some(role), false) => {
                    RoleId(role.parse().expect("invalid role id"))
                        .mention()
                        .to_string()
                        + "\n"
                }
                (None, true) => "@everyone\n".to_string(),
                _ => unreachable!(),
            };

            channel.send_message(&http.http, |msg| msg.content(ping))
        })
        .collect::<Vec<_>>();

    let sent_messages = notifications
        .into_iter()
        .map(|notification| {
            let channel = ChannelId(
                notification
                    .channel
                    .parse()
                    .expect("Valid snowflake from api"),
            );

            channel.send_message(&http.http, |msg| {
                msg.embed(move |embed| notification_embed(embed, &notification))
            })
        })
        .collect::<Vec<_>>();

    futures::future::join_all(pings)
        .await
        .iter()
        .filter(|result| result.is_err())
        .for_each(|err| debug!(?err, "Error when sending notification"));
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
        .field(
            format!("Start: {}", format_datetime(notif.event.start)),
            &notif.event.description,
            false,
        )
}

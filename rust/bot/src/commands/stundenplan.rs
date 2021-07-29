use crate::error::{BotError, BotResult};
use crate::functions::format_day_time;
use crate::requests::CorsClient;
use dto::Timetable;
use serenity::client::Context;
use serenity::model::prelude::*;

type Embed = (String, Vec<(String, String, bool)>);

pub async fn handle_timetable(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let guild_id = interaction
        .guild_id
        .ok_or(BotError::Other("Guild id not found"))?;

    let timetable = {
        let map = ctx.data.read().await;
        let client = map.get::<CorsClient>().unwrap();
        client.get_timetable(guild_id.0).await?
    };

    let result = if let Some(timetable) = timetable {
        if timetable.iter().all(|vec| vec.is_empty()) {
            not_found()
        } else {
            show_timetable(timetable)
        }
    } else {
        not_found()
    };

    Ok(interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .create_embed(move |embed| embed.title(result.0).fields(result.1))
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
        })
        .await?)
}

fn not_found() -> Embed {
    (
        ("Stundenplan nicht gefunden".to_string()),
        vec![(
            "Für Admins".to_string(),
            "Aktiviere den Stundenplan in den Admineinstellungen".to_string(),
            false,
        )],
    )
}

fn show_timetable(timetable: Timetable) -> Embed {
    const DAY_NAMES: [&'static str; 7] = [
        "Montag",
        "Dienstag",
        "Mittwoch",
        "Donnerstag",
        "Freitag",
        "Samstag",
        "Sonntag",
    ];

    let title = "Stundenplan".to_string();
    let fields = timetable
        .iter()
        .zip(DAY_NAMES.iter())
        .filter(|(day, _)| !day.is_empty())
        .map(|(day, name)| {
            (
                name.to_string(),
                day.iter()
                    .map(|lesson| {
                        format!(
                            "`{}-{}` {}",
                            format_day_time(lesson.start),
                            format_day_time(lesson.end),
                            lesson.subject
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
                true,
            )
        })
        .collect();
    (title, fields)
}

use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::Utc;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::debug;

use dto::Lesson;
pub use setup::setup_slash_commands;

use crate::commands::functions::{absolute_time_as_weekday, from_utc_timestamp, from_utc_to_cest};
use crate::error::{BotError, BotResult};
use crate::requests::CorsClient;

mod events;
mod functions;
mod setup;

pub async fn create_interaction_response<'a>(
    ctx: &Context,
    data: &ApplicationCommandInteractionData,
    interaction: &Interaction,
) -> BotResult<()> {
    match data.name.as_str() {
        "info" => info(ctx, &interaction).await?,
        "event" => events::handle_event_command(ctx, &interaction, &data.options).await?,
        "wielangenoch" => wie_lange_noch(ctx, interaction).await?,
        name => debug!("{}, {:#?}", name, data.options),
    }
    Ok(())
}

async fn wie_lange_noch(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let guild_id = interaction
        .guild_id
        .ok_or(BotError::Other("Guild id not found"))?;

    let map = ctx.data.read().await;
    let client = map.get::<CorsClient>().unwrap();

    let timetable = client.get_timetable(guild_id.0).await?;
    let found = timetable.is_some();

    let (lesson, next) = if let Some(ref timetable) = timetable {
        let (diff, weekday) = functions::absolute_time_as_weekday(chrono::Utc::now());
        functions::wie_lange_noch(timetable, diff, weekday)
    } else {
        (None, None)
    };

    Ok(interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .create_embed(|embed| wie_lange_noch_embed(embed, found, lesson, next))
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
        })
        .await?)
}

async fn info(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let (corsin, nils) = tokio::join!(
        UserId(546052568619679744).to_user(&ctx.http),
        UserId(414755070161453076).to_user(&ctx.http)
    );
    let (corsin, nils) = (corsin?, nils?);
    let corsin_first = rand::random::<bool>();

    Ok(interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .create_embed(|embed| info_embed(embed, corsin, nils, corsin_first))
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
        })
        .await?)
}

fn wie_lange_noch_embed<'a>(
    embed: &'a mut CreateEmbed,
    found_timetable: bool,
    lesson: Option<&Lesson>,
    next: Option<&Lesson>,
) -> &'a mut CreateEmbed {
    let now = absolute_time_as_weekday(Utc::now()).0;

    match (found_timetable, lesson, next) {
        (false, _, _) => embed.title("Stundenplan nicht gefunden").field(
            "Für Admins",
            "Aktiviere den Stundenplan in den Admineinstellungen",
            false,
        ),
        (_, None, None) => embed.title("Keine Schule mehr heute!"),
        (_, None, Some(next)) => {
            let time_to_next = format_time(next.start - now);
            embed.title("Keine Lektion").field(
                format!("Nächste Lektion: {}", next.subject),
                format!("Start in: {}h", time_to_next),
                false,
            )
        }
        (_, Some(lesson), next) => {
            let (current, _) = absolute_time_as_weekday(Utc::now());
            let remaining = lesson.end - current;
            embed
                .title(format!("Aktuelle Lektion: {}", lesson.subject))
                .field(
                    format!("Noch {}h", format_time(remaining),),
                    &lesson.description,
                    false,
                );
            if let Some(next) = next {
                let time_to_next = format_time(next.start - now);

                embed.field(
                    format!("Nächste Lektion: {}", &next.subject),
                    format!("in {}h", time_to_next),
                    false,
                );
            }
            embed
        }
    }
}

fn format_time(time: i64) -> DelayedFormat<StrftimeItems<'static>> {
    from_utc_timestamp(time).format("%H:%M:%S")
}

fn info_embed(
    embed: &mut CreateEmbed,
    user1: User,
    user2: User,
    user1_first: bool,
) -> &mut CreateEmbed {
    let (u1, u2) = if user1_first {
        (user1, user2)
    } else {
        (user2, user1)
    };

    embed
        .title("CORS")
        .fields(vec![
            (
                "Create Organized Relaxed School",
                "CORS ist ein Terminverwaltungssystem für Schulklassen, durch das nichts mehr vergessen werden kann",
                false,
            ),
            (
                "Creators",
                &format!(
                    "{}#{} & {}#{}",
                    u1.name, u1.discriminator, u2.name, u2.discriminator
                ),
                true,
            ),
            (
                "Github",
                "[davinci-cors](https://github.com/Nilstrieb/davinci-cors)",
                true,
            ),
        ])
        .footer(|f| f.text("Bot Version 0.0"))
}

pub fn format_datetime(
    time: i64,
) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'static>> {
    from_utc_to_cest(from_utc_timestamp(time)).format("%d.%m.%Y %H:%M")
}

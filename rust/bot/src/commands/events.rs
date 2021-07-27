use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::functions::from_utc_timestamp;
use crate::error::{BotError, BotResult};
use crate::requests::CorsClient;
use chrono::Utc;
use serenity::builder::CreateEmbed;
use tracing::{debug, warn};

pub async fn handle_event_command(
    ctx: &Context,
    interaction: &Interaction,
    options: &[ApplicationCommandInteractionDataOption],
) -> BotResult<()> {
    match options.first() {
        Some(subcommand) => match subcommand.name.as_str() {
            "all" => show_all_events(ctx, interaction).await?,
            "next" => show_next_events(ctx, interaction).await?,
            "filter" => show_filter_events(ctx, interaction, subcommand).await?,
            "search" => show_search_events(ctx, interaction, subcommand).await?,
            _ => debug!(?subcommand, "Invalid subcommand"),
        },
        None => debug!("No subcommand"),
    }

    Ok(())
}

async fn show_all_events(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let events = get_events(ctx, interaction.guild_id, None, None).await?;
    warn!(events = ?events);

    send_events(ctx, interaction, events.as_slice()).await
}

async fn show_next_events(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let current_time = Utc::now().timestamp_millis();
    let events = get_events(ctx, interaction.guild_id, None, Some(current_time)).await?;

    warn!(len = %events.len());
    send_events(ctx, interaction, events.as_slice()).await
}

async fn show_filter_events(
    ctx: &Context,
    interaction: &Interaction,
    option: &ApplicationCommandInteractionDataOption,
) -> BotResult<()> {
    let typ = option
        .options
        .first()
        .ok_or(BotError::Other("event show filter has no option"))?;

    if let Some(serde_json::Value::String(typ)) = &typ.value {
        let events = get_events(ctx, interaction.guild_id, None, None)
            .await?
            .into_iter()
            .filter(|event| event.r#type.as_str() == typ)
            .collect::<Vec<_>>();

        let events = events;

        send_events(ctx, interaction, events.as_slice()).await
    } else {
        return Err(BotError::Other("event show filter has invalid option"));
    }
}

async fn show_search_events(
    ctx: &Context,
    interaction: &Interaction,
    option: &ApplicationCommandInteractionDataOption,
) -> BotResult<()> {
    let typ = option
        .options
        .first()
        .ok_or(BotError::Other("event show search has no option"))?;

    if let Some(serde_json::Value::String(query)) = &typ.value {
        let events = get_events(ctx, interaction.guild_id, None, None)
            .await?
            .into_iter()
            .filter(|event| event.name.contains(query) || event.description.contains(query))
            .collect::<Vec<_>>();

        let events = events;

        send_events(ctx, interaction, events.as_slice()).await
    } else {
        return Err(BotError::Other("event show search has invalid option"));
    }
}

async fn get_events(
    ctx: &Context,
    guild_id: Option<GuildId>,
    before: Option<i64>,
    after: Option<i64>,
) -> BotResult<Vec<dto::Event>> {
    let guild_id = guild_id.ok_or(BotError::Other("Guild id not found"))?;

    let map = ctx.data.read().await;
    let client = map.get::<CorsClient>().unwrap();

    Ok(client.get_events(guild_id.0, before, after).await?)
}

async fn send_events(
    ctx: &Context,
    interaction: &Interaction,
    events: &[dto::Event],
) -> BotResult<()> {
    Ok(interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .create_embed(|embed| event_embed(embed, events))
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
        })
        .await?)
}

fn event_embed<'a>(embed: &'a mut CreateEmbed, events: &[dto::Event]) -> &'a mut CreateEmbed {
    let mut fields = events
        .iter()
        .map(|event| {
            let end_value = if let Some(end) = event.end {
                format!(" - {} UTC", format_datetime(end))
            } else {
                "".to_string()
            };

            (
                format!("{} - {}", format_date(event.start), event.name),
                format!(
                    "{} UTC{} \n {}",
                    format_datetime(event.start),
                    end_value,
                    event.description
                ),
                true,
            )
        })
        .collect::<Vec<_>>();

    if fields.is_empty() {
        fields.push((
            "Keine Events gefunden".to_string(),
            "Admins können Events in der Web-Version eintragen".to_string(),
            true,
        ));
    }

    embed
        .title("Events")
        .fields(fields)
        .footer(|f| f.text("CORS"))
}

fn format_date(time: i64) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'static>> {
    from_utc_timestamp(time).format("%d.%m")
}
fn format_datetime(
    time: i64,
) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'static>> {
    from_utc_timestamp(time).format("%d.%m.%Y %H:%M")
}

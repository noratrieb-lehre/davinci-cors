use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::{BotError, BotResult};
use crate::requests::CorsClient;
use chrono::Utc;
use serenity::builder::CreateEmbed;
use tracing::{info, warn};

pub async fn handle_event_command(
    ctx: &Context,
    interaction: &Interaction,
    options: &[ApplicationCommandInteractionDataOption],
) -> BotResult<()> {
    match options.first() {
        Some(subcommand) => match subcommand.name.as_str() {
            "all" => show_all_events(ctx, interaction).await?,
            "next" => show_next_events(ctx, interaction).await?,
            "filter" => show_filter_events(ctx, interaction, options).await?,
            //   "search" => show_search_events(ctx, interaction, options).await?,
            _ => warn!("Invalid subcommand"),
        },
        None => warn!("No subcommand"),
    }

    Ok(())
}

async fn show_all_events(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let events = get_events(ctx, interaction.guild_id, None, None).await?;

    send_events(ctx, interaction, events.as_slice()).await
}

async fn show_next_events(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let current_time = Utc::now().timestamp_millis();
    let events = get_events(ctx, interaction.guild_id, None, Some(current_time)).await?;

    let events = events
        .into_iter()
        .filter(|event| event.end > Some(current_time))
        .collect::<Vec<_>>();

    send_events(ctx, interaction, events.as_slice()).await
}

async fn show_filter_events(
    ctx: &Context,
    interaction: &Interaction,
    options: &[ApplicationCommandInteractionDataOption],
) -> BotResult<()> {
    let events = get_events(ctx, interaction.guild_id, None, None).await?;

    info!(?options);

    let events = events;

    send_events(ctx, interaction, events.as_slice()).await
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
    embed
        .title("Events")
        .fields(
            events
                .iter()
                .map(|event| (&event.name, &event.description, true))
                .collect::<Vec<(&String, &String, bool)>>(),
        )
        .footer(|f| f.text("CORS"))
}

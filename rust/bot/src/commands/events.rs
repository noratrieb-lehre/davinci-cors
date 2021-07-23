use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::{BotError, BotResult};
use crate::requests::CorsClient;
use serenity::builder::CreateEmbed;

pub async fn handle_event_command(
    ctx: &Context,
    interaction: &Interaction,
    options: &[ApplicationCommandInteractionDataOption],
) -> BotResult<()> {
    match options.first() {
        Some(subcommand) => match subcommand.name.as_str() {
            "show" => show_events(ctx, interaction).await?,
            _ => eprintln!("Invalid subcommand"),
        },
        None => eprintln!("No subcommand"),
    }

    Ok(())
}

async fn show_events(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    let guild_id = interaction
        .guild_id
        .ok_or(BotError::Other("Guild id not found"))?;

    let map = ctx.data.read().await;
    let client = map.get::<CorsClient>().unwrap();

    let events = client.get_events(guild_id.0, None, None).await?;

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

fn event_embed(embed: &mut CreateEmbed, events: Vec<dto::Event>) -> &mut CreateEmbed {
    embed
        .title("Events")
        .fields(
            events
                .into_iter()
                .map(|event| (event.name, event.description, true))
                .collect::<Vec<(String, String, bool)>>(),
        )
        .footer(|f| f.text("CORS"))
}

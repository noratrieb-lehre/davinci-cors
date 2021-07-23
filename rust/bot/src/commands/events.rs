use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::error::{BotError, BotResult};

pub async fn handle_event_command(
    ctx: &Context,
    interaction: &Interaction,
    options: &Vec<ApplicationCommandInteractionDataOption>,
) -> BotResult<()> {
    match options.first() {
        Some(subcommand) => match subcommand.name.as_str() {
            "show" => show_event(ctx, interaction).await?,
            _ => eprintln!("Invalid subcommand"),
        },
        None => eprintln!("No subcommand"),
    }

    Ok(())
}

async fn show_event(ctx: &Context, interaction: &Interaction) -> BotResult<()> {
    Ok(())
}

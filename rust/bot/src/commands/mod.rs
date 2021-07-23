mod events;
mod setup;

use serenity::builder::CreateEmbed;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::result::Result;

use crate::error::BotError;
pub use setup::setup_slash_commands;

pub async fn create_interaction_response<'a>(
    ctx: &Context,
    data: &ApplicationCommandInteractionData,
    interaction: &Interaction,
) -> Result<(), BotError> {
    match data.name.as_str() {
        "info" => info(ctx, &interaction).await?,
        "events" => events::handle_event_command(ctx, &interaction, &data.options).await?,
        "wielangenoch" => todo!(),
        name => Ok(println!("{}, {:#?}", name, data.options)),
    }
    Ok(())
}

async fn info(ctx: &Context, interaction: &Interaction) -> Result<()> {
    let (corsin, nils) = tokio::join!(
        UserId(546052568619679744).to_user(&ctx.http),
        UserId(414755070161453076).to_user(&ctx.http)
    );
    let (corsin, nils) = (corsin?, nils?);
    let corsin_first = rand::random::<bool>();

    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .create_embed(|embed| info_embed(embed, corsin, nils, corsin_first))
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
        })
        .await
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

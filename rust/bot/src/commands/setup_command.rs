use crate::error::{BotError, BotResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::requests::CorsClient;
use tracing::{debug, info};
use uuid::Uuid;

type SetupEmbed = (String, (String, String, bool));

pub async fn handle_setup_command(
    ctx: &Context,
    interaction: &Interaction,
    options: &[ApplicationCommandInteractionDataOption],
) -> BotResult<()> {
    let (title, (f1, f2, f3)) = setup_command(ctx, interaction, options).await?;

    Ok(interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message
                        .create_embed(|embed| embed.title(title).field(f1, f2, f3))
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                })
        })
        .await?)
}

pub async fn setup_command(
    ctx: &Context,
    interaction: &Interaction,
    options: &[ApplicationCommandInteractionDataOption],
) -> BotResult<SetupEmbed> {
    let map = ctx.data.read().await;
    let client = map.get::<CorsClient>().unwrap();

    let user = interaction
        .member
        .as_ref()
        .ok_or(BotError::Other("Member not found"))?;

    let guild = interaction
        .guild_id
        .as_ref()
        .ok_or(BotError::Other("Guild not found"))?;

    let class_id = client.get_class(guild.0).await?;
    let has_rights = validate_admin_perms(client, user.user.id, class_id.id).await?;

    match has_rights {
        None => Ok((
            "Account nicht verlinkt".to_string(),
            (
                "Discord-Account nicht mit CORS-Account verlinkt".to_string(),
                "Verlinke deinen Discord Account mit deinem CORS-Account auf der Website"
                    .to_string(),
                false,
            ),
        )),
        Some(false) => Ok((
            "Keine Rechte".to_string(),
            (
                "Braucht: Administrator".to_string(),
                "Das ändern von Servereinstellungen braucht mindestens Administratorenrechte"
                    .to_string(),
                false,
            ),
        )),
        Some(true) => match options.first() {
            Some(subcommand) => match subcommand.name.as_str() {
                "notification_channel" => {
                    Ok(notification_channel(ctx, interaction, subcommand).await?)
                }
                "notification_everyone_ping" => {
                    Ok(notification_everyone_ping(ctx, interaction, subcommand).await?)
                }
                "notification_role_ping" => {
                    Ok(notification_role_ping(ctx, interaction, subcommand).await?)
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        },
    }
}

async fn notification_channel(
    ctx: &Context,
    interaction: &Interaction,
    option: &ApplicationCommandInteractionDataOption,
) -> BotResult<SetupEmbed> {
    let channel = option
        .options
        .first()
        .ok_or(BotError::Other("event show search has no option"))?;

    match &channel.value {
        Some(value) => {
            info!(?value, "value")
        }
        None => {
            info!("no channel provided")
        }
    }

    todo!()
}

async fn notification_everyone_ping(
    ctx: &Context,
    interaction: &Interaction,
    option: &ApplicationCommandInteractionDataOption,
) -> BotResult<SetupEmbed> {
    todo!()
}

async fn notification_role_ping(
    ctx: &Context,
    interaction: &Interaction,
    option: &ApplicationCommandInteractionDataOption,
) -> BotResult<SetupEmbed> {
    todo!()
}

async fn validate_admin_perms(
    client: &CorsClient,
    userid: UserId,
    class_id: Uuid,
) -> BotResult<Option<bool>> {
    let member = client.get_member(userid, class_id).await?;
    Ok(member.map(|member| member.role.has_rights()))
}

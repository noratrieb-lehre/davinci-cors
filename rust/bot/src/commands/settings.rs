use crate::error::{BotError, BotResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::requests::CorsClient;
use tracing::{debug, warn};
use uuid::Uuid;

type SetupEmbed = (&'static str, (String, String, bool));

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

    let guild_id = interaction
        .guild_id
        .as_ref()
        .ok_or(BotError::Other("Guild not found"))?;
    let mut guild = match client.get_guild(guild_id.0).await? {
        Some(guild) => guild,
        None => {
            return Ok((
                "Klasse nicht registriert",
                (
                    "Verbinde die Klasse mit dem Discord Server im Admin Panel".to_string(),
                    "Diese Aktion kann nur von Administratoren durchgeführt werden.".to_string(),
                    false,
                ),
            ))
        }
    };

    let class_id = client.get_class(guild_id.0).await?;
    let has_rights = validate_admin_perms(client, user.user.id, class_id.id).await?;

    match has_rights {
        None => Ok((
            "Account nicht verlinkt",
            (
                "Discord-Account nicht mit CORS-Account verlinkt".to_string(),
                "Verlinke deinen Discord Account mit deinem CORS-Account auf der Website"
                    .to_string(),
                false,
            ),
        )),
        Some(false) => Ok((
            "Keine Rechte",
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
                    Ok(notification_channel(subcommand, client, &mut guild).await?)
                }
                "notification_everyone_ping" => {
                    Ok(notification_everyone_ping(subcommand, client, &mut guild).await?)
                }
                "notification_role_ping" => {
                    Ok(notification_role_ping(subcommand, client, &mut guild).await?)
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        },
    }
}

async fn notification_channel(
    option: &ApplicationCommandInteractionDataOption,
    client: &CorsClient,
    guild: &mut dto::Guild,
) -> BotResult<SetupEmbed> {
    match option.options.first() {
        Some(ApplicationCommandInteractionDataOption {
            resolved: Some(ApplicationCommandInteractionDataOptionValue::Channel(channel)),
            ..
        }) => {
            debug!(?channel, "notification channel");
            client
                .edit_guild_settings({
                    guild.notif_channel = Some(channel.id.to_string());
                    guild
                })
                .await?;
            Ok((
                "Benachrichtigungschannel gesetzt",
                (
                    "Benachrichtigungschannel gesetzt.".to_string(),
                    format!(
                        "Benachrichtigungen werden jetzt in <#{}> geschickt",
                        channel.id
                    ),
                    false,
                ),
            ))
        }
        Some(_) => {
            warn!("Not a channel");
            unreachable!()
        }
        None => {
            debug!("no channel provided");
            client
                .edit_guild_settings({
                    guild.notif_channel = None;
                    guild
                })
                .await?;
            Ok((
                "Benachrichtigungen ausgeschaltet",
                (
                    "Benachrichtigungen ausgeschaltet.".to_string(),
                    "Benachrichtigungen können jederzeit wieder eingeschaltet werden.".to_string(),
                    false,
                ),
            ))
        }
    }
}

async fn notification_everyone_ping(
    option: &ApplicationCommandInteractionDataOption,
    client: &CorsClient,
    guild: &mut dto::Guild,
) -> BotResult<SetupEmbed> {
    match option.options.first() {
        Some(ApplicationCommandInteractionDataOption {
            resolved: Some(ApplicationCommandInteractionDataOptionValue::Boolean(everyone)),
            ..
        }) => {
            debug!(?everyone, "everyone ping");
            client
                .edit_guild_settings({
                    guild.notif_ping_everyone = *everyone;
                    guild
                })
                .await?;
            Ok(match everyone {
                true => (
                    "@everyone Ping eingeschaltet",
                    (
                        "Ping ausgeschaltet".to_string(),
                        "Bei Benachrichtigungen wird jetzt in @everyone gepingt".to_string(),
                        false,
                    ),
                ),
                false => (
                    "@everyone Ping ausgeschaltet",
                    (
                        "Ping ausgeschaltet".to_string(),
                        "Bei Benachrichtigungen wird jetzt nicht mehr @everyone gepingt"
                            .to_string(),
                        false,
                    ),
                ),
            })
        }
        value => {
            warn!(?value, "Invalid boolean");
            unreachable!()
        }
    }
}

async fn notification_role_ping(
    option: &ApplicationCommandInteractionDataOption,
    client: &CorsClient,
    guild: &mut dto::Guild,
) -> BotResult<SetupEmbed> {
    match option.options.first() {
        Some(ApplicationCommandInteractionDataOption {
            resolved: Some(ApplicationCommandInteractionDataOptionValue::Role(role)),
            ..
        }) => {
            debug!(?role, "notification role");
            client
                .edit_guild_settings({
                    guild.notif_ping_role = Some(role.id.to_string());
                    guild
                })
                .await?;
            Ok((
                "Benachrichtigungsrolle gesetzt",
                (
                    "Benachrichtigungsrolle gesetzt".to_string(),
                    format!(
                        "Bei Benachrichtigungen wird jetzt in <@&{}> gepingt",
                        role.id
                    ),
                    false,
                ),
            ))
        }
        Some(_) => {
            warn!("Not a role");
            unreachable!()
        }
        None => {
            debug!("no role provided");
            client
                .edit_guild_settings({
                    guild.notif_ping_role = None;
                    guild
                })
                .await?;
            Ok((
                "Benachrichtigungsrolle ausgeschaltet",
                (
                    "Benachrichtigungsrolle ausgeschaltet.".to_string(),
                    "Die Benachrichtigungsrolle kann jederzeit wieder eingeschaltet werden."
                        .to_string(),
                    false,
                ),
            ))
        }
    }
}

async fn validate_admin_perms(
    client: &CorsClient,
    userid: UserId,
    class_id: Uuid,
) -> BotResult<Option<bool>> {
    let member = client.get_member(userid, class_id).await?;
    Ok(member.map(|member| member.role.has_rights()))
}

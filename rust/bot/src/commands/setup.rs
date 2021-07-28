use serenity::builder::CreateApplicationCommands;
use serenity::model::interactions::ApplicationCommandOptionType;
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::info;

pub async fn setup_slash_commands(ctx: &Context) {
    //ApplicationCommand::create_global_application_commands(&ctx.http, create_commands)
    //    .await
    //    .expect("Could not create slash commands");

    GuildId(865480040682749982)
        .create_application_commands(&ctx.http, create_commands)
        .await
        .expect("Could not create slash commands");

    info!("Setup slash commands.");
}

fn create_commands(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
    commands
        .create_application_command(|command| command.name("events").description("Zeigt Events an"))
        .create_application_command(|command| {
            command
                .name("wielangenoch")
                .description("Zeigt an, wie lange die Lektion nocht geht")
        })
        .create_application_command(|command| {
            command
                .name("event")
                .description("Events verwalten")
                .create_option(|option| {
                    option
                        .name("all")
                        .description("Alle Events anzeigen")
                        .kind(ApplicationCommandOptionType::SubCommand)
                })
                .create_option(|option| {
                    option
                        .name("next")
                        .description("Die nächsten Events anzeigen")
                        .kind(ApplicationCommandOptionType::SubCommand)
                })
                .create_option(|option| {
                    option
                        .name("filter")
                        .description("Die nächsten Events anzeigen")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|option| {
                            option
                                .name("typ")
                                .description("Der Typ nach dem gefiltert werden soll")
                                .kind(ApplicationCommandOptionType::String)
                                .add_string_choice("Hausaufgabe", "homework")
                                .add_string_choice("Prüfung", "exam")
                                .add_string_choice("Ferien", "holidays")
                                .add_string_choice("Andere", "other")
                                .required(true)
                        })
                })
                .create_option(|option| {
                    option
                        .name("search")
                        .description("Die nächsten Events anzeigen")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|option| {
                            option
                                .name("query")
                                .description("Der Suchterm")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
        })
        .create_application_command(|command| command.name("info").description("Botinformationen"))
        .create_application_command(|command| {
            command
                .name("setup")
                .description("Servereinstellungen")
                .create_option(|option| {
                    option
                        .name("notification_channel")
                        .description("Channel für Benachrichtigungen (leer zum Aussschalten)")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|option| {
                            option
                                .name("channel")
                                .description("Der Channel für Benachrichtigungen")
                                .kind(ApplicationCommandOptionType::Channel)
                                .required(false)
                        })
                })
                .create_option(|option| {
                    option
                        .name("notification_everyone_ping")
                        .description("@everyone in Benachrichtigungen pingen")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|option| {
                            option
                                .name("everyone")
                                .description("@everyone pingen")
                                .kind(ApplicationCommandOptionType::Boolean)
                                .required(true)
                        })
                })
                .create_option(|option| {
                    option
                        .name("notification_role_ping")
                        .description(
                            "Eine Rolle in Benachrichtigungen pingen (leer zum Aussschalten)",
                        )
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|option| {
                            option
                                .name("role")
                                .description("Die role zum pingen")
                                .kind(ApplicationCommandOptionType::Role)
                                .required(false)
                        })
                })
        })
}

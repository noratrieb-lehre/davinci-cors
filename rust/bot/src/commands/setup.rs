use serenity::builder::CreateApplicationCommands;
use serenity::model::interactions::ApplicationCommandOptionType;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn setup_slash_commands(ctx: &Context) {
    //ApplicationCommand::create_global_application_commands(&ctx.http, create_commands)
    //    .await
    //    .expect("Could not create slash commands");

    GuildId(865480040682749982)
        .create_application_commands(&ctx.http, create_commands)
        .await
        .expect("Could not create slash commands");

    println!("Setup slash commands.");
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
            command.name("wln").description("Alias für wielangenoch")
        })
        .create_application_command(|command| {
            command
                .name("event")
                .description("Events verwalten")
                .create_option(|option| {
                    option
                        .name("show")
                        .description("Events anzeigen")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .create_sub_option(|option| {
                            option
                                .name("selector")
                                .description("Welche Events man will")
                                .kind(ApplicationCommandOptionType::String)
                                .required(false)
                                .add_string_choice("Alle Events", "all")
                                .add_string_choice("Die nächsten Events (default)", "next")
                                .add_string_choice("Nur Events eines types", "filter")
                                .add_string_choice("In Name und Description suchen", "search")
                        })
                        .create_sub_option(|option| {
                            option
                                .name("parameter")
                                .description("Welche Events man will")
                                .kind(ApplicationCommandOptionType::String)
                                .required(false)
                        })
                })
            // })
        })
        .create_application_command(|commands| {
            commands.name("info").description("Botinformationen")
        })
}

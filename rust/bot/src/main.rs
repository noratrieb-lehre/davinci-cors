mod commands;
mod error;
mod requests;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "CORS Bot connected - {}#{} ({})",
            ready.user.name, ready.user.discriminator, ready.user.id
        );
        commands::setup_slash_commands(&ctx).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if interaction.kind == InteractionType::ApplicationCommand {
            if let Some(data) = interaction.data.as_ref() {
                match data {
                    InteractionData::ApplicationCommand(data) => {
                        if let Err(why) =
                            commands::create_interaction_response(&ctx, data, &interaction).await
                        {
                            eprintln!("Error: {}", why);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok().expect("Could not load .env file");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let app_id = std::env::var("APPLICATION_ID")
        .expect("Expected a token in the environment")
        .parse()
        .expect("Invalid app id");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(app_id)
        .await
        .expect("Could not create client");

    println!("Connecting client...");

    client.start().await.expect("Could not create client");
}

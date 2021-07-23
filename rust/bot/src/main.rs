mod commands;
mod error;
mod requests;

use crate::requests::CorsClient;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::sync::Arc;

struct Handler;

impl TypeMapKey for CorsClient {
    type Value = Arc<Self>;
}

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
            if let Some(InteractionData::ApplicationCommand(data)) = interaction.data.as_ref() {
                if let Err(why) =
                    commands::create_interaction_response(&ctx, data, &interaction).await
                {
                    eprintln!("Error: {}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Could not load .env file");

    let token = std::env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let cors_token =
        std::env::var("CORS_API_TOKEN").expect("Expected a cors api token in the environment");
    let app_id = std::env::var("APPLICATION_ID")
        .expect("Expected a token in the environment")
        .parse()
        .expect("Invalid app id");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(app_id)
        .await
        .expect("Could not create client");

    {
        let mut data = client.data.write().await;
        data.insert::<CorsClient>(Arc::new(CorsClient::from_token(cors_token)))
    }

    println!("Connecting client...");

    client.start().await.expect("Could not create client");
}

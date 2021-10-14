use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::{async_trait, Client};

// Local
mod commands;
mod state;
mod utils;
use commands::*;
use songbird::SerenityInit;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&MUSIC_GROUP);

    let mut client = Client::builder(TOKEN)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await?;

    client.start().await?;

    Ok(())
}

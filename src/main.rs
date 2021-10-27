use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;
use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::prelude::Activity;
use serenity::prelude::TypeMapKey;
use serenity::{async_trait, Client};

// Local
mod commands;
mod model;
mod state;
mod utils;
use commands::*;
use songbird::SerenityInit;

struct Database;

impl TypeMapKey for Database {
    type Value = mongodb::Database;
}

// Event bot handler
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);

        // set bot activity
        ctx.set_activity(Activity::listening("$ayuda")).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        // All commands groups are in groups module
        .help(&MY_HELP)
        .group(&MUSIC_GROUP)
        .group(&PLAYLIST_GROUP);

    let mut client = Client::builder(std::env::var("DISCORD_TOKEN").unwrap())
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await?;

    {
        let client_options =
            ClientOptions::parse(std::env::var("MONGO_URI").unwrap()).await?;

        let mongo_client = MongoClient::with_options(client_options)?;
        let db = mongo_client.database("shizu");

        // Bot global data
        let mut data = client.data.write().await;
        data.insert::<Database>(db);
    }

    client.start().await?;

    Ok(())
}

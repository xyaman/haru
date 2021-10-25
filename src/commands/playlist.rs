use std::time::Duration;

use crate::{Database, model, utils};
use mongodb::bson::doc;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::*,
};

#[group("collector")]
#[prefixes(playlist, pl)]
#[commands(new, add)]
struct Playlist;

#[command]
#[max_args(1)]
async fn new(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let name = match args.single::<String>() {
        Ok(n) => n,
        Err(_) => {
            msg.reply(&ctx.http, "Necesito un nombre para crear una playlist")
                .await?;
            return Ok(());
        }
    };

    // Check if there is already a playlist with that name in this guild
    let db = {
        let data_read = ctx.data.read().await;

        // Mongo database is wrapped in Arc
        // https://docs.rs/mongodb/2.0.1/mongodb/struct.Database.html
        data_read.get::<Database>().expect("").clone()
    };

    let playlist_coll = db.collection::<model::Playlist>("playlists");

    // Check if playlist name exists in our collection
    let filter = doc! { "name": &name, "guild_id": msg.guild_id.unwrap().to_string()};
    if let Some(_) = playlist_coll.find_one(filter, None).await? {
        msg.reply(&ctx.http, "Ya existe una playlist con ese nombre").await?;
        return Ok(());
    }

    // Now we create the playlist with that name
    let playlist = model::Playlist::new(name, msg.guild_id.unwrap().to_string());
    playlist_coll.insert_one(playlist, None).await?;

    // Feedback to user
    msg.reply(&ctx.http, "Playlist `{}` creada, para mas informacion usa $help")
        .await?;

    Ok(())
}

#[command]
#[min_args(2)]
#[description = "Agrega una cancion a una playlist"]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let pl_name = args.single::<String>().unwrap();
    // let track_name = args.single::<String>().unwrap();
    let track_name = args.rest();

    // Check if there is already a playlist with that name in this guild
    let db = {
        let data_read = ctx.data.read().await;

        // Mongo database is wrapped in Arc
        // https://docs.rs/mongodb/2.0.1/mongodb/struct.Database.html
        data_read.get::<Database>().expect("").clone()
    };

    let playlist_coll = db.collection::<model::Playlist>("playlists");
    
    // Check if playlist name exists in our collection
    let filter = doc! { "name": &pl_name, "guild_id": msg.guild_id.unwrap().to_string()};
    let pl = match playlist_coll.find_one(filter, None).await? {
        Some(pl) => pl,
        None => {
            msg.reply(&ctx.http, "En este servidor no existe una playlist con este nombre").await?;
            return Ok(());
        }
    };

    let source;
    let is_url = track_name.starts_with("http");

    // Handle urls different than query
    if is_url {
        source = songbird::ytdl(track_name).await?;
    } else {
        source = songbird::input::ytdl_search(track_name).await?;
    }

    let res_reaction = msg.channel_id
    .send_message(&ctx.http, |m| {
        m.embed(|e| utils::playlist_add_track_embed(e, *source.metadata));
        m
    })
    .await?;

    // Add reactions
    let yes = '✅';
    let no = '❌';
    res_reaction.react(&ctx.http, yes).await?;
    res_reaction.react(&ctx.http, no).await?;
    
    if let Some(reaction) = res_reaction.await_reaction(&ctx).timeout(Duration::from_secs(60)).author_id(msg.author.id).await {
        
        let emoji = &reaction.as_inner_ref().emoji;
        
        match emoji.as_data().as_str() {
        // add new track
        "✅" => {
            let filter = doc! { "_id": pl.id };
            let update = doc! {"$push": {"tracks": {"query": track_name}}};
            playlist_coll.update_one(filter, update, None).await?;
        }
        // dont add track
        "❌" => {}
        _ => {}
        }

    }
    Ok(())
}

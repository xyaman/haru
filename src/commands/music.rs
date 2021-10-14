use crate::utils;
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    http::Http,
    model::prelude::*,
};
use songbird::{
    input::Metadata, Event, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent,
};
use std::sync::Arc;

/// Struct that implements VoiceEventHandler
/// `act` will be called when every song ends
struct TrackEnded {
    manager: Arc<Songbird>,
    http: Arc<Http>,
    guild_id: GuildId,
    channel_id: ChannelId,
    metadata: Metadata,
    mention: String,
}

/// Called when song end
#[serenity::async_trait]
impl VoiceEventHandler for TrackEnded {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        // See if bot is still connected to guild
        if let Some(handler_lock) = self.manager.get(self.guild_id) {
            let mut handler = handler_lock.lock().await;

            // If queue is 0, then leave the guild to avoid unnecessary data usage
            if handler.queue().len() == 0 {
                let _ = handler.leave().await;
                return None;
            }

            // Send now playing embed for the next song
            let next_song_metadata = handler.queue().current()?.metadata().clone();
            let _ = self
                .channel_id
                .send_message(&self.http, |m| {
                    m.embed(|e| utils::now_playing_embed(e, next_song_metadata, &self.mention))
                })
                .await;
        }

        None
    }
}

#[group]
#[commands(play, skip)]
pub struct Music;

#[command]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // Join channel always
    join_channel(ctx, msg).await?;

    // If there are no args, send a messsage and return
    if args.is_empty() {
        msg.channel_id
            .say(&ctx.http, "mmmm necesito una palabra o un link de youtube")
            .await?;
        return Ok(());
    }

    // This should never fails
    let guild = msg.guild(&ctx.cache).await.expect("Can't get guild");

    // Get global songbird instance
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird is not initialized");

    // None if the bot is not connected (this should always works, because we connect to server in
    // line above)
    if let Some(handler_lock) = manager.get(guild.id) {
        let mut handler = handler_lock.lock().await;

        let full_message = args.message();
        let is_url = full_message.starts_with("http");
        let source;

        // Handle urls different than query
        if is_url {
            source = songbird::ytdl(full_message).await;
        } else {
            source = songbird::input::ytdl_search(full_message).await;
        }

        let source = match source {
            Ok(source) => source,
            Err(why) => {
                msg.channel_id
                    .say(&ctx.http, format!("[Internal Error] {}", why))
                    .await?;

                return Ok(());
            }
        };

        let metadata = source.metadata.clone();

        // Start song if queue len is 0, otherwise will be just queued.
        handler.enqueue_source(source);

        let channel_id = msg.channel_id.clone();
        let http = ctx.http.clone();

        // Add event handler
        let _ = handler
            .queue()
            .current_queue()
            .last()
            .expect("No queue, but it should be at least  1")
            .add_event(
                Event::Track(TrackEvent::End),
                TrackEnded {
                    manager,
                    guild_id: guild.id,
                    http,
                    metadata: *metadata.clone(),
                    channel_id,
                    mention: msg.author.mention().to_string(),
                },
            );

        // If there is already playing a song, print that song is queued
        if handler.queue().len() > 1 {
            // Send queued embed
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| utils::song_queued_embed(e, *metadata, handler.queue().len()));
                    m
                })
                .await?;
        } else {
            // Send now playing embed
            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        utils::now_playing_embed(e, *metadata, msg.author.mention().to_string())
                    });
                    m
                })
                .await?;
        }
    }

    Ok(())
}

#[command]
async fn skip(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = if let Some(id) = msg.guild_id {
        id
    } else {
        return Ok(());
    };

    // Get songbird bot instance
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird is not initialized");

    // None if the bot is not connected
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        if let Err(e) = handler.queue().skip() {
            // Send Error
            println!("Error when skiping: {}", e);
        }
    } else {
        // Currently the bot is not in a channel voice in this guild
        msg.reply(&ctx.http, "No estoy en ningun canal de voz :p")
            .await?;
    }

    Ok(())
}

async fn join_channel(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.expect("Cant get build");
    let _user = msg.channel_id;

    // Get user current voice channel
    let voice_channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|s| s.channel_id);

    let voice_channel_id = match voice_channel_id {
        Some(c) => c,
        None => {
            msg.reply(&ctx.http, "You need to be in a voice channel")
                .await?;
            return Ok(());
        }
    };

    // Retrieve the Songbird voice client from a serenity context’s shared key-value store.
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation");

    // It seems this method already handle cases when user connects to same voice channel
    let _handler = manager.join(guild.id, voice_channel_id).await;

    Ok(())
}

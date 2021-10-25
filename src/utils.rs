use serenity::builder::CreateEmbed;
use songbird::{input::Metadata, tracks::TrackQueue};

/// Returns a CreateEmbed with song info. It should be used with the current playing track/song
pub fn now_playing_embed<'a>(embed: &'a mut CreateEmbed, metadata: Metadata, mention: &str) -> &'a mut CreateEmbed {
    embed.title(metadata.title.unwrap_or("No Name".into()));
    embed.thumbnail(metadata.thumbnail.unwrap_or_default());
    embed.description(format!("[{}]", mention));

    embed
}

/// Returns a CreateEmbed with song info. It should be used with a new enqueued track/song
pub fn song_queued_embed<'a>(embed: &'a mut CreateEmbed, metadata: Metadata, position: usize) -> &'a mut CreateEmbed {
    let title = metadata.title.unwrap_or("No name".into());

    embed.description(format!("**{}** - agregada a la cola en posición: {}", title, position));
    embed.thumbnail(metadata.thumbnail.unwrap_or_default());

    embed
}

/// Returns a String with info of the track queue
pub fn track_queue_content<'a>(queue: &TrackQueue) -> String {
    let mut content = String::new();
    content.push_str("```go\n"); // go just for a little color syntax
    content.push_str("# | Nombre\n");

    for (index, track) in queue.current_queue().iter().enumerate() {
        let metadata = track.metadata();
        let track_info = format!("{} | {}\n", index, metadata.title.as_ref().unwrap());
        content.push_str(&track_info);
    }

    content.push_str("```");
    content
}

/// Returns a CreateEmbed with new track info. It's used when user may want to confirm new track of
/// a playlist
pub fn playlist_add_track_embed<'a>(e: &'a mut CreateEmbed, m: Metadata) -> &'a mut CreateEmbed {
    e.title("¿Deseas agregar esta cancion?");
    e.description(m.title.unwrap_or_default());
    e.thumbnail(m.thumbnail.unwrap_or_default());
    e.url(m.source_url.unwrap_or_default());
    e
}

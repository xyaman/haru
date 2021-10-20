use serenity::builder::CreateEmbed;
use songbird::{input::Metadata, tracks::TrackQueue};

pub fn now_playing_embed<'a>(
    embed: &'a mut CreateEmbed,
    metadata: Metadata,
    mention: impl Into<String>,
) -> &'a mut CreateEmbed {
    embed.title(metadata.title.unwrap_or("No Name".into()));
    embed.thumbnail(metadata.thumbnail.unwrap_or_default());
    embed.description(mention.into());

    embed
}

pub fn song_queued_embed<'a>(
    embed: &'a mut CreateEmbed,
    metadata: Metadata,
    position: usize,
) -> &'a mut CreateEmbed {
    let title = metadata.title.unwrap_or("No name".into());

    embed.description(format!(
        "{} - agregada a la cola en posición: {}",
        title, position
    ));
    embed.thumbnail(metadata.thumbnail.unwrap_or_default());

    embed
}

pub fn track_queue_content<'a>(queue: &TrackQueue) -> String {
    let mut content = String::new();
    content.push_str("```go"); // go just for a little color syntax
    content.push_str("# | Nombre\n");

    for (index, track) in queue.current_queue().iter().enumerate() {
        let metadata = track.metadata();
        let track_info = format!("{} | {}\n", index, metadata.title.as_ref().unwrap());
        content.push_str(&track_info);
    }

    content.push_str("```");
    content
}

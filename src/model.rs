use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Playlist {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub name: String,
    pub guild_id: String,
    #[serde(default)]
    pub tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(name: String, guild_id: String) -> Self {
        Self {
            id: None,
            name,
            guild_id,
            tracks: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub query: String,
}

impl Track {
    pub fn new(query: String) -> Self {
        Self { query }
    }
}

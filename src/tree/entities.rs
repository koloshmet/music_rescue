use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct TrackInfo<'a> {
    pub artist_name: &'a str,
    pub artist: &'a Artist,
    pub album_title: &'a str,
    pub album: &'a Album,
    pub track_no: usize,
    pub track: &'a Option<Track>,
}

#[derive(Clone)]
pub struct ArtistInfo<'a> {
    pub artist_name: &'a str,
    pub artist: &'a Artist,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub title: String,
    pub file: std::path::PathBuf,
    pub new_file: Option<std::path::PathBuf>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Album {
    pub year: i32,
    pub tracks: Vec<Option<Track>>,
}

impl Album {
    pub fn new(y: i32) -> Self {
        Self {
            year: y,
            tracks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Artist {
    pub albums: std::collections::HashMap<String, Album>,
}

impl Artist {
    pub fn new() -> Self {
        Self::default()
    }
}

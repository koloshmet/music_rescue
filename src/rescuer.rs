use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Track {
    pub title: String,
    pub file: String
}

#[derive(Serialize, Deserialize, Clone)]
struct Album {
    pub year: i32,
    pub tracks: Vec<Option<Track>>
}

impl Album {
    pub fn new(y: i32) -> Self {
        return Self{ year: y, tracks: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Artist {
    pub albums: std::collections::HashMap<String, Album>
}

impl Artist {
    pub fn new() -> Self {
        return Self{ albums: std::collections::HashMap::new() }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct MediaTree {
    pub artists: std::collections::HashMap<String, Artist>
}

pub struct MusicRescuer {
    media_tree: MediaTree
}

impl MusicRescuer {
    pub fn new() -> Self {
        return Self{
            media_tree: MediaTree{ artists: std::collections::HashMap::new() }
        };
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        return serde_json::to_string_pretty(&self.media_tree);
    }

    pub fn rescue_dir(&mut self, dir: &std::path::Path) {
        for entry_res in std::fs::read_dir(dir).unwrap() {
            if let Ok(entry) = entry_res {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    self.rescue_file(entry_path.as_path());
                } else if entry_path.is_dir() {
                    self.rescue_dir(entry_path.as_path());
                }
            } else {
                eprintln!("Error: can't read dir {}", dir.display());
            }
        }
    }

    fn rescue_file(&mut self, file: &std::path::Path) {
        if let Ok(tag) = audiotags::Tag::new().read_from_path(file) {
            if let (Some(artist), Some(album), Some(year)) = (tag.artist(), tag.album_title(), tag.year()) {
                if let (Some(title), Some(track_n), Some(file_s)) = (tag.title(), tag.track_number(), file.to_str()) {
                    self.add_track(file_s, artist, album, year, track_n as usize, title);
                } else {
                    eprintln!("Error: incorrect track data {}", file.display());
                }
            } else {
                eprintln!("Error: incorrect artist/album data {}", file.display());
            }
        } else {
            eprintln!("Error: can't read audio file {}", file.display());
        }
    }

    fn add_track(&mut self, file: &str, artist: &str, album: &str, year: i32, track_number: usize, title: &str) {
        let art = self.media_tree.artists.entry(artist.to_string()).or_insert(Artist::new());
        let alb = art.albums.entry(album.to_string()).or_insert(Album::new(year));
        if alb.tracks.len() < track_number {
            alb.tracks.resize(track_number, None);
        }
        alb.tracks[track_number - 1] = Some(Track{title: title.to_string(), file: file.to_string()});
    }
}


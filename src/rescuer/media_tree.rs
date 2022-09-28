use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct Track {
    pub title: String,
    pub file: String,
    pub new_file: Option<String>
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
pub struct MediaTree {
    artists: std::collections::HashMap<String, Artist>,
    root: Option<std::path::PathBuf>
}

impl MediaTree {
    pub fn new() -> Self {
        return Self{
            artists: std::collections::HashMap::new(),
            root: None
        };
    }

    pub fn with_root(root: &std::path::Path) -> Self {
        return Self{
            artists: std::collections::HashMap::new(),
            root: Some(std::path::PathBuf::from(root))
        };
    }

    pub fn add_track(
        &mut self,
        file: &std::path::Path,
        artist: &str,
        album: &str,
        year: i32,
        track_number: usize,
        title: &str)
    {
        let new_file_path = self.root.as_ref().and_then(|root| {
            let mut path = root.clone();
            path.push(MediaTree::clean_bad_chars(artist.to_string()));
            path.push(MediaTree::clean_bad_chars(format!("{} - {}", year, album)));
            file.extension().and_then(|e| e.to_str()).and_then(|ext| {
                path.push(MediaTree::clean_bad_chars(format!("{} - {}.{}", track_number, title, ext)));
                Some(path)
            })
        });

        let art = self.artists.entry(artist.to_string()).or_insert(Artist::new());
        let trc = Some(Track{
            title: title.to_string(),
            file: file.to_string_lossy().to_string(),
            new_file: new_file_path.as_ref().and_then(|p| Some(p.to_string_lossy().to_string()))
        });
        if let Some(alb) = art.albums.get_mut(album) {
            if alb.tracks.len() < track_number {
                alb.tracks.resize(track_number, None);
            }
            alb.tracks[track_number - 1] = trc;
        } else {
            let mut alb = Album::new(year);
            alb.tracks.resize(track_number, None);
            alb.tracks[track_number - 1] = trc;
            art.albums.insert(album.to_string(), alb);
            if let Some(album_path) = new_file_path.as_ref().and_then(|f| f.parent()) {
                std::fs::create_dir_all(album_path).unwrap();
            }
        }
        if let Some(file_path) = new_file_path {
            std::fs::rename(file, file_path).unwrap();
        }
    }

    fn clean_bad_chars(mut s: String) -> String {
        const BAD_CHARS: [char; 9] = ['<', '>', ':', '"', '\\', '/', '|', '?', '*'];
        s.retain(|c| !BAD_CHARS.contains(&c));
        return s;
    }
}

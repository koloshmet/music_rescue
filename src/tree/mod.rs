pub mod entities;

use entities::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct MusicTree {
    artists: std::collections::HashMap<String, Artist>,
    root: std::path::PathBuf,
}

impl MusicTree {
    pub fn from_root(root: std::path::PathBuf) -> Self {
        Self {
            artists: std::collections::HashMap::new(),
            root,
        }
    }

    pub fn root(&self) -> &std::path::PathBuf {
        &self.root
    }

    pub fn artists<'a>(&'a self) -> TraverseArtists<'a> {
        TraverseArtists {
            artists: self.artists.iter(),
        }
    }

    pub fn tracks<'a>(&'a self) -> TraverseTracks<'a> {
        TraverseTracks::new(&self.artists)
    }

    pub fn add_track(
        &mut self,
        file: &std::path::Path,
        artist_name: &str,
        album_title: &str,
        year: i32,
        track_number: usize,
        title: &str,
    ) -> bool {
        let new_file_path = file.extension().map(|ext| {
            let mut path =
                std::path::PathBuf::from(Self::clean_bad_chars(artist_name.to_string()).trim());
            path.push(Self::clean_bad_chars(format!("{year} - {album_title}")).trim());
            path.push(format!(
                "{track_number} - {}",
                Self::clean_bad_chars(title.to_string()),
            ));
            path.set_extension(ext);
            path
        });

        let artist = self.artists.entry(artist_name.to_string()).or_default();
        let track = Some(Track {
            title: title.to_string(),
            file: std::path::PathBuf::from(file),
            new_file: new_file_path,
        });
        if let Some(album) = artist.albums.get_mut(album_title) {
            if album.tracks.len() < track_number {
                album.tracks.resize(track_number, None);
            }
            let track_entry = &mut album.tracks[track_number - 1];
            if track_entry.is_some() {
                return false;
            }
            *track_entry = track;
        } else {
            let mut album = Album::new(year);
            album.tracks.resize(track_number, None);
            album.tracks[track_number - 1] = track;
            artist.albums.insert(album_title.to_string(), album);
        }

        true
    }

    fn clean_bad_chars(mut s: String) -> String {
        const BAD_CHARS: [char; 10] = ['<', '>', ':', '"', '\\', '/', '|', '?', '*', '.'];
        s.retain(|c| !BAD_CHARS.contains(&c));
        s
    }
}

#[derive(Default)]
pub struct TraverseTracks<'a> {
    artists: std::collections::hash_map::Iter<'a, String, Artist>,
    albums: std::collections::hash_map::Iter<'a, String, Album>,
    tracks: std::slice::Iter<'a, Option<Track>>,
    cur: Option<TrackInfo<'a>>,
}

impl<'a> TraverseTracks<'a> {
    fn new(artist_map: &'a std::collections::HashMap<String, Artist>) -> Self {
        let mut result = Self {
            artists: artist_map.iter(),
            ..Default::default()
        };

        let Some((name, artist)) = result.artists.next() else {
            return result;
        };

        result.albums = artist.albums.iter();
        let Some((title, album)) = result.albums.next() else {
            return result;
        };

        result.tracks = album.tracks.iter();
        let Some(track) = result.tracks.next() else {
            return result;
        };
        result.cur = Some(TrackInfo {
            artist_name: name,
            artist,
            album_title: title,
            album,
            track_no: 1,
            track,
        });

        result
    }
}

impl<'a> Iterator for TraverseTracks<'a> {
    type Item = TrackInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.cur.clone();

        let Some(cur) = self.cur.as_mut() else {
            return result;
        };

        loop {
            if let Some(track) = self.tracks.next() {
                cur.track_no += 1;
                cur.track = track;
                break;
            }

            if let Some((album_name, album)) = self.albums.next() {
                cur.album_title = album_name;
                cur.album = album;
                cur.track_no = 0;
                self.tracks = album.tracks.iter();
                continue;
            }

            if let Some((artist_name, artist)) = self.artists.next() {
                cur.artist_name = artist_name;
                cur.artist = artist;
                self.albums = artist.albums.iter();
                continue;
            }

            self.cur = None;
            break;
        }

        result
    }
}

#[derive(Default)]
pub struct TraverseArtists<'a> {
    artists: std::collections::hash_map::Iter<'a, String, Artist>,
}

impl<'a> Iterator for TraverseArtists<'a> {
    type Item = ArtistInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.artists.next().map(|(artist_name, artist)| ArtistInfo {
            artist_name,
            artist,
        })
    }
}

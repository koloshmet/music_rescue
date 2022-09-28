mod media_tree;

pub struct MusicRescuer {
    media_tree: media_tree::MediaTree,
    progress_counter: usize,
    error_counter: usize
}

impl MusicRescuer {
    pub fn new() -> Self {
        return Self{
            media_tree: media_tree::MediaTree::new(),
            progress_counter: 0,
            error_counter: 0
        };
    }

    pub fn with_target(target: &std::path::Path) -> Self {
        return Self{
            media_tree: media_tree::MediaTree::with_root(target),
            progress_counter: 0,
            error_counter: 0
        };
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

    pub fn to_json(&self) -> serde_json::Result<String> {
        return serde_json::to_string_pretty(&self.media_tree);
    }

    pub fn print_report(&self) {
        println!("======\nREPORT\n======");
        println!("Rescued: {}\nErrors: {}", self.progress_counter, self.error_counter);
    }

    fn rescue_file(&mut self, file: &std::path::Path) {
        if file.extension().is_none() {
            eprintln!("Note: skipping not an audio file {}", file.display());
            return;
        }
        if let Ok(tag) = audiotags::Tag::new().read_from_path(file) {
            self.rescue_audio(file, tag.as_ref());
        } else {
            eprintln!("Warning: can't read audio file {}", file.display());
        }
    }

    fn rescue_audio(&mut self, file: &std::path::Path, tag: &dyn audiotags::AudioTag) {
        if let (Some(artist), Some(album), Some(year)) = (tag.artist(), tag.album_title(), tag.year()) {
            if let (Some(title), Some(track_n)) = (tag.title(), tag.track_number()) {
                self.report_progress();
                self.media_tree.add_track(file, artist, album, year, track_n as usize, title);
            } else {
                self.report_error();
                eprintln!("Error: incorrect track data {}", file.display());
            }
        } else {
            self.report_error();
            eprintln!("Error: incorrect artist/album data {}", file.display());
        }
    }

    fn report_progress(&mut self) {
        self.progress_counter += 1;
        if self.progress_counter % 100 == 0 {
            println!("Progress: {} items processed", self.progress_counter);
        }
    }

    fn report_error(&mut self) {
        self.error_counter += 1;
    }
}


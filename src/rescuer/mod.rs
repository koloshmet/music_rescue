mod reporter;

use reporter::{Reporter, RescueErrorLevel};

use crate::tree::MusicTree;

pub struct MusicRescuer {
    music_tree: MusicTree,
    reporter: Reporter,
}

impl MusicRescuer {
    pub fn from_root(root: std::path::PathBuf) -> std::io::Result<Self> {
        Self::verify_root(&root)?;

        let mut result = Self {
            music_tree: MusicTree::from_root(root),
            reporter: Reporter::new(),
        };
        result.scan();
        Ok(result)
    }

    pub fn from_index(music_tree: MusicTree) -> std::io::Result<Self> {
        Self::verify_root(music_tree.root())?;

        Ok(Self {
            music_tree,
            reporter: Reporter::new(),
        })
    }

    pub fn rescue(&mut self, target_path: &std::path::Path) {
        for track_info in self.music_tree.tracks() {
            let Some(track) = track_info.track else {
                continue;
            };
            let Some(new_track_file) = &track.new_file else {
                continue;
            };

            let path = target_path.join(new_track_file);
            let Some(album_path) = path.parent() else {
                panic!("Fatal: incorrect new_file {} in index", path.display());
            };
            if let Err(e) = std::fs::create_dir_all(album_path) {
                panic!("Fatal: can't create dir {}; {}", album_path.display(), e);
            }
            if path.exists() {
                self.reporter
                    .report_error(RescueErrorLevel::RescueExists, &path);
                continue;
            }
            if let Err(e) = std::fs::copy(&track.file, path) {
                panic!("Fatal: can't copy file {}; {}", track.file.display(), e);
            }
            self.reporter.report_progress();
        }
    }

    pub fn print_report(&self) {
        self.reporter.print_report();
    }

    pub fn music_tree(&self) -> &MusicTree {
        &self.music_tree
    }

    fn verify_root(root: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        if !root.as_ref().exists() {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }
        if !root.as_ref().is_dir() {
            return Err(std::io::Error::from(std::io::ErrorKind::NotADirectory));
        }
        Ok(())
    }

    fn scan(&mut self) {
        let mut stack = vec![self.music_tree.root().clone()];
        while let Some(dir) = stack.pop().as_ref() {
            let Ok(dir_it) = std::fs::read_dir(dir).inspect_err(|e| {
                eprintln!("Error: can't read dir {}, error {}", dir.display(), e);
            }) else {
                continue;
            };

            let reporting_dir_it = dir_it.filter_map(|entry| match entry {
                Err(e) => {
                    eprintln!(
                        "Error: can't read entry in dir {}, error {}",
                        dir.display(),
                        e
                    );
                    None
                }
                Ok(entry) => Some(entry),
            });
            for entry in reporting_dir_it {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    self.scan_file(entry_path.as_path());
                } else if entry_path.is_dir() {
                    stack.push(entry_path);
                }
            }
        }
    }

    fn scan_file(&mut self, file: &std::path::Path) {
        if file.extension().is_none() {
            return self
                .reporter
                .report_error(RescueErrorLevel::Extention, file);
        }
        let Ok(tag) = audiotags::Tag::new().read_from_path(file) else {
            return self.reporter.report_error(RescueErrorLevel::Data, file);
        };
        self.scan_audio(file, tag.as_ref());
    }

    fn scan_audio(&mut self, file: &std::path::Path, tag: &dyn audiotags::AudioTag) {
        let (Some(artist), Some(album), Some(year)) = (tag.artist(), tag.album_title(), tag.year())
        else {
            return self.reporter.report_error(RescueErrorLevel::Album, file);
        };
        let (Some(title), Some(track_n)) = (tag.title(), tag.track_number()) else {
            return self.reporter.report_error(RescueErrorLevel::Track, file);
        };

        self.reporter.report_progress();
        self.music_tree
            .add_track(file, artist, album, year, track_n as usize, title);
    }
}

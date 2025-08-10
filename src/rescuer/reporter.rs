#[derive(Default)]
pub(super) struct Reporter {
    progress_counter: usize,
    error_counter: usize,
}

impl Reporter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_report(&self) {
        println!("\n======\nREPORT\n======\n");
        println!(
            "Rescued: {}\nErrors: {}",
            self.progress_counter, self.error_counter
        );
    }

    pub fn report_progress(&mut self) {
        self.progress_counter += 1;
        print!("Progress: {}/?\r", self.progress_counter);
    }

    pub fn report_error(&mut self, error_level: RescueErrorLevel, file: &std::path::Path) {
        self.error_counter += 1;
        match error_level {
            RescueErrorLevel::Extention => {
                eprintln!("Note: skipping not an audio file {}", file.display());
            }
            RescueErrorLevel::Data => {
                eprintln!("Warning: can't read file as audio {}", file.display());
            }
            RescueErrorLevel::Album => {
                eprintln!("Error: incorrect artist/album data {}", file.display());
            }
            RescueErrorLevel::Track => {
                eprintln!("Error: incorrect track data {}", file.display());
            }
            RescueErrorLevel::RescueExists => {
                eprintln!("Warning: target file {} already exists", file.display());
            }
        }
    }
}

pub(super) enum RescueErrorLevel {
    Extention,
    Data,
    Album,
    Track,
    RescueExists,
}

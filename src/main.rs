use music_rescue::MusicRescuer;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Index {
        #[arg(long)]
        work_dir: Option<std::path::PathBuf>,
        #[arg(long)]
        out_index: Option<std::path::PathBuf>,
    },
    Rescue {
        #[arg(long)]
        work_dir: Option<std::path::PathBuf>,
        #[arg(long)]
        index: Option<std::path::PathBuf>,

        out_dir: std::path::PathBuf,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let cwd = std::env::current_dir()?;

    match cli.command {
        Command::Index {
            work_dir,
            out_index,
        } => {
            let index_path =
                out_index.unwrap_or_else(|| cwd.join("rescue_index").with_extension("json"));

            let index_file = std::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&index_path)
                .inspect_err(|_| eprintln!("Error: can't open file {}", index_path.display()))?;

            let rescuer = MusicRescuer::from_root(work_dir.unwrap_or(cwd))?;
            rescuer.print_report();

            serde_json::to_writer_pretty(index_file, rescuer.music_tree())?;

            Ok(())
        }
        Command::Rescue {
            work_dir,
            index,
            out_dir,
        } => {
            let mut rescuer = match index {
                Some(index_path) => {
                    let index_file = std::fs::OpenOptions::new()
                        .read(true)
                        .open(&index_path)
                        .inspect_err(|_| {
                            eprintln!("Error: can't open file {}", index_path.display())
                        })?;

                    MusicRescuer::from_index(serde_json::from_reader(index_file)?)
                }
                None => MusicRescuer::from_root(work_dir.unwrap_or(cwd)),
            }?;

            rescuer.rescue(&out_dir);
            rescuer.print_report();

            Ok(())
        }
    }
}

use std::io::Write;
use crate::rescuer::MusicRescuer;

mod rescuer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root_dir = std::path::Path::new(&args[1]);

    let mut rescuer = if let Some(target_dir) = args.get(2) {
        MusicRescuer::with_target(std::path::Path::new(target_dir))
    } else {
        MusicRescuer::new()
    };

    let result_file = args.get(3).and_then(|path| {
        Some(std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(path).unwrap())
    });

    rescuer.rescue_dir(root_dir);
    rescuer.print_report();

    if let Some(mut file) = result_file {
        file.write_all(rescuer.to_json().unwrap().as_bytes()).unwrap();
    } else {
        println!("{}", rescuer.to_json().unwrap());
    }
}

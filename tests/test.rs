use music_rescue::MusicRescuer;
use music_rescue::tree::MusicTree;

#[test]
fn test() {
    let collection_path = std::env::home_dir()
        .unwrap()
        .join("Music")
        .join("Collection");

    let rescuer = MusicRescuer::from_root(collection_path).unwrap();
    rescuer.print_report();

    let index = serde_json::to_string_pretty(rescuer.music_tree()).unwrap();

    for track_info in rescuer.music_tree().tracks() {
        let file = track_info
            .track
            .as_ref()
            .map(|t| t.new_file.as_ref())
            .unwrap_or_default();
        println!(
            "{}",
            file.map(|p| p.display())
                .unwrap_or(std::path::Path::new("").display()),
        );
    }

    println!("\n{}\n", &index);

    let music_tree: MusicTree = serde_json::from_str(&index).unwrap();
    let rescuer2 = MusicRescuer::from_index(music_tree).unwrap();
    rescuer2.print_report();
    println!("{}", rescuer2.music_tree().root().display());
    for track_info in rescuer2.music_tree().tracks() {
        let file = track_info
            .track
            .as_ref()
            .map(|t| t.new_file.as_ref())
            .unwrap_or_default();
        println!(
            "{}",
            file.map(|p| p.display())
                .unwrap_or(std::path::Path::new("").display()),
        );
    }
}

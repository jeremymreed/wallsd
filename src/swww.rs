use std::process::Command;
use rand::Rng;
use crate::collection;

pub fn set_wallpapers(collection: &collection::Collection, outputs: &Vec<String>) {

    for output in outputs {
        let random_index = rand::thread_rng().gen_range(0..collection.collection.len());

        let random_wallpaper = collection.collection.get(random_index).unwrap();

        let swww_output = Command::new("swww")
            .arg("img")
            .arg("-o")
            .arg(output)
            .arg(random_wallpaper)
            .output()
            .expect("failed to execute process");

        println!("output: {}", output);
        println!("random_wallpaper: {}", random_wallpaper);
        println!("status: {}", swww_output.status);
        println!("stdout: {}", String::from_utf8_lossy(&swww_output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&swww_output.stderr));
    }
}
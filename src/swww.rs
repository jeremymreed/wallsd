use std::process::Command;
use rand::Rng;
use crate::output::Output;

pub fn set_wallpaper(output: &Output) {
    let random_index = rand::thread_rng().gen_range(0..output.images.len());

    let random_wallpaper = output.images.get(random_index).unwrap();

    let swww_output = Command::new("swww")
        .arg("img")
        .arg("-o")
        .arg(&output.name)
        .arg(random_wallpaper)
        .output()
        .expect("failed to execute process");

    tracing::debug!("output: {}", output.name);
    tracing::debug!("random_wallpaper: {}", random_wallpaper);
    tracing::debug!("status: {}", swww_output.status);
    tracing::debug!("stdout: {}", String::from_utf8_lossy(&swww_output.stdout));
    tracing::debug!("stderr: {}", String::from_utf8_lossy(&swww_output.stderr));
}
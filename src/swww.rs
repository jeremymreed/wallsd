use std::process::Command;
use rand::Rng;
use crate::mode::Mode;
use crate::output::Output;

/* Chooses a random wallpaper, and then changes the wallpaper. */
pub fn set_wallpaper(output: &Output) -> String {

    if !output.images.is_empty() {
        let index = match output.mode {
            Mode::Slideshow => {
                rand::thread_rng().gen_range(0..output.images.len())
            },
            Mode::Oneshot => {
                0
            }
        };

        let wallpaper = output.images.get(index).unwrap();

        let swww_output = Command::new("swww")
            .arg("img")
            .arg("-o")
            .arg(&output.name)
            .arg(wallpaper)
            .output()
            .expect("failed to execute process");

        tracing::debug!("output:    {}", output.name);
        tracing::debug!("wallpaper: {}", wallpaper);
        tracing::debug!("status:    {}", swww_output.status);
        tracing::debug!("stdout:    {}", String::from_utf8_lossy(&swww_output.stdout));
        tracing::debug!("stderr:    {}", String::from_utf8_lossy(&swww_output.stderr));

        wallpaper.to_owned()
    } else {
        tracing::warn!("No images found for output: {}", output.name);
        String::from("")
    }
}
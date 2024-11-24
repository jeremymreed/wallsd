use std::process::Command;
use crate::output::Output;

/* Chooses a random wallpaper, and then changes the wallpaper. */
pub fn set_wallpaper(output: &mut Output) {

    if !output.images.is_empty() {
        // TODO: The index should be stored in each output.
        // Shouldn't generate or set the index here.
        let wallpaper = output.get_next_wallpaper();

        let swww_output = Command::new("swww")
            .arg("img")
            .arg("-o")
            .arg(&output.name)
            .arg(&wallpaper)
            .output()
            .expect("failed to execute process");

        tracing::debug!("output:    {}", output.name);
        tracing::debug!("wallpaper: {}", wallpaper);
        tracing::debug!("status:    {}", swww_output.status);
        tracing::debug!("stdout:    {}", String::from_utf8_lossy(&swww_output.stdout));
        tracing::debug!("stderr:    {}", String::from_utf8_lossy(&swww_output.stderr));
    } else {
        // In the case there are no images loaded.
        tracing::warn!("No images found for output: {}", output.name);
        // Do nothing for now.
    }
}
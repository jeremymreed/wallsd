use rand::Rng;
use chrono::{DateTime, Local};
use crate::resolution::Resolution;
use crate::mode::Mode;

// TODO: Consider putting the wallpaer index here, instead of storing the path of the current wallpaper.
#[derive(Debug, Clone)]
pub struct Output {
    pub name: String,
    pub resolution: Resolution,
    pub mode: Mode,
    pub oncalendar_string: String,
    pub target_time: DateTime<Local>,
    pub current_wallpaper: String,
    pub current_index: usize,
    pub images: Vec<String>,
}

impl Output {
    pub fn get_next_wallpaper(&mut self) -> String {
        if !self.images.is_empty() {
            let index = match self.mode {
                Mode::Slideshow => {
                    rand::thread_rng().gen_range(0..self.images.len())
                },
                Mode::Oneshot => {
                    self.current_index
                }
            };

            if index > self.images.len() {
                tracing::error!("Index out of bounds: {:#?}", index);
                panic!("Index out of bounds: {:#?}", index);
            }

            self.current_index = index;
            self.current_wallpaper = self.images[self.current_index].clone();
            self.images[index].clone()
        } else {
            tracing::warn!("No images found for output: {}", self.name);
            String::from("")
        }
    }
}
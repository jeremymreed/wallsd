use zvariant::Type;
use serde::{Deserialize, Serialize};
use crate::mode;

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct OutputSettings {
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub mode: mode::Mode,
    pub oncalendar: String,
    pub current_wallpaper: String,
    pub current_index: u64,
    pub number_of_images: u64,
    pub images: Vec<String>,
}
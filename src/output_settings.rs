use zvariant::Type;
use serde::{Deserialize, Serialize};
use crate::mode;

#[derive(Serialize, Deserialize, Type, Debug)]
pub struct OutputSettings {
    pub name: String,
    pub mode: mode::Mode,
    pub oncalendar: String,
    pub current_wallpaper: String,
    pub number_of_images: u64,
}
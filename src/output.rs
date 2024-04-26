use chrono::{DateTime, Local};
use crate::resolution::Resolution;
use crate::mode::Mode;

#[derive(Debug, Clone)]
pub struct Output {
    pub name: String,
    pub resolution: Resolution,
    pub mode: Mode,
    pub oncalendar_string: String,
    pub target_time: DateTime<Local>,
    pub current_wallpaper: String,
    pub images: Vec<String>,
}
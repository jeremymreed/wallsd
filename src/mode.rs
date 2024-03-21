#[derive(Debug, Clone)]
pub enum Mode {
    ONESHOT,
    SLIDESHOW(String),
}
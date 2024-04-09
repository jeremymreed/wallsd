use crate::resolution::Resolution;
use crate::mode::Mode;

#[derive(Debug, Clone)]
pub struct Output {
    pub name: String,
    pub resolution: Resolution,
    pub mode: Mode,
    pub images: Vec<String>,
}
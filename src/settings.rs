use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub inkscape_path: String,
    pub frame_resolution: usize,
    pub max_threads: usize,
}

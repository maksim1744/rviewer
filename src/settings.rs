use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub conversion_tool: Option<String>,
    pub inkscape_path: Option<String>,
    pub frame_resolution: Option<usize>,
    pub max_threads: Option<usize>,
}

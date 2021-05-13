use crate::figure::Figure;

use std::sync::{Arc, Mutex};

use druid::{Data, Size, Lens};

#[derive(Clone, Data)]
pub struct DrawProperties {
    pub width: f64,
    pub font: f64,
    pub was_messages: usize,
}

#[derive(Clone, Data, Lens)]
pub struct AppData {
    pub objects: Arc<Mutex<Vec<Box<dyn Figure + Send>>>>,
    pub frames: Arc<Mutex<Vec<Vec<usize>>>>,
    pub frame: usize,
    pub fps_speed: Arc<Mutex<f64>>,
    pub size: Arc<Mutex<Size>>,
    pub tags: Arc<Mutex<Vec<(String, bool)>>>,
    pub draw_properties: Arc<Mutex<DrawProperties>>,
    pub svg_width_scale: f64,

    pub finished: Arc<Mutex<bool>>,
}

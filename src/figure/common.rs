use crate::app_data::DrawProperties;
use crate::parse::Params;

use druid::Color;

pub struct CommonParams {
    pub color: Color,
    pub tags: Vec<String>,
    pub keep: bool,
}

impl CommonParams {
    pub fn new(params: &Params, _draw_properties: &mut DrawProperties) -> Self {
        Self {
            color: params.get("col").unwrap_or(Color::rgb8(0 as u8, 0 as u8, 0 as u8)),
            tags: params.get("t").unwrap_or(Vec::new()),
            keep: params.get("k").unwrap_or(false),
        }
    }
}

impl Default for CommonParams {
    fn default() -> Self {
        Self {
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
            keep: false,
        }
    }
}

use crate::app_data::DrawProperties;
use crate::parse::Params;

use druid::Color;

#[derive(Clone)]
pub struct CommonParams {
    pub color: Color,
    pub tags: Vec<String>,
    pub keep: bool,
    pub id: Option<i32>,
    pub func: Option<String>,
}

impl CommonParams {
    pub fn new(params: &Params, _draw_properties: &mut DrawProperties) -> Self {
        Self {
            color: params.get("col").unwrap_or(Color::rgb8(0 as u8, 0 as u8, 0 as u8)),
            tags: params.get("t").unwrap_or(Vec::new()),
            keep: params.get("k").unwrap_or(false),
            id: params.get("id"),
            func: params.get("fu"),
        }
    }
}

impl Default for CommonParams {
    fn default() -> Self {
        Self {
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
            keep: false,
            id: None,
            func: None,
        }
    }
}

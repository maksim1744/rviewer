use crate::app_data::DrawProperties;
use crate::in_between::InBetweenProperties;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::Point;

use std::any::Any;
use std::collections::HashSet;

use svg::Document;

pub mod rect;
pub use rect::MRect;
pub mod circle;
pub use circle::MCircle;
pub mod grid;
pub use grid::MGrid;
pub mod line;
pub use line::MLine;
pub mod poly;
pub use poly::MPoly;
pub mod text;
pub use text::MText;
pub mod message;
pub use message::MMessage;
pub mod common;
pub use common::CommonParams;

pub trait Figure {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point);
    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document;
    fn common(&self) -> &CommonParams;
    fn as_any(&self) -> &dyn Any;

    fn tags(&self) -> &Vec<String> {
        &self.common().tags
    }

    fn keep(&self) -> bool {
        self.common().keep
    }

    fn need_to_draw(&self, tags: &HashSet<String>) -> bool {
        self.tags().is_empty() || self.tags().iter().any(|x| tags.contains(x))
    }

    fn color_to_string(&self) -> String {
        let (r, g, b, _a) = self.common().color.as_rgba8();
        format!("rgb({}, {}, {})", r, g, b)
    }
}

pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Option<Box<dyn Figure + Send>> {
    if s.starts_with("rect") {
        Some(Box::new(MRect::from_string(s, draw_properties)))
    } else if s.starts_with("circle") {
        Some(Box::new(MCircle::from_string(s, draw_properties)))
    } else if s.starts_with("line") {
        Some(Box::new(MLine::from_string(s, draw_properties)))
    } else if s.starts_with("grid") {
        Some(Box::new(MGrid::from_string(s, draw_properties)))
    } else if s.starts_with("poly") {
        Some(Box::new(MPoly::from_string(s, draw_properties)))
    } else if s.starts_with("text") {
        Some(Box::new(MText::from_string(s, draw_properties)))
    } else if s.starts_with("msg") {
        Some(Box::new(MMessage::from_string(s, draw_properties)))
    } else {
        None
    }
}

macro_rules! in_betweens_match {
    ($a:ident, $b:ident, $in_between_properties:ident, $t:ty) => {
        if let Some(a) = $a.as_any().downcast_ref::<$t>() {
            return <$t>::in_betweens(&a, $b.as_any().downcast_ref::<$t>().unwrap(), $in_between_properties)
                .into_iter()
                .map(|x| Box::new(x) as Box<dyn Figure + Send>)
                .collect();
        }
    };
}

pub fn in_betweens(a: &dyn Figure, b: &dyn Figure, in_between_properties: &InBetweenProperties) -> Vec<Box<dyn Figure + Send>> {
    in_betweens_match!(a, b, in_between_properties, MRect);
    in_betweens_match!(a, b, in_between_properties, MCircle);
    in_betweens_match!(a, b, in_between_properties, MLine);
    in_betweens_match!(a, b, in_between_properties, MGrid);
    in_betweens_match!(a, b, in_between_properties, MPoly);
    in_betweens_match!(a, b, in_between_properties, MText);
    Vec::new()
}

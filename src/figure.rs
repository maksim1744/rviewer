use crate::app_data::DrawProperties;

use druid::widget::prelude::*;
use druid::{Point, Color};

use std::collections::HashSet;

use svg::Document;

pub mod rect;
pub use rect::MRect;
pub mod circle;
pub use circle::MCircle;
pub mod line;
pub use line::MLine;
pub mod poly;
pub use poly::MPoly;
pub mod text;
pub use text::MText;
pub mod message;
pub use message::MMessage;

pub trait Figure {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point);

    fn draw_on_image(&self, img: Document, scale: f64) -> Document;

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String>;

    fn need_to_draw(&self, tags: &HashSet<String>) -> bool {
        let iter = self.get_tags();
        if iter.size_hint().0 == 0 {
            true
        } else {
            self.get_tags().any(|x| tags.contains(x))
        }
    }

    fn color_to_string(&self, color: &Color) -> String {
        let (r, g, b, _a) = color.as_rgba8();
        format!("rgb({}, {}, {})", r, g, b)
    }

    // fn color_to_image_rgb(&self, color: &Color) -> Rgb<u8> {
    //     let (r, g, b, _a) = color.as_rgba8();
    //     Rgb([r, g, b])
    // }

    // fn point_to_i32_pair(&self, point: Point) -> (i32, i32) {
    //     (point.x.round() as i32, point.y.round() as i32)
    // }

    // fn point_to_f32_pair(&self, point: Point) -> (f32, f32) {
    //     (point.x as f32, point.y as f32)
    // }

    // fn scale_point(&self, point: Point, scale: f64) -> Point {
    //     Point::new(point.x * scale, point.y * scale)
    // }
}

pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Option<Box<dyn Figure + Send>> {
    if s.starts_with("rect") {
        Some(Box::new(MRect::from_string(s, draw_properties)))
    } else if s.starts_with("circle") {
        Some(Box::new(MCircle::from_string(s, draw_properties)))
    } else if s.starts_with("line") {
        Some(Box::new(MLine::from_string(s, draw_properties)))
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


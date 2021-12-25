use crate::figure::Figure;
use crate::app_data::DrawProperties;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::{Color, Point};
use druid::kurbo::Line;

use svg::Document;
use svg::node::element::Line as SvgLine;

pub struct MLine {
    start: Point,
    finish: Point,
    width: f64,
    color: Color,
    tags: Vec<String>,
    keep: bool,
}

impl MLine {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut line = MLine {
            start: Point::new(0.0, 0.0),
            finish: Point::new(0.0, 0.0),
            width: draw_properties.width,
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
            keep: false,
        };
        let error_message = format!("Can't parse line from string [{}]", s);
        for token in s.split_whitespace() {
            if token.starts_with("s=") {
                let mut iter = token[3..token.len() - 1].split(",");
                line.start.x = iter.next().expect(&error_message).parse().expect(&error_message);
                line.start.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("f=") {
                let mut iter = token[3..token.len() - 1].split(",");
                line.finish.x = iter.next().expect(&error_message).parse().expect(&error_message);
                line.finish.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("w=") {
                line.width = token[2..].trim().parse().expect(&error_message);
            } else if token.starts_with("col=") {
                let mut iter = token[5..token.len() - 1].split(",");
                let r = iter.next().expect(&error_message).parse().expect(&error_message);
                let g = iter.next().expect(&error_message).parse().expect(&error_message);
                let b = iter.next().expect(&error_message).parse().expect(&error_message);
                let a = match iter.next() {
                    Some(x) => x.parse().expect(&error_message),
                    None => 255,
                };
                line.color = Color::rgba8(r, g, b, a);
            } else if token.starts_with("t=") {
                line.tags.push(String::from(token[2..].trim()));
            } else if token == "keep" {
                line.keep = true;
            }
        }
        line
    }
}

impl Figure for MLine {
    fn draw(&self, ctx: &mut PaintCtx, _scale: f64, transform: &dyn Fn(Point) -> Point) {
        let start = transform(self.start);
        let finish = transform(self.finish);
        ctx.stroke(Line::new(start, finish), &self.color, self.width);
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let start = (params.transform)(self.start);
        let finish = (params.transform)(self.finish);
        let line = SvgLine::new()
            .set("x1", start.x)
            .set("y1", params.size.height - start.y)
            .set("x2", finish.x)
            .set("y2", params.size.height - finish.y)
            .set("stroke-width", self.width * params.width_scale)
            .set("stroke", self.color_to_string(&self.color))
            .set("opacity", self.color.as_rgba().3 as f64);
        img.add(line)
    }

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }

    fn is_keep(&self) -> bool {
        self.keep
    }
}

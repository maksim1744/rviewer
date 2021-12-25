use crate::figure::Figure;
use crate::app_data::DrawProperties;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::{Color, Point, Rect};

use svg::Document;
use svg::node::element::Rectangle as SvgRect;

pub struct MRect {
    center: Point,
    size: Point,
    fill: bool,
    width: f64,
    alignment: (char, char),
    color: Color,
    tags: Vec<String>,
    keep: bool,
}

impl MRect {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut rect = MRect {
            center: Point::new(0.0, 0.0),
            size: Point::new(0.0, 0.0),
            fill: false,
            width: draw_properties.width,
            alignment: ('C', 'C'),
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
            keep: false,
        };
        let error_message = format!("Can't parse rect from string [{}]", s);
        for token in s.split_whitespace() {
            if token.starts_with("c=") {
                let mut iter = token[3..token.len() - 1].split(",");
                rect.center.x = iter.next().expect(&error_message).parse().expect(&error_message);
                rect.center.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("s=") {
                let mut iter = token[3..token.len() - 1].split(",");
                rect.size.x = iter.next().expect(&error_message).parse().expect(&error_message);
                rect.size.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("w=") {
                rect.width = token[2..].trim().parse().expect(&error_message);
            } else if token.starts_with("f=") {
                if token == "f=1" {
                    rect.fill = true;
                }
            } else if token.starts_with("a=") {
                rect.alignment = (token.chars().nth(2).expect(&error_message), token.chars().nth(3).expect(&error_message));
            } else if token.starts_with("col=") {
                let mut iter = token[5..token.len() - 1].split(",");
                let r = iter.next().expect(&error_message).parse().expect(&error_message);
                let g = iter.next().expect(&error_message).parse().expect(&error_message);
                let b = iter.next().expect(&error_message).parse().expect(&error_message);
                let a = match iter.next() {
                    Some(x) => x.parse().expect(&error_message),
                    None => 255,
                };
                rect.color = Color::rgba8(r, g, b, a);
            } else if token.starts_with("t=") {
                rect.tags.push(String::from(token[2..].trim()));
            } else if token == "keep" {
                rect.keep = true;
            }
        }
        rect
    }
}

impl Figure for MRect {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point) {
        let mut center = self.center;
        if self.alignment.0 == 'B' {
            center.x += self.size.x / 2.;
        } else if self.alignment.0 == 'E' {
            center.x -= self.size.x / 2.;
        }
        if self.alignment.1 == 'B' {
            center.y += self.size.y / 2.;
        } else if self.alignment.1 == 'E' {
            center.y -= self.size.y / 2.;
        }
        let center = transform(center);
        let mut size = self.size;
        size.x *= scale;
        size.y *= scale;
        let rect = Rect::from_center_size(center, Size::new(size.x, size.y));
        if self.fill {
            ctx.fill(rect, &self.color);
        } else {
            ctx.stroke(rect, &self.color, self.width);
        }
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let color = self.color_to_string(&self.color);
        let mut center = self.center;
        if self.alignment.0 == 'B' {
            center.x += self.size.x / 2.;
        } else if self.alignment.0 == 'E' {
            center.x -= self.size.x / 2.;
        }
        if self.alignment.1 == 'B' {
            center.y += self.size.y / 2.;
        } else if self.alignment.1 == 'E' {
            center.y -= self.size.y / 2.;
        }
        let center = (params.transform)(center);
        let mut rect = SvgRect::new()
            .set("x", center.x - self.size.x / 2.0)
            .set("y", params.size.height - (center.y + self.size.y / 2.0))
            .set("width", self.size.x)
            .set("height", self.size.y)
            .set("stroke-width", self.width * params.width_scale)
            .set("opacity", self.color.as_rgba().3 as f64);
        if self.fill {
            rect = rect.set("fill", color);
        } else {
            rect = rect.set("fill", "none").set("stroke", color);
        }
        img.add(rect)
    }

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }

    fn is_keep(&self) -> bool {
        self.keep
    }
}

use crate::figure::Figure;
use crate::app_data::DrawProperties;

use druid::widget::prelude::*;
use druid::{Color, Point, Rect};

pub struct MRect {
    center: Point,
    size: Point,
    fill: bool,
    width: f64,
    alignment: (char, char),
    color: Color,
    tags: Vec<String>,
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

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }
}

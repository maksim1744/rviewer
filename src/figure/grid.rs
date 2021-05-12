use crate::figure::Figure;
use crate::app_data::DrawProperties;

use druid::widget::prelude::*;
use druid::{Color, Point};
use druid::kurbo::Line;

pub struct MGrid {
    center: Point,
    size: Point,
    dims: (usize, usize),
    width: f64,
    color: Color,
    tags: Vec<String>,
}

impl MGrid {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut grid = MGrid {
            center: Point::new(0.0, 0.0),
            size: Point::new(0.0, 0.0),
            dims: (1, 1),
            width: draw_properties.width,
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
        };
        let error_message = format!("Can't parse grid from string [{}]", s);
        for token in s.split_whitespace() {
            if token.starts_with("c=") {
                let mut iter = token[3..token.len() - 1].split(",");
                grid.center.x = iter.next().expect(&error_message).parse().expect(&error_message);
                grid.center.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("s=") {
                let mut iter = token[3..token.len() - 1].split(",");
                grid.size.x = iter.next().expect(&error_message).parse().expect(&error_message);
                grid.size.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("d=") {
                let mut iter = token[3..token.len() - 1].split(",");
                grid.dims.0 = iter.next().expect(&error_message).parse().expect(&error_message);
                grid.dims.1 = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("w=") {
                grid.width = token[2..].trim().parse().expect(&error_message);
            } else if token.starts_with("col=") {
                let mut iter = token[5..token.len() - 1].split(",");
                let r = iter.next().expect(&error_message).parse().expect(&error_message);
                let g = iter.next().expect(&error_message).parse().expect(&error_message);
                let b = iter.next().expect(&error_message).parse().expect(&error_message);
                let a = match iter.next() {
                    Some(x) => x.parse().expect(&error_message),
                    None => 255,
                };
                grid.color = Color::rgba8(r, g, b, a);
            } else if token.starts_with("t=") {
                grid.tags.push(String::from(token[2..].trim()));
            }
        }
        grid
    }
}

impl Figure for MGrid {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point) {
        let center = transform(self.center);
        let mut size = self.size;
        size.x *= scale;
        size.y *= scale;

        for i in 0..self.dims.0 + 1 {
            ctx.stroke(Line::new(Point::new(center.x - size.x / 2. + size.x / self.dims.0 as f64 * i as f64, center.y - size.y / 2.),
                                 Point::new(center.x - size.x / 2. + size.x / self.dims.0 as f64 * i as f64, center.y + size.y / 2.)), &self.color, self.width);
        }
        for i in 0..self.dims.1 + 1 {
            ctx.stroke(Line::new(Point::new(center.x - size.x / 2., center.y - size.y / 2. + size.y / self.dims.1 as f64 * i as f64),
                                 Point::new(center.x + size.x / 2., center.y - size.y / 2. + size.y / self.dims.1 as f64 * i as f64)), &self.color, self.width);
        }
    }

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }
}

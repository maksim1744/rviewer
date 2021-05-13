use crate::figure::Figure;
use crate::app_data::DrawProperties;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::{Color, Point};
use druid::kurbo::Circle;

use svg::Document;
use svg::node::element::Circle as SvgCircle;

pub struct MCircle {
    center: Point,
    radius: f64,
    fill: bool,
    width: f64,
    color: Color,
    tags: Vec<String>,
}

impl MCircle {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut circle = MCircle {
            center: Point::new(0.0, 0.0),
            radius: 1.0,
            fill: false,
            width: draw_properties.width,
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
        };
        let error_message = format!("Can't parse circle from string [{}]", s);
        for token in s.split_whitespace() {
            if token.starts_with("c=") {
                let mut iter = token[3..token.len() - 1].split(",");
                circle.center.x = iter.next().expect(&error_message).parse().expect(&error_message);
                circle.center.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("r=") {
                let mut iter = token[2..].split(",");
                circle.radius = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("w=") {
                circle.width = token[2..].trim().parse().expect(&error_message);
            } else if token.starts_with("f=") {
                if token == "f=1" {
                    circle.fill = true;
                }
            } else if token.starts_with("col=") {
                let mut iter = token[5..token.len() - 1].split(",");
                let r = iter.next().expect(&error_message).parse().expect(&error_message);
                let g = iter.next().expect(&error_message).parse().expect(&error_message);
                let b = iter.next().expect(&error_message).parse().expect(&error_message);
                let a = match iter.next() {
                    Some(x) => x.parse().expect(&error_message),
                    None => 255,
                };
                circle.color = Color::rgba8(r, g, b, a);
            } else if token.starts_with("t=") {
                circle.tags.push(String::from(token[2..].trim()));
            }
        }
        circle
    }
}

impl Figure for MCircle {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point) {
        let center = transform(self.center);
        let mut r = self.radius;
        r *= scale;
        let circle = Circle::new(center, r);
        if self.fill {
            ctx.fill(circle, &self.color);
        } else {
            ctx.stroke(circle, &self.color, self.width);
        }
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let color = self.color_to_string(&self.color);
        let mut circ = SvgCircle::new()
            .set("cx", self.center.x)
            .set("cy", params.size.height - self.center.y)
            .set("r" , self.radius  )
            .set("stroke-width" , self.width * params.width_scale)
            .set("opacity", self.color.as_rgba().3 as f64);
        if self.fill {
            circ = circ.set("fill", color);
        } else {
            circ = circ.set("fill", "none").set("stroke", color);
        }
        img.add(circ)
    }

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }
}

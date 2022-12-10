use crate::app_data::DrawProperties;
use crate::figure::Figure;
use crate::poly::Poly;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::{Color, Point};

use svg::node::element::Polygon as SvgPolygon;
use svg::node::element::Polyline as SvgPolyline;
use svg::Document;

pub struct MPoly {
    points: Vec<Point>,
    fill: bool,
    width: f64,
    color: Color,
    tags: Vec<String>,
    keep: bool,
}

impl MPoly {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut poly = MPoly {
            points: Vec::new(),
            fill: false,
            width: draw_properties.width,
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
            keep: false,
        };
        let error_message = format!("Can't parse poly from string [{}]", s);
        for token in s.split_whitespace() {
            if token.starts_with("p=") {
                let mut iter = token[3..token.len() - 1].split(",");
                poly.points.push(Point::new(
                    iter.next().expect(&error_message).parse().expect(&error_message),
                    iter.next().expect(&error_message).parse().expect(&error_message),
                ));
            } else if token.starts_with("w=") {
                poly.width = token[2..].trim().parse().expect(&error_message);
            } else if token.starts_with("f=") {
                if token == "f=1" {
                    poly.fill = true;
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
                poly.color = Color::rgba8(r, g, b, a);
            } else if token.starts_with("t=") {
                poly.tags.push(String::from(token[2..].trim()));
            } else if token == "k" {
                poly.keep = true;
            }
        }
        poly
    }
}

impl Figure for MPoly {
    fn draw(&self, ctx: &mut PaintCtx, _scale: f64, transform: &dyn Fn(Point) -> Point) {
        let points = self.points.iter().map(|&x| transform(x)).collect::<Vec<_>>();
        let poly = Poly::from_vec(&points);
        if self.fill {
            ctx.fill(poly, &self.color);
        } else {
            ctx.stroke(poly, &self.color, self.width);
        }
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let color = self.color_to_string(&self.color);
        let points = self.points.iter().map(|&x| (params.transform)(x));
        let points = points
            .map(|p| format!("{},{}", p.x, params.size.height - p.y))
            .collect::<Vec<_>>()
            .join(" ");
        if self.fill {
            let poly = SvgPolygon::new()
                .set("points", points)
                .set("stroke-width", self.width * params.width_scale)
                .set("opacity", self.color.as_rgba().3 as f64)
                .set("fill", color);
            img.add(poly)
        } else {
            let poly = SvgPolyline::new()
                .set("points", points)
                .set("stroke-width", self.width * params.width_scale)
                .set("opacity", self.color.as_rgba().3 as f64)
                .set("stroke", color)
                .set("fill", "none");
            img.add(poly)
        }
    }

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }

    fn is_keep(&self) -> bool {
        self.keep
    }
}

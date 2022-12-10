use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::parse::Params;
use crate::poly::Poly;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::Point;

use svg::node::element::Polygon as SvgPolygon;
use svg::node::element::Polyline as SvgPolyline;
use svg::Document;

pub struct MPoly {
    points: Vec<Point>,
    fill: bool,
    width: f64,
    common: CommonParams,
}

impl MPoly {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let params = Params::from_str(s);
        Self {
            points: params.get("p").unwrap_or(Vec::new()),
            fill: params.get("f").unwrap_or(false),
            width: params.get("w").unwrap_or(draw_properties.width),
            common: CommonParams::new(&params, draw_properties),
        }
    }
}

impl Figure for MPoly {
    fn draw(&self, ctx: &mut PaintCtx, _scale: f64, transform: &dyn Fn(Point) -> Point) {
        let points = self.points.iter().map(|&x| transform(x)).collect::<Vec<_>>();
        let poly = Poly::from_vec(&points);
        if self.fill {
            ctx.fill(poly, &self.common.color);
        } else {
            ctx.stroke(poly, &self.common.color, self.width);
        }
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let color = self.color_to_string();
        let points = self.points.iter().map(|&x| (params.transform)(x));
        let points = points
            .map(|p| format!("{},{}", p.x, params.size.height - p.y))
            .collect::<Vec<_>>()
            .join(" ");
        if self.fill {
            let poly = SvgPolygon::new()
                .set("points", points)
                .set("stroke-width", self.width * params.width_scale)
                .set("opacity", self.common.color.as_rgba().3 as f64)
                .set("fill", color);
            img.add(poly)
        } else {
            let poly = SvgPolyline::new()
                .set("points", points)
                .set("stroke-width", self.width * params.width_scale)
                .set("opacity", self.common.color.as_rgba().3 as f64)
                .set("stroke", color)
                .set("fill", "none");
            img.add(poly)
        }
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }
}

use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::parse::Params;
use crate::svg_params::SvgParams;

use druid::kurbo::Line;
use druid::widget::prelude::*;
use druid::Point;

use svg::node::element::Line as SvgLine;
use svg::Document;

pub struct MLine {
    start: Point,
    finish: Point,
    width: f64,
    common: CommonParams,
}

impl MLine {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let params = Params::from_str(s);
        Self {
            start: params.get("s").unwrap_or(Point::new(0.0, 0.0)),
            finish: params.get("f").unwrap_or(Point::new(0.0, 0.0)),
            width: params.get("w").unwrap_or(draw_properties.width),
            common: CommonParams::new(&params, draw_properties),
        }
    }
}

impl Figure for MLine {
    fn draw(&self, ctx: &mut PaintCtx, _scale: f64, transform: &dyn Fn(Point) -> Point) {
        let start = transform(self.start);
        let finish = transform(self.finish);
        ctx.stroke(Line::new(start, finish), &self.common.color, self.width);
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
            .set("stroke", self.color_to_string())
            .set("opacity", self.common.color.as_rgba().3 as f64);
        img.add(line)
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }
}

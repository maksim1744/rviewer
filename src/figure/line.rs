use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::in_between::{interpolate, InBetweenProperties};
use crate::parse::Params;
use crate::svg_params::SvgParams;
use crate::transform::Transform;

use std::any::Any;

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

    pub fn in_betweens(a: &Self, b: &Self, in_between_properties: &InBetweenProperties) -> Vec<Self> {
        let func = b
            .common
            .func
            .as_ref()
            .and_then(|x| in_between_properties.funcs.get(x))
            .unwrap_or(&in_between_properties.func);
        (0..in_between_properties.frames - 1)
            .map(|i| func[i])
            .map(|k| Self {
                start: interpolate(&a.start, &b.start, k),
                finish: interpolate(&a.finish, &b.finish, k),
                width: interpolate(&a.width, &b.width, k),
                common: interpolate(&a.common, &b.common, k),
            })
            .collect()
    }
}

impl Figure for MLine {
    fn draw(&self, ctx: &mut PaintCtx, _scale: f64, transform: &Transform) {
        let start = transform.point(self.start);
        let finish = transform.point(self.finish);
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

    fn as_any(&self) -> &dyn Any {
        self
    }
}

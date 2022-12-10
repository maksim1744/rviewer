use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::in_between::{interpolate, InBetweenProperties};
use crate::parse::Params;
use crate::svg_params::SvgParams;

use std::any::Any;

use druid::kurbo::Circle;
use druid::widget::prelude::*;
use druid::Point;

use svg::node::element::Circle as SvgCircle;
use svg::Document;

#[derive(Clone)]
pub struct MCircle {
    center: Point,
    radius: f64,
    fill: bool,
    width: f64,
    common: CommonParams,
}

impl MCircle {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let params = Params::from_str(s);
        Self {
            center: params.get("c").unwrap_or(Point::new(0.0, 0.0)),
            radius: params.get("r").unwrap_or(1.0),
            fill: params.get("f").unwrap_or(false),
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
                center: interpolate(&a.center, &b.center, k),
                radius: interpolate(&a.radius, &b.radius, k),
                fill: a.fill,
                width: interpolate(&a.width, &b.width, k),
                common: interpolate(&a.common, &b.common, k),
            })
            .collect()
    }
}

impl Figure for MCircle {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point) {
        let center = transform(self.center);
        let mut r = self.radius;
        r *= scale;
        let circle = Circle::new(center, r);
        if self.fill {
            ctx.fill(circle, &self.common.color);
        } else {
            ctx.stroke(circle, &self.common.color, self.width);
        }
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let center = (params.transform)(self.center);
        let color = self.color_to_string();
        let mut circ = SvgCircle::new()
            .set("cx", center.x)
            .set("cy", params.size.height - center.y)
            .set("r", self.radius)
            .set("stroke-width", self.width * params.width_scale)
            .set("opacity", self.common.color.as_rgba().3 as f64);
        if self.fill {
            circ = circ.set("fill", color);
        } else {
            circ = circ.set("fill", "none").set("stroke", color);
        }
        img.add(circ)
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

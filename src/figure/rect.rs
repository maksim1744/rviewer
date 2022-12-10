use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::in_between::{interpolate, InBetweenProperties};
use crate::parse::Params;
use crate::svg_params::SvgParams;

use std::any::Any;

use druid::widget::prelude::*;
use druid::{Point, Rect};

use svg::node::element::Rectangle as SvgRect;
use svg::Document;

pub struct MRect {
    center: Point,
    size: Point,
    fill: bool,
    width: f64,
    alignment: (char, char),
    common: CommonParams,
}

impl MRect {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let params = Params::from_str(s);
        Self {
            center: params.get("c").unwrap_or(Point::new(0.0, 0.0)),
            size: params.get("s").unwrap_or(Point::new(0.0, 0.0)),
            fill: params.get("f").unwrap_or(false),
            width: params.get("w").unwrap_or(draw_properties.width),
            alignment: params.get("a").unwrap_or(('C', 'C')),
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
                size: interpolate(&a.size, &b.size, k),
                fill: a.fill,
                width: interpolate(&a.width, &b.width, k),
                alignment: a.alignment,
                common: interpolate(&a.common, &b.common, k),
            })
            .collect()
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
            ctx.fill(rect, &self.common.color);
        } else {
            ctx.stroke(rect, &self.common.color, self.width);
        }
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let color = self.color_to_string();
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
            .set("opacity", self.common.color.as_rgba().3 as f64);
        if self.fill {
            rect = rect.set("fill", color);
        } else {
            rect = rect.set("fill", "none").set("stroke", color);
        }
        img.add(rect)
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

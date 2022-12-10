use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::in_between::{interpolate, InBetweenProperties};
use crate::parse::Params;
use crate::svg_params::SvgParams;

use std::any::Any;

use druid::kurbo::Line;
use druid::widget::prelude::*;
use druid::Point;

use svg::node::element::Line as SvgLine;
use svg::Document;

pub struct MGrid {
    center: Point,
    size: Point,
    dims: (usize, usize),
    width: f64,
    alignment: (char, char),
    common: CommonParams,
}

impl MGrid {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let params = Params::from_str(s);
        Self {
            center: params.get("c").unwrap_or(Point::new(0.0, 0.0)),
            size: params.get("s").unwrap_or(Point::new(0.0, 0.0)),
            dims: params.get("d").unwrap_or((1, 1)),
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
                dims: interpolate(&a.dims, &b.dims, k),
                width: interpolate(&a.width, &b.width, k),
                alignment: a.alignment,
                common: interpolate(&a.common, &b.common, k),
            })
            .collect()
    }
}

impl Figure for MGrid {
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

        for i in 0..self.dims.0 + 1 {
            ctx.stroke(
                Line::new(
                    Point::new(center.x - size.x / 2. + size.x / self.dims.0 as f64 * i as f64, center.y - size.y / 2.),
                    Point::new(center.x - size.x / 2. + size.x / self.dims.0 as f64 * i as f64, center.y + size.y / 2.),
                ),
                &self.common.color,
                self.width,
            );
        }
        for i in 0..self.dims.1 + 1 {
            ctx.stroke(
                Line::new(
                    Point::new(center.x - size.x / 2., center.y - size.y / 2. + size.y / self.dims.1 as f64 * i as f64),
                    Point::new(center.x + size.x / 2., center.y - size.y / 2. + size.y / self.dims.1 as f64 * i as f64),
                ),
                &self.common.color,
                self.width,
            );
        }
    }

    fn draw_on_image(&self, mut img: Document, params: &SvgParams) -> Document {
        let mut center = self.center.clone();
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
        for i in 0..self.dims.0 + 1 {
            let line = SvgLine::new()
                .set("x1", center.x - self.size.x / 2. + self.size.x / self.dims.0 as f64 * i as f64)
                .set("y1", params.size.height - (center.y - self.size.y / 2.))
                .set("x2", center.x - self.size.x / 2. + self.size.x / self.dims.0 as f64 * i as f64)
                .set("y2", params.size.height - (center.y + self.size.y / 2.))
                .set("stroke-width", self.width * params.width_scale)
                .set("stroke", self.color_to_string())
                .set("opacity", self.common.color.as_rgba().3 as f64);
            img = img.add(line);
        }
        for i in 0..self.dims.1 + 1 {
            let line = SvgLine::new()
                .set("x1", center.x - self.size.x / 2.)
                .set(
                    "y1",
                    params.size.height - (center.y - self.size.y / 2. + self.size.y / self.dims.1 as f64 * i as f64),
                )
                .set("x2", center.x + self.size.x / 2.)
                .set(
                    "y2",
                    params.size.height - (center.y - self.size.y / 2. + self.size.y / self.dims.1 as f64 * i as f64),
                )
                .set("stroke-width", self.width * params.width_scale)
                .set("stroke", self.color_to_string())
                .set("opacity", self.common.color.as_rgba().3 as f64);
            img = img.add(line);
        }

        img
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

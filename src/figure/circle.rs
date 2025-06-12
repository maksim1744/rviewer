use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::in_between::{interpolate, InBetweenProperties};
use crate::parse::Params;
use crate::svg_params::SvgParams;
use crate::transform::Transform;

use std::any::Any;
use std::f64::consts::PI;

use druid::kurbo::Circle;
use druid::widget::prelude::*;
use druid::Point;

use svg::node::element::{path::Data, Circle as SvgCircle, Path};
use svg::Document;

#[derive(Clone)]
pub struct MCircle {
    center: Point,
    radius: f64,
    fill: bool,
    width: f64,
    arc: Option<(f64, f64)>,
    common: CommonParams,
}

impl MCircle {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let params = Params::from_str(s);
        let fix_ang = |mut a: f64| -> f64 {
            while a <= -PI {
                a += PI * 2.0;
            }
            while a > PI {
                a -= PI * 2.0;
            }
            a
        };
        Self {
            center: params.get("c").unwrap_or(Point::new(0.0, 0.0)),
            radius: params.get("r").unwrap_or(1.0),
            fill: params.get("f").unwrap_or(false),
            width: params.get("w").unwrap_or(draw_properties.width),
            arc: params.get("arc").map(|(fr, to)| (fix_ang(fr), fix_ang(to))),
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
                arc: match (a.arc.as_ref(), b.arc.as_ref()) {
                    (Some(a), Some(b)) => Some(interpolate(a, b, k)),
                    _ => None,
                },
            })
            .collect()
    }

    fn get_flipped_arc(&self, flipy: bool) -> Option<(f64, f64)> {
        self.arc.map(|(mut fr, mut to)| {
            if !flipy {
                std::mem::swap(&mut fr, &mut to);
                fr = -fr;
                to = -to;
            }
            (fr, to)
        })
    }
}

impl Figure for MCircle {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &Transform) {
        let center = transform.point(self.center);
        let mut r = self.radius;
        r *= scale;
        let circle = Circle::new(center, r);
        match self.get_flipped_arc(transform.flipy()) {
            None => {
                if self.fill {
                    ctx.fill(circle, &self.common.color);
                } else {
                    ctx.stroke(circle, &self.common.color, self.width);
                }
            }
            Some((fr, to)) => {
                let mut diff = to - fr;
                if diff < 0.0 {
                    diff += PI * 2.0;
                }
                if self.fill {
                    let circle = circle.segment(0.0, fr, diff);
                    ctx.fill(circle, &self.common.color);
                } else {
                    let circle = circle.segment(r, fr, diff);
                    ctx.stroke(circle, &self.common.color, self.width);
                }
            }
        }
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let center = (params.transform)(self.center);
        let color = self.color_to_string();
        let opacity = self.common.color.as_rgba().3 as f64;
        let radius = self.radius;
        let stroke_width = self.width * params.width_scale;
        let cx = center.x;
        let cy = params.size.height - center.y;

        match self.get_flipped_arc(params.flipy) {
            None => {
                let mut circ = SvgCircle::new()
                    .set("cx", cx)
                    .set("cy", cy)
                    .set("r", radius)
                    .set("stroke-width", stroke_width)
                    .set("opacity", opacity);

                if self.fill {
                    circ = circ.set("fill", color);
                } else {
                    circ = circ.set("fill", "none").set("stroke", color);
                }

                img.add(circ)
            }
            Some((fr, to)) => {
                let sx = cx + radius * fr.cos();
                let sy = cy + radius * fr.sin();
                let ex = cx + radius * to.cos();
                let ey = cy + radius * to.sin();

                let start_point = (sx, sy);
                let end_point = (ex, ey);
                let center_svg = (cx, cy);

                let delta = (to - fr + PI * 2.0) % (2.0 * PI);
                let large_arc = if delta > PI { 1 } else { 0 };
                let sweep = 1;

                let mut data = Data::new().move_to(start_point);
                data = data.elliptical_arc_to((radius, radius, 0.0, large_arc, sweep, end_point.0, end_point.1));

                if self.fill {
                    data = data.line_to(center_svg).close();
                }

                let mut path = Path::new().set("d", data).set("opacity", opacity);

                if self.fill {
                    path = path.set("fill", color).set("stroke", "none");
                } else {
                    path = path.set("fill", "none").set("stroke", color).set("stroke-width", stroke_width);
                }

                img.add(path)
            }
        }
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

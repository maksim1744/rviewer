// stolen directly from druid::widget::Slider and adjusted to my needs


// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::app_data::AppData;

use druid::widget::prelude::*;
use druid::{Size, Rect, Point};
use druid::kurbo::{Circle, RoundedRect};
use druid::{theme, LinearGradient, UnitPoint};
use druid::piet::kurbo::Shape;


pub struct ISlider {
    min: usize,
    max: usize,
    knob_pos: Point,
    knob_hovered: bool,
    x_offset: f64,
}

impl ISlider {
    /// Create a new `ISlider`.
    pub fn new() -> ISlider {
        ISlider {
            min: 0,
            max: 1,
            knob_pos: Default::default(),
            knob_hovered: Default::default(),
            x_offset: Default::default(),
        }
    }

    /// Builder-style method to set the range covered by this slider.
    ///
    /// The default range is `0.0..1.0`.
    pub fn with_range(mut self, min: usize, max: usize) -> Self {
        self.min = min;
        self.max = max;
        self
    }
}

const TRACK_THICKNESS: f64 = 4.0;
const BORDER_WIDTH: f64 = 2.0;
const KNOB_STROKE_WIDTH: f64 = 2.0;

impl ISlider {
    fn knob_hit_test(&self, knob_width: f64, mouse_pos: Point) -> bool {
        let knob_circle = Circle::new(self.knob_pos, knob_width / 2.);
        knob_circle.winding(mouse_pos) > 0
    }

    fn calculate_value(&self, mouse_x: f64, knob_width: f64, slider_width: f64) -> usize {
        let scalar = ((mouse_x + self.x_offset - knob_width / 2.) / (slider_width - knob_width))
            .max(0.0)
            .min(1.0);
        (self.min as f64 + scalar * (self.max as f64 - self.min as f64)).round() as usize
    }

    fn normalize(&self, data: usize) -> f64 {
        ((data as f64).max(self.min as f64).min(self.max as f64) - self.min as f64) / (self.max as f64 - self.min as f64).max(1.0)
    }
}

impl Widget<AppData> for ISlider {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        let knob_size = env.get(theme::BASIC_WIDGET_HEIGHT);
        let slider_width = ctx.size().width;

        match event {
            Event::MouseDown(mouse) => {
                ctx.set_active(true);
                if self.knob_hit_test(knob_size, mouse.pos) {
                    self.x_offset = self.knob_pos.x - mouse.pos.x
                } else {
                    self.x_offset = 0.;
                    data.frame = self.calculate_value(mouse.pos.x, knob_size, slider_width);
                }
                ctx.request_paint();
            }
            Event::MouseUp(mouse) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    data.frame = self.calculate_value(mouse.pos.x, knob_size, slider_width);
                    ctx.request_paint();
                }
            }
            Event::MouseMove(mouse) => {
                if ctx.is_active() {
                    data.frame = self.calculate_value(mouse.pos.x, knob_size, slider_width);
                    ctx.request_paint();
                }
                if ctx.is_hot() {
                    let knob_hover = self.knob_hit_test(knob_size, mouse.pos);
                    if knob_hover != self.knob_hovered {
                        self.knob_hovered = knob_hover;
                        ctx.request_paint();
                    }
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &AppData, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppData, data: &AppData, _env: &Env) {
        self.max = data.frames.lock().unwrap().len().max(1) - 1;
        ctx.request_paint();
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppData, env: &Env) -> Size {
        bc.debug_check("ISlider");
        let height = env.get(theme::BASIC_WIDGET_HEIGHT);
        let width = env.get(theme::WIDE_WIDGET_WIDTH);
        let baseline_offset = (height / 2.0) - TRACK_THICKNESS;
        ctx.set_baseline_offset(baseline_offset);
        bc.constrain((width, height))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, env: &Env) {
        let clamped = self.normalize(data.frame);
        let rect = ctx.size().to_rect();
        let knob_size = env.get(theme::BASIC_WIDGET_HEIGHT);

        //Paint the background
        let background_width = rect.width() - knob_size;
        let background_origin = Point::new(knob_size / 2., (knob_size - TRACK_THICKNESS) / 2.);
        let background_size = Size::new(background_width, TRACK_THICKNESS);
        let background_rect = Rect::from_origin_size(background_origin, background_size)
            .inset(-BORDER_WIDTH / 2.)
            .to_rounded_rect(2.);

        let background_gradient = LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            (
                env.get(theme::BACKGROUND_LIGHT),
                env.get(theme::BACKGROUND_DARK),
            ),
        );

        ctx.stroke(background_rect, &env.get(theme::BORDER_DARK), BORDER_WIDTH);

        ctx.fill(background_rect, &background_gradient);

        //Get ready to paint the knob
        let is_active = ctx.is_active();
        let is_hovered = self.knob_hovered;

        let knob_position = (rect.width() - knob_size) * clamped + knob_size / 2.;
        self.knob_pos = Point::new(knob_position, knob_size / 2.);
        let knob_rect_size = (knob_size - KNOB_STROKE_WIDTH) * 0.6;
        let knob_rect = RoundedRect::from_rect(
            Rect::from_center_size(self.knob_pos, Size::new(knob_rect_size, knob_rect_size)), knob_rect_size / 5.0);

        let normal_knob_gradient = LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            (
                env.get(theme::FOREGROUND_LIGHT),
                env.get(theme::FOREGROUND_DARK),
            ),
        );
        let flipped_knob_gradient = LinearGradient::new(
            UnitPoint::TOP,
            UnitPoint::BOTTOM,
            (
                env.get(theme::FOREGROUND_DARK),
                env.get(theme::FOREGROUND_LIGHT),
            ),
        );

        let knob_gradient = if is_active {
            flipped_knob_gradient
        } else {
            normal_knob_gradient
        };

        //Paint the border
        let border_color = if is_hovered || is_active {
            env.get(theme::FOREGROUND_LIGHT)
        } else {
            env.get(theme::FOREGROUND_DARK)
        };

        ctx.stroke(knob_rect, &border_color, KNOB_STROKE_WIDTH);

        //Actually paint the knob
        ctx.fill(knob_rect, &knob_gradient);
    }
}

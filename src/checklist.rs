use std::sync::{Arc, Mutex};

use druid::kurbo::{BezPath, Size};
use druid::piet::{LineCap, LineJoin, RenderContext, StrokeStyle};
use druid::theme;
use druid::widget::prelude::*;
use druid::{Color, Point, Rect};

use druid::{Command, Target, Selector};

use druid::piet::{FontFamily, Text, TextLayoutBuilder, TextAlignment};

const HEIGHT_KOEF: f64 = 1.2;

pub struct Checklist {
    width: f64,
    selected: Option<usize>,
    notify_widget: Option<WidgetId>,
}

impl Checklist {
    pub fn new(notify_widget: Option<WidgetId>) -> Checklist {
        Checklist {
            width: 100.0,
            selected: Some(0),
            notify_widget: notify_widget,
        }
    }

    fn get_selected_index(&self, y: f64, cellh: f64, n: usize) -> Option<usize> {
        if y <= 0.0 || y >= cellh * n as f64 * HEIGHT_KOEF {
            return None;
        }
        let candidate: usize = (y / (cellh * HEIGHT_KOEF)).floor() as usize;
        let offset = (y - cellh * HEIGHT_KOEF * (candidate as f64 + 0.5)).abs() / cellh;
        if offset <= 0.5 {
            Some(candidate)
        } else {
            None
        }
    }
}

impl Widget<Arc<Mutex<Vec<(String, bool)>>>> for Checklist {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Arc<Mutex<Vec<(String, bool)>>>, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(e) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        self.selected = self.get_selected_index(e.pos.y, env.get(theme::BASIC_WIDGET_HEIGHT), data.lock().unwrap().len());
                        if let Some(ind) = self.selected {
                            if data.lock().unwrap()[ind].1 {
                                data.lock().unwrap()[ind].1 = false;
                            } else {
                                data.lock().unwrap()[ind].1 = true;
                            }
                        }
                        if let Some(id) = self.notify_widget {
                            ctx.submit_command(Command::new(Selector::new("update"), (), Target::Widget(id)));
                        }
                    }
                    ctx.request_paint();
                }
            }
            Event::MouseMove(e) => {
                let new_selected = self.get_selected_index(e.pos.y, env.get(theme::BASIC_WIDGET_HEIGHT), data.lock().unwrap().len());
                if new_selected != self.selected {
                    self.selected = new_selected;
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &Arc<Mutex<Vec<(String, bool)>>>, _env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Arc<Mutex<Vec<(String, bool)>>>, _data: &Arc<Mutex<Vec<(String, bool)>>>, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, _bc: &BoxConstraints, data: &Arc<Mutex<Vec<(String, bool)>>>, env: &Env) -> Size {
        Size::new(self.width, env.get(theme::BASIC_WIDGET_HEIGHT) * data.lock().unwrap().len() as f64 * HEIGHT_KOEF)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Arc<Mutex<Vec<(String, bool)>>>, env: &Env) {
        let size = env.get(theme::BASIC_WIDGET_HEIGHT);
        let border_width = 1.;

        let mut position = size * (HEIGHT_KOEF - 1.0) / 2.0;
        let mut ind: usize = 0;
        for (key, val) in data.lock().unwrap().iter() {
            let rect =
                Rect::from_origin_size(Point::new(0.0, position), Size::new(size, size))
                .inset(-border_width / 2.)
                .to_rounded_rect(2.);


            let border_color = if ctx.is_hot() && self.selected == Some(ind) {
                env.get(theme::BORDER_LIGHT)
            } else {
                env.get(theme::BORDER_DARK)
            };

            ctx.stroke(rect, &border_color, border_width);

            if *val {
                // Paint the checkmark
                let mut path = BezPath::new();
                path.move_to((4.0, position + 9.0));
                path.line_to((8.0, position + 13.0));
                path.line_to((14.0, position + 5.0));

                let style = StrokeStyle::new()
                    .line_cap(LineCap::Round)
                    .line_join(LineJoin::Round);

                ctx.stroke_styled(path, &env.get(theme::TEXT_COLOR), 2., &style);
            }

            let text = ctx.text();
            let layout = text
                .new_text_layout(key.clone())
                .font(FontFamily::SANS_SERIF, size * 0.7)
                .text_color(Color::rgb8(0xff, 0xff, 0xff))
                .alignment(TextAlignment::Start)
                .build()
                .unwrap();

            ctx.draw_text(&layout, Point::new(size + size * 0.2, position));

            position += size * HEIGHT_KOEF;
            ind += 1;
        }
    }
}

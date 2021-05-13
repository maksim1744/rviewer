use std::sync::{Arc, Mutex};
use std::env;
use std::thread;

use std::time::Duration;

use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};

use std::collections::HashSet;

use druid::widget::prelude::*;
use druid::widget::{Flex, Widget, MainAxisAlignment, CrossAxisAlignment, SizedBox, Label, Align};
use druid::{Size, AppLauncher, WindowDesc, Point, WidgetExt, MouseButton, TimerToken};
use druid::{MenuDesc, MenuItem, LocalizedString, Selector};
use druid::Code;

use svg::Document;
use svg::node::element::Rectangle as SvgRect;
mod svg_params;
use svg_params::SvgParams;

mod app_data;
mod figure;
mod islider;
mod poly;
mod checklist;
use checklist::Checklist;

use islider::ISlider;
use app_data::*;

const PADDING: f64 = 8.0;

struct DrawingWidget {
    scale: f64,
    center: Point,
    size: Size,
    last_mouse_pos: Point,
    mouse_down: bool,
    timer_id: TimerToken,
    running: bool,
    last_data_size: Size,
}

impl DrawingWidget {
    fn transform(&self, mut p: Point) -> Point {
        p.x = (p.x - self.center.x) * self.scale + self.size.width / 2.0;
        p.y = (p.y - self.center.y) * self.scale + self.size.height / 2.0;
        p
    }

    fn inv_transform(&self, mut p: Point) -> Point {
        p.x = (p.x - self.size.width / 2.0) / self.scale + self.center.x;
        p.y = (p.y - self.size.height / 2.0) / self.scale + self.center.y;
        p
    }

    fn internal_save_frame(&self, data: &AppData, frame: usize, file_name: String) {
        let size = data.size.lock().unwrap().clone();

        let mut img = Document::new()
            .set("viewBox", (0, 0, size.width, size.height))
            .set("width", size.width)
            .set("height", size.height);

        let enabled_tags = data.tags.lock().unwrap().iter().filter(|(_, b)| *b).map(|(tag, _)| tag.clone()).collect::<HashSet<String>>();

        let frame = &data.frames.lock().unwrap()[frame];

        let rect = SvgRect::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", size.width)
            .set("height", size.height)
            .set("fill", "rgb(41, 41, 41)");
        img = img.add(rect);

        let params = SvgParams {
            size: size.clone(),
            width_scale: data.svg_width_scale,
        };

        for ind in frame.iter() {
            let item = &data.objects.lock().unwrap()[*ind];
            if item.need_to_draw(&enabled_tags) {
                img = item.draw_on_image(img, &params);
            }
        }

        svg::save(file_name, &img).unwrap();
    }

    fn save_frame(&self, data: &AppData) {
        if data.frame >= data.frames.lock().unwrap().len() {
            return;
        }

        self.internal_save_frame(data, data.frame, "frame.svg".to_string());
    }

    fn save_all_frames(&self, data: &AppData) {
        fs::create_dir_all("frames").unwrap();

        let total_frames = data.frames.lock().unwrap().len();
        for frame in 0..total_frames {
            self.internal_save_frame(data, frame, format!("frames/{:05}.svg", frame + 1));
        }
    }
}

impl Widget<AppData> for DrawingWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppData, _env: &Env) {
        ctx.request_focus();
        match event {
            Event::MouseMove(e) => {
                if e.buttons.contains(MouseButton::Left) && self.mouse_down {
                    self.center.x -= (e.pos.x - self.last_mouse_pos.x) / self.scale;
                    self.center.y -= (e.pos.y - self.last_mouse_pos.y) / self.scale;
                    self.last_mouse_pos = e.pos;
                    ctx.request_paint();
                }
            },
            Event::Wheel(e) => {
                let new_scale = self.scale * 0.01_f64.max(1.1_f64.powf(-e.wheel_delta.y / 50.0));

                let mouse_was = self.inv_transform(e.pos);
                self.scale = new_scale;
                let mouse_now = self.inv_transform(e.pos);

                self.center.x -= mouse_now.x - mouse_was.x;
                self.center.y -= mouse_now.y - mouse_was.y;

                ctx.request_paint();
            },
            Event::MouseDown(e) => {
                self.mouse_down = true;
                self.last_mouse_pos = e.pos.clone();
            },
            Event::MouseUp(_) => {
                self.mouse_down = false;
            },
            Event::KeyDown(e) => {
                match e.code {
                    Code::ArrowRight => {
                        if data.frame + 1 < data.frames.lock().unwrap().len() {
                            data.frame += 1;
                            ctx.request_paint();
                        }
                    },
                    Code::ArrowLeft => {
                        if data.frame != 0 {
                            data.frame -= 1;
                            ctx.request_paint();
                        }
                    },
                    Code::Space => {
                        if self.running {
                            self.running = false;
                            self.timer_id = TimerToken::INVALID;
                        } else {
                            self.running = true;
                            self.timer_id = ctx.request_timer(Duration::from_secs_f64(*data.fps_speed.lock().unwrap()));
                        }
                    },
                    Code::Digit0 => {
                        self.last_data_size = Size::new(0.0, 0.0);
                        ctx.request_paint();
                    },
                    _ => (),
                }
            },
            Event::Timer(id) => {
                if *id == self.timer_id {
                    if data.frame + 1 < data.frames.lock().unwrap().len() {
                        data.frame += 1;
                        self.timer_id = ctx.request_timer(Duration::from_secs_f64(*data.fps_speed.lock().unwrap()));
                        ctx.request_paint();
                    } else if !*data.finished.lock().unwrap() {
                        self.timer_id = ctx.request_timer(Duration::from_secs_f64(*data.fps_speed.lock().unwrap()));
                    } else {
                        self.running = false;
                        self.timer_id = TimerToken::INVALID;
                    }
                }
            },
            Event::Command(c) => {
                if c.is::<()>(Selector::new("save_frame")) {
                    self.save_frame(data);
                } else if c.is::<()>(Selector::new("save_all_frames")) {
                    self.save_all_frames(data);
                }
                ctx.request_paint();
            },
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppData,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppData,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &Env) {
        self.size = ctx.size();

        let data_size = data.size.lock().unwrap().clone();

        if data_size != self.last_data_size {
            self.center = Point::new(data_size.width / 2.0, data_size.height / 2.0);
            self.scale = (self.size.height / data_size.height)
                     .min(self.size.width / data_size.width) * 0.9;
            self.last_data_size = data_size;
        }

        let transform = |mut p : Point| -> Point {
            p.y = data_size.height - p.y;
            self.transform(p)
        };

        let enabled_tags = data.tags.lock().unwrap().iter().filter(|(_, b)| *b).map(|(tag, _)| tag.clone()).collect::<HashSet<String>>();

        if data.frame < data.frames.lock().unwrap().len() {
            let frame = &data.frames.lock().unwrap()[data.frame];
            for ind in frame.iter() {
                let item = &data.objects.lock().unwrap()[*ind];
                if item.need_to_draw(&enabled_tags) {
                    item.draw(ctx, self.scale, &transform);
                }
            }
        }

        // let text = ctx.text();
        // let layout = text
        //     .new_text_layout(format!("{:?}", data.tags))
        //     .font(FontFamily::SERIF, 15.0)
        //     .text_color(Color::rgb8(0xff, 0xff, 0xff))
        //     // .alignment(TextAlignment::Start)
        //     .build()
        //     .unwrap();

        // ctx.draw_text(&layout, Point::new(10.0, 10.0));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    let mut init_frames: Vec<usize> = Vec::new();

    let objects = Arc::new(Mutex::new(Vec::new()));
    let objects_ptr = objects.clone();

    let frames = Arc::new(Mutex::new(Vec::new()));
    let frames_ptr = frames.clone();

    let tags = Arc::new(Mutex::new(Vec::new()));
    let tags_ptr = tags.clone();

    let draw_properties = Arc::new(Mutex::new(DrawProperties {
        width: 1.0,
        font: 1.0,
        was_messages: 0,
    }));
    let draw_properties_ptr = draw_properties.clone();

    let fps_speed = Arc::new(Mutex::new(0.033));
    let fps_speed_ptr = fps_speed.clone();

    let size = Arc::new(Mutex::new(Size::new(10.0, 10.0)));
    let size_ptr = size.clone();

    let finished = Arc::new(Mutex::new(false));
    let finished_ptr = finished.clone();

    let handle = thread::spawn(move || {
        let app_data = AppData {
            objects: objects_ptr,
            frames: frames_ptr,
            frame: 0,
            fps_speed: fps_speed_ptr,
            size: size_ptr,
            tags: tags_ptr,
            draw_properties: draw_properties_ptr,
            svg_width_scale: 0.3,
            finished: finished_ptr,
        };

        let window = WindowDesc::new(make_layout)
            .window_size(Size {
                width: 800.0,
                height: 600.0,
            })
            .menu(MenuDesc::new(LocalizedString::new("my title"))
                .append(
                    MenuItem::new(LocalizedString::new("Save frame"), Selector::new("save_frame")))
                .append(
                    MenuItem::new(LocalizedString::new("Save all frames"), Selector::new("save_all_frames")))
                )
            .resizable(true)
            .title("Viewer");
        AppLauncher::with_window(window)
            .launch(app_data)
            .expect("launch failed");
    });

    let iter: Box<dyn BufRead> = if args.len() > 0 {
        Box::new(io::BufReader::new(File::open(&args[0]).unwrap()))
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    let mut tags_set: HashSet<String> = HashSet::new();
    let mut disabled_tags: HashSet<String> = HashSet::new();

    let mut unparsed = 0;

    let mut last_frame = Vec::new();
    let mut is_initial_tick = true;


    for line in iter.lines() {
        let line = line.unwrap();
        if line.starts_with("tick") {
            if is_initial_tick {
                init_frames = last_frame.clone();
            } else {
                frames.lock().unwrap().push(last_frame.clone());
            }
            last_frame = init_frames.clone();
            print!("\rreading tick {}", frames.lock().unwrap().len() + 1);
            draw_properties.lock().unwrap().was_messages = 0;
            io::stdout().flush().unwrap();
            is_initial_tick = false;
        } else if line.starts_with("speed") {
            *fps_speed.lock().unwrap() = 1.0 / line[6..].trim().parse::<f64>().unwrap();
        } else if line.starts_with("width") {
            draw_properties.lock().unwrap().width = line[6..].trim().parse().unwrap();
        } else if line.starts_with("font") {
            draw_properties.lock().unwrap().font = line[5..].trim().parse().unwrap();
        } else if line.starts_with("size") {
            let line = line.trim();
            let mut iter = line.trim()[6..line.len() - 1].split(",");
            size.lock().unwrap().width = iter.next().unwrap().parse().unwrap();
            size.lock().unwrap().height = iter.next().unwrap().parse().unwrap();
        } else if line.starts_with("disable ") {
            let dtag = line[8..].trim().to_string();
            for (tag, b) in tags.lock().unwrap().iter_mut() {
                if *tag == dtag {
                    *b = false;
                }
            }
            disabled_tags.insert(dtag);
        } else  {
            match figure::from_string(&line, &mut draw_properties.lock().unwrap()) {
                Some(x) => {
                    for tag in x.get_tags() {
                        if !tags_set.contains(tag) {
                            tags_set.insert(tag.clone());
                            tags.lock().unwrap().push((tag.clone(), !disabled_tags.contains(tag)));
                        }
                    }
                    if !frames.lock().unwrap().is_empty() {
                        last_frame.push(objects.lock().unwrap().len());
                    } else {
                        last_frame.push(objects.lock().unwrap().len());
                    }
                    objects.lock().unwrap().push(x);
                },
                None => unparsed += 1,
            };
        }
    }
    frames.lock().unwrap().push(last_frame.clone());

    *finished.lock().unwrap() = true;

    println!("");
    println!("unparsed: {}", unparsed);

    // app_data.objects = Rc::new(objects);
    // app_data.frame = app_data.frame.min(frames.len() - 1);
    // app_data.frames = Rc::new(frames);
    // let mut tags = tags.iter().map(|x| (x.clone(), true)).collect::<Vec<_>>();
    // tags.sort();
    // app_data.tags = Arc::new(Mutex::new(tags));

    handle.join().unwrap();
}

fn make_layout() -> impl Widget<AppData> {
    let drawing_widget_id = WidgetId::next();

    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_flex_child(
                    DrawingWidget {
                        scale: 1.0,
                        center: Point::new(0.0, 0.0),
                        size: Size::new(1.0, 1.0),
                        last_mouse_pos: Point::new(0.0, 0.0),
                        mouse_down: false,
                        timer_id: TimerToken::INVALID,
                        running: false,
                        last_data_size: Size::new(0.0, 0.0),
                    }.with_id(drawing_widget_id),
                    1.0
                )
                .with_spacer(PADDING)
                .with_child(
                    Checklist::new(Some(drawing_widget_id)).lens(AppData::tags)
                )
                .cross_axis_alignment(CrossAxisAlignment::Start),
            1.0
        )
        .with_spacer(PADDING)
        .with_child(
            Flex::row()
                .with_flex_child(
                    SizedBox::new(ISlider::new().with_range(0, 10)).expand_width(),
                    1.0
                )
                .with_child(
                    SizedBox::new(
                        Align::right(
                            Label::new(|data: &AppData, _env: &_| format!("{} / {}", data.frame + 1, data.frames.lock().unwrap().len()))
                        )
                    ).width(100.0)
                )
        )
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::End)
        .padding(PADDING)
}

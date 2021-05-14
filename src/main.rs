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
use druid::{Menu, MenuItem};
use druid::{Command, Selector, Target};
use druid::Code;
use druid::WindowId;

use svg::Document;
use svg::node::element::Rectangle as SvgRect;
mod svg_params;
use svg_params::SvgParams;

use threadpool::ThreadPool;

use subprocess::{Popen, PopenConfig, Redirection};

mod settings;
use settings::Settings;

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

    fn internal_save_frame_as_svg(&self, data: &AppData, frame: usize, file_name: &str) {
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
            width_scale: *data.svg_width_scale.lock().unwrap(),
        };

        for ind in frame.iter() {
            let item = &data.objects.lock().unwrap()[*ind];
            if item.need_to_draw(&enabled_tags) {
                img = item.draw_on_image(img, &params);
            }
        }

        svg::save(file_name, &img).unwrap();
    }

    fn internal_save_frame_as_png(&self, data: &AppData, frame: usize, file_name: &str) -> std::thread::JoinHandle<()> {
        let svg_file_name = format!("_tmp_frame_{}_.svg", frame);
        self.internal_save_frame_as_svg(data, frame, &svg_file_name);
        let settings = get_settings();

        let png_file = Arc::new(Mutex::new(file_name.to_string().clone()));
        let svg_file = Arc::new(Mutex::new(svg_file_name.to_string().clone()));

        if settings.conversion_tool.as_ref().unwrap() == "rsvg-convert" {
            thread::spawn(move || {
                std::process::Command::new("rsvg-convert".to_string())
                        .arg(&*svg_file.lock().unwrap())
                        .arg("-o")
                        .arg(&*png_file.lock().unwrap())
                        .arg("-w")
                        .arg(settings.frame_resolution.unwrap().to_string())
                        .output().unwrap();
                fs::remove_file(&*svg_file.lock().unwrap()).unwrap()
            })
        } else {
            thread::spawn(move || {
                std::process::Command::new(settings.inkscape_path.unwrap())
                        .arg("-o")
                        .arg(&*png_file.lock().unwrap())
                        .arg("-w")
                        .arg(settings.frame_resolution.unwrap().to_string())
                        .arg(&*svg_file.lock().unwrap())
                        .output().unwrap();
                fs::remove_file(&*svg_file.lock().unwrap()).unwrap()
            })
        }
    }

    fn save_frame_as_svg(&self, data: &AppData) {
        if data.frame >= data.frames.lock().unwrap().len() {
            return;
        }

        let frame = data.frame;
        self.internal_save_frame_as_svg(data, frame, "frame.svg");
        println!("Saved frame {} as frame.svg", frame + 1);
    }

    fn save_all_frames_as_svg(&self, data: &AppData) {
        fs::create_dir_all("frames").unwrap();

        let total_frames = data.frames.lock().unwrap().len();
        for frame in 0..total_frames {
            print!("\rSaving frame {}/{}", frame + 1, total_frames);
            io::stdout().flush().unwrap();
            self.internal_save_frame_as_svg(data, frame, &format!("frames/{:05}.svg", frame + 1));
        }
        println!("\r{} frames saved in folder frames", total_frames);
    }

    fn save_frame_as_png(&self, data: &AppData) {
        if data.frame >= data.frames.lock().unwrap().len() {
            return;
        }

        let frame = data.frame;
        let handle = self.internal_save_frame_as_png(data, frame, "frame.png");
        thread::spawn(move || {
            handle.join().unwrap();
            println!("Saved frame {} as frame.png", frame + 1);
        });
    }

    fn save_all_frames_as_png(&self, data: &AppData) {
        fs::create_dir_all("frames").unwrap();

        let total_frames = data.frames.lock().unwrap().len();
        let settings = get_settings();
        let pool = ThreadPool::new(settings.max_threads.unwrap());
        for frame in 0..total_frames {
            let svg_file = format!("frames/_tmp_frame_{}_.svg", frame + 1);
            self.internal_save_frame_as_svg(data, frame, &svg_file);

            print!("\rCreated svg {}/{}", frame + 1, total_frames);
            io::stdout().flush().unwrap();
        }

        for frame in 0..total_frames {
            let svg_file = format!("frames/_tmp_frame_{}_.svg", frame + 1);
            let png_file = format!("frames/{:05}.png", frame + 1);
            let inkscape_path = settings.inkscape_path.clone().unwrap();
            let frame_resolution = settings.frame_resolution.unwrap();

            if settings.conversion_tool.as_ref().unwrap() == "rsvg-convert" {
                pool.execute(move || {
                    std::process::Command::new("rsvg-convert".to_string())
                            .arg(&svg_file)
                            .arg("-o")
                            .arg(&png_file)
                            .arg("-w")
                            .arg(frame_resolution.to_string())
                            .output().unwrap();
                    fs::remove_file(&svg_file).unwrap();
                    print!("\rSaved frame {}/{}", frame + 1, total_frames);
                    io::stdout().flush().unwrap();
                });
            } else {
                pool.execute(move || {
                    let mut timeout = 30;
                    loop {
                        let mut p = Popen::create(
                            &[inkscape_path.clone(), "-o".to_string(), png_file.clone(), "-w".to_string(), frame_resolution.to_string(), svg_file.clone()],
                            PopenConfig::default()).unwrap();

                        p.wait_timeout(Duration::from_secs(timeout)).unwrap();
                        if let None = p.poll() {
                            p.terminate().unwrap();
                            eprintln!("\rFrame {} failed       ", &frame);
                            if timeout == 90 {
                                eprintln!("\rMax attempts for frame {} reached", &frame);
                                break;
                            }
                            timeout += 30;
                            continue;
                        }
                        break;
                    }
                    fs::remove_file(&svg_file).unwrap();
                    print!("\rSaved frame {}/{}", frame + 1, total_frames);
                    io::stdout().flush().unwrap();
                });
            }
        }
        print!("\r                               ");

        thread::spawn(move || {
            pool.join();
            println!("\rSaved {} frames in folder frames", total_frames);
        });
    }

    fn make_video_from_frames(&self, data: &AppData) {
        let fps = *data.fps_speed.lock().unwrap();
        thread::spawn(move || {
            match fs::remove_file("video.mp4".to_string()) {
                _ => {}
            };
            let mut p = Popen::create(&format!("ffmpeg -r {} -i frames/%05d.png -c:v libx264 -vf pad=ceil(iw/2)*2:ceil(ih/2)*2 -pix_fmt yuv420p video.mp4", 1. / fps)
                .split_whitespace().collect::<Vec<_>>(), PopenConfig {
                stdin: Redirection::Pipe,
                stdout: Redirection::Pipe,
                ..Default::default()
            }).unwrap();
            p.wait().unwrap();
            println!("Video created");
        });
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
                if c.is::<()>(Selector::new("save_frame_as_svg")) {
                    self.save_frame_as_svg(data);
                } else if c.is::<()>(Selector::new("save_frame_as_png")) {
                    self.save_frame_as_png(data);
                } else if c.is::<()>(Selector::new("save_all_frames_as_svg")) {
                    self.save_all_frames_as_svg(data);
                } else if c.is::<()>(Selector::new("save_all_frames_as_png")) {
                    self.save_all_frames_as_png(data);
                } else if c.is::<()>(Selector::new("make_video_from_frames")) {
                    self.make_video_from_frames(data);
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

    let svg_width_scale = Arc::new(Mutex::new(0.3));
    let svg_width_scale_ptr = svg_width_scale.clone();

    let handle = thread::spawn(move || {
        let app_data = AppData {
            objects: objects_ptr,
            frames: frames_ptr,
            frame: 0,
            fps_speed: fps_speed_ptr,
            size: size_ptr,
            tags: tags_ptr,
            draw_properties: draw_properties_ptr,
            svg_width_scale: svg_width_scale_ptr,
            finished: finished_ptr,
        };

        let window = WindowDesc::new(make_layout())
            .window_size(Size {
                width: 800.0,
                height: 600.0,
            })
            .menu(make_menu)
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
        } else if line.starts_with("svgwidth") {
            *svg_width_scale.lock().unwrap() = line[9..].trim().parse::<f64>().unwrap();
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

fn make_menu(_id: Option<WindowId>, _data: &AppData, _env: &Env) -> Menu<AppData> {
    Menu::new("my title")
        .entry(MenuItem::new("Save frame as svg").command(Command::new(Selector::new("save_frame_as_svg"), (), Target::Auto)))
        .entry(MenuItem::new("Save frame as png").command(Command::new(Selector::new("save_frame_as_png"), (), Target::Auto)))
        .entry(MenuItem::new("Save all frames as svg").command(Command::new(Selector::new("save_all_frames_as_svg"), (), Target::Auto)))
        .entry(MenuItem::new("Save all frames as png").command(Command::new(Selector::new("save_all_frames_as_png"), (), Target::Auto)))
        .entry(MenuItem::new("Make video from frames").command(Command::new(Selector::new("make_video_from_frames"), (), Target::Auto)))
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

fn get_settings() -> Settings {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("settings.json");
    let path = path.as_path();
    if !path.is_file() {
        std::fs::File::create(path).unwrap().write_all("{}".to_string().as_bytes()).unwrap();
    }
    let mut settings: Settings = match serde_json::from_str(&fs::read_to_string(path).unwrap()) {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Can't parse json from \"settings.json\"");
            std::process::exit(1);
        }
    };

    if settings.conversion_tool.is_none() {
        settings.conversion_tool = Some("rsvg-convert".to_string());
    }
    if settings.inkscape_path.is_none() {
        settings.inkscape_path = Some("C:/Program files/Inkscape/bin/inkscape.exe".to_string());
    }
    if settings.frame_resolution.is_none() {
        settings.frame_resolution = Some(1080);
    }
    if settings.max_threads.is_none() {
        settings.max_threads = Some(4);
    }

    std::fs::File::create(path).unwrap().write_all(serde_json::to_string_pretty(&settings).unwrap().as_bytes()).unwrap();

    settings
}

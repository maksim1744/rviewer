use std::io::Write;

pub use rviewerable_derive::Rviewerable;

// TRAITS

trait Printable {
    fn print<T: Write>(&self, name: &str, writer: &mut T);
    fn print_sep<T: Write>(&self, name: &str, writer: &mut T, sep: &str) {
        self.print(name, writer);
        write!(writer, "{}", sep).unwrap();
    }
}

impl Printable for f64 {
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        write!(writer, "{}={}", name, self).unwrap();
    }
}
impl Printable for i32 {
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        write!(writer, "{}={}", name, self).unwrap();
    }
}

impl Printable for String {
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        if self.contains(' ') {
            write!(writer, "{}=\"{}\"", name, self).unwrap();
        } else {
            write!(writer, "{}={}", name, self).unwrap();
        }
    }
}

impl Printable for bool {
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        if *self {
            write!(writer, "{}={}", name, *self as i32).unwrap();
        }
    }
    fn print_sep<T: Write>(&self, name: &str, writer: &mut T, sep: &str) {
        if *self {
            self.print(name, writer);
            write!(writer, "{}", sep).unwrap();
        }
    }
}

impl Printable for (f64, f64) {
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        write!(writer, "{}=({},{})", name, self.0, self.1).unwrap();
    }
}
impl Printable for (usize, usize) {
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        write!(writer, "{}=({},{})", name, self.0, self.1).unwrap();
    }
}

impl<P> Printable for Option<P>
where
    P: Printable,
{
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        if let Some(x) = self {
            x.print(name, writer);
        }
    }
    fn print_sep<T: Write>(&self, name: &str, writer: &mut T, sep: &str) {
        if let Some(x) = self {
            x.print_sep(name, writer, sep);
        }
    }
}

impl<P> Printable for Vec<P>
where
    P: Printable,
{
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        for x in self.iter() {
            x.print(name, writer);
        }
    }
    fn print_sep<T: Write>(&self, name: &str, writer: &mut T, sep: &str) {
        for x in self.iter() {
            x.print_sep(name, writer, sep);
        }
    }
}

pub trait Rviewerable {
    fn new() -> Self;
    fn draw<T: Write>(self, writer: &mut T);
}

// HELPER STRUCTS

#[derive(Default, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::rgba(r, g, b, 255)
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn r(mut self, r: u8) -> Self {
        self.r = r;
        self
    }
    pub fn g(mut self, g: u8) -> Self {
        self.g = g;
        self
    }
    pub fn b(mut self, b: u8) -> Self {
        self.b = b;
        self
    }
    pub fn a(mut self, a: u8) -> Self {
        self.a = a;
        self
    }

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    pub const CYAN: Color = Color::rgb(0, 255, 255);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const ORANGE: Color = Color::rgb(255, 165, 0);
}

impl Printable for Color {
    fn print<T: Write>(&self, _name: &str, writer: &mut T) {
        write!(writer, "col=({},{},{}", self.r, self.g, self.b).unwrap();
        if self.a != 255 {
            write!(writer, ",{}", self.a).unwrap();
        }
        write!(writer, ")").unwrap();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Alignment {
    Begin,
    Center,
    End,
}

impl Alignment {
    pub fn to_char(&self) -> char {
        match self {
            Alignment::Begin => 'B',
            Alignment::Center => 'C',
            Alignment::End => 'E',
        }
    }
}

impl Printable for (Alignment, Alignment) {
    fn print<T: Write>(&self, name: &str, writer: &mut T) {
        write!(writer, "{}={}{}", name, self.0.to_char(), self.1.to_char()).unwrap();
    }
}

// HELPER FUNCS

pub fn tick<T: Write>(writer: &mut T) {
    writeln!(writer, "tick").unwrap();
    writer.flush().unwrap();
}

pub fn disable_tag<T: Write>(tag: &str, writer: &mut T) {
    writeln!(writer, "disable {}", tag).unwrap();
}

pub fn set_func<T: Write>(func: &str, values: &[f64], writer: &mut T) {
    writeln!(
        writer,
        "setfunc {} {}",
        func,
        values.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" ")
    )
    .unwrap();
}

pub fn message<T: Write>(msg: &str, writer: &mut T) {
    writeln!(writer, "msg {}", msg).unwrap();
}

// INIT

#[must_use]
#[derive(Default, Rviewerable)]
pub struct Init {
    #[rviewer("size")]
    size: Option<(f64, f64)>,
    #[rviewer("shift")]
    shift: Option<(f64, f64)>,
    #[rviewer("font")]
    font: Option<f64>,
    #[rviewer("speed")]
    speed: Option<f64>,
    #[rviewer("width")]
    width: Option<f64>,
    #[rviewer("svgwidth")]
    svg_width: Option<f64>,
    #[rviewer("in_betweens")]
    in_betweens: Option<i32>,
    #[rviewer("flipy")]
    flipy: bool,
}

// STRUCTS

#[must_use]
#[derive(Rviewerable)]
#[rviewer(name("rect"))]
pub struct Rect {
    center: Option<(f64, f64)>,
    size: Option<(f64, f64)>,
    width: Option<f64>,
    fill: bool,
    color: Option<Color>,
    alignment: Option<(Alignment, Alignment)>,
    #[rviewer("id")]
    id: Option<i32>,
    #[rviewer("fu")]
    func: Option<String>,
    tag: Vec<String>,
    keep: bool,
}

#[must_use]
#[derive(Rviewerable)]
#[rviewer(name("circle"))]
pub struct Circle {
    center: Option<(f64, f64)>,
    radius: Option<f64>,
    #[rviewer("arc")]
    arc: Option<(f64, f64)>,
    width: Option<f64>,
    fill: bool,
    color: Option<Color>,
    #[rviewer("id")]
    id: Option<i32>,
    #[rviewer("fu")]
    func: Option<String>,
    tag: Vec<String>,
    keep: bool,
}

#[must_use]
#[derive(Rviewerable)]
#[rviewer(name("line"))]
pub struct Line {
    start: Option<(f64, f64)>,
    finish: Option<(f64, f64)>,
    width: Option<f64>,
    color: Option<Color>,
    #[rviewer("id")]
    id: Option<i32>,
    #[rviewer("fu")]
    func: Option<String>,
    tag: Vec<String>,
    keep: bool,
}

#[must_use]
#[derive(Rviewerable)]
#[rviewer(name("grid"))]
pub struct Grid {
    center: Option<(f64, f64)>,
    size: Option<(f64, f64)>,
    dims: Option<(usize, usize)>,
    width: Option<f64>,
    color: Option<Color>,
    alignment: Option<(Alignment, Alignment)>,
    #[rviewer("id")]
    id: Option<i32>,
    #[rviewer("fu")]
    func: Option<String>,
    tag: Vec<String>,
    keep: bool,
}

#[must_use]
#[derive(Rviewerable)]
#[rviewer(name("poly"))]
pub struct Poly {
    point: Vec<(f64, f64)>,
    width: Option<f64>,
    fill: bool,
    color: Option<Color>,
    alignment: Option<(Alignment, Alignment)>,
    #[rviewer("id")]
    id: Option<i32>,
    #[rviewer("fu")]
    func: Option<String>,
    tag: Vec<String>,
    keep: bool,
}

#[must_use]
#[derive(Rviewerable)]
#[rviewer(name("text"))]
pub struct Text {
    #[rviewer("m")]
    text: Option<String>,
    center: Option<(f64, f64)>,
    #[rviewer("s")]
    font: Option<f64>,
    color: Option<Color>,
    alignment: Option<(Alignment, Alignment)>,
    #[rviewer("id")]
    id: Option<i32>,
    #[rviewer("fu")]
    func: Option<String>,
    tag: Vec<String>,
    keep: bool,
}

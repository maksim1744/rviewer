use druid::{Point, Size};

pub struct SvgParams<'a> {
    pub size: Size,
    pub width_scale: f64,
    pub flipy: bool,
    pub transform: &'a dyn Fn(Point) -> Point,
}

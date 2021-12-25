use druid::{Size, Point};

pub struct SvgParams<'a> {
    pub size: Size,
    pub width_scale: f64,
    pub transform: &'a dyn Fn(Point) -> Point,
}

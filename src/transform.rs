use druid::{Point, Size};

pub struct Transform<'a> {
    screen_transform: Box<dyn Fn(Point) -> Point + 'a>,
    shift: Size,
    data_size: Size,
    flipy: bool,
}

impl<'a> Transform<'a> {
    pub fn new(screen_transform: impl Fn(Point) -> Point + 'a, shift: Size, data_size: Size, flipy: bool) -> Self {
        Self {
            screen_transform: Box::new(screen_transform),
            shift,
            data_size,
            flipy,
        }
    }

    pub fn point(&self, mut p: Point) -> Point {
        p.x += self.shift.width;
        p.y += self.shift.height;
        if !self.flipy {
            p.y = self.data_size.height - p.y;
        }
        (self.screen_transform)(p)
    }

    pub fn flipy(&self) -> bool {
        self.flipy
    }
}

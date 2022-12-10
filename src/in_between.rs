use std::collections::HashMap;

use crate::figure::CommonParams;

use druid::Color;
use druid::Point;

pub struct InBetweenProperties {
    pub frames: usize,
    pub func: Vec<f64>,
    pub funcs: HashMap<String, Vec<f64>>,
}

impl InBetweenProperties {
    pub fn new() -> Self {
        Self {
            frames: 1,
            func: Vec::new(),
            funcs: HashMap::new(),
        }
    }
}

pub fn interpolate<T: Interpolate>(a: &T, b: &T, k: f64) -> T {
    T::interpolate(a, b, k)
}

pub trait Interpolate {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self;
}

impl Interpolate for f64 {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self {
        a * (1. - k) + b * k
    }
}

impl Interpolate for u8 {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self {
        (*a as f64 * (1. - k) + *b as f64 * k).round() as Self
    }
}

impl Interpolate for usize {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self {
        (*a as f64 * (1. - k) + *b as f64 * k).round() as Self
    }
}

impl Interpolate for (usize, usize) {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self {
        (interpolate(&a.0, &b.0, k), interpolate(&a.1, &b.1, k))
    }
}

impl Interpolate for Point {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self {
        Self::new(interpolate(&a.x, &b.x, k), interpolate(&a.y, &b.y, k))
    }
}

impl Interpolate for Color {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self {
        let ta = a.as_rgba8();
        let tb = b.as_rgba8();
        Self::rgba8(
            interpolate(&ta.0, &tb.0, k),
            interpolate(&ta.1, &tb.1, k),
            interpolate(&ta.2, &tb.2, k),
            interpolate(&ta.3, &tb.3, k),
        )
    }
}

impl Interpolate for CommonParams {
    fn interpolate(a: &Self, b: &Self, k: f64) -> Self {
        let mut result = a.clone();
        result.color = interpolate(&a.color, &b.color, k);
        result
    }
}

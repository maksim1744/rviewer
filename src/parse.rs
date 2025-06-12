use std::collections::HashMap;

use druid::{Color, Point};

pub struct Params<'a> {
    params: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Params<'a> {
    pub fn from_str(s: &'a str) -> Self {
        let mut params: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
        for (key, value) in s
            .split_whitespace()
            .map(|s| {
                let mut iter = s.split('=');
                (iter.next().unwrap(), iter.next().unwrap_or(""))
            })
            .filter(|(_name, value)| !value.is_empty())
        {
            params.entry(key).or_default().push(value);
        }
        Self { params }
    }

    pub fn get<T: Param>(&self, name: &str) -> Option<T> {
        self.params.get(name).and_then(|x| T::from(x))
    }
}

pub trait Param {
    fn from(s: &[&str]) -> Option<Self>
    where
        Self: Sized;
}

impl Param for () {
    fn from(_s: &[&str]) -> Option<Self> {
        Some(())
    }
}

impl Param for bool {
    fn from(s: &[&str]) -> Option<Self> {
        match s[0] {
            "0" => Some(false),
            "1" | "" => Some(true),
            _ => None,
        }
    }
}

impl Param for f64 {
    fn from(s: &[&str]) -> Option<Self> {
        s[0].parse().ok()
    }
}

impl Param for i32 {
    fn from(s: &[&str]) -> Option<Self> {
        s[0].parse().ok()
    }
}

impl Param for String {
    fn from(s: &[&str]) -> Option<Self> {
        Some(s[0].to_string())
    }
}

impl Param for Vec<String> {
    fn from(s: &[&str]) -> Option<Self> {
        Some(s.iter().map(|s| s.to_string()).collect())
    }
}

impl Param for (char, char) {
    fn from(s: &[&str]) -> Option<Self> {
        Some((s[0].chars().nth(0)?, s[0].chars().nth(1)?))
    }
}

impl Param for (usize, usize) {
    fn from(s: &[&str]) -> Option<Self> {
        let s = s[0];
        let mut iter = s[1..s.len() - 1].split(',');
        Some((iter.next()?.parse().ok()?, iter.next()?.parse().ok()?))
    }
}

impl Param for Point {
    fn from(s: &[&str]) -> Option<Self> {
        let s = s[0];
        let mut iter = s[1..s.len() - 1].split(',');
        Some(Point::new(iter.next()?.parse().ok()?, iter.next()?.parse().ok()?))
    }
}

impl Param for (f64, f64) {
    fn from(s: &[&str]) -> Option<Self> {
        let s = s[0];
        let mut iter = s[1..s.len() - 1].split(',');
        Some((iter.next()?.parse().ok()?, iter.next()?.parse().ok()?))
    }
}

impl Param for Vec<Point> {
    fn from(s: &[&str]) -> Option<Self> {
        s.iter().map(|s| <Point as Param>::from(&[s])).collect()
    }
}

impl Param for Color {
    fn from(s: &[&str]) -> Option<Self> {
        let s = s[0];
        let mut iter = s[1..s.len() - 1].split(',');
        let r = iter.next()?.parse().ok()?;
        let g = iter.next()?.parse().ok()?;
        let b = iter.next()?.parse().ok()?;
        let a = match iter.next() {
            Some(x) => x.parse().ok()?,
            None => 255,
        };
        Some(Color::rgba8(r, g, b, a))
    }
}

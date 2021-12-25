use crate::figure::Figure;
use crate::app_data::DrawProperties;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::{Color, Point};

use druid::piet::{FontFamily, Text, TextLayoutBuilder, TextLayout};

use svg::Document;
use svg::node::element::Text as SvgText;
use svg::node::Text as SvgText2;

// used for vertical alignment
const K_VERTICAL_AL: f64 = 0.5;

pub struct MText {
    center: Point,
    text: String,
    font: f64,
    color: Color,
    alignment: (char, char),
    tags: Vec<String>,
    keep: bool,
}

impl MText {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut text = MText {
            center: Point::new(0.0, 0.0),
            text: String::new(),
            font: draw_properties.font,
            alignment: ('C', 'C'),
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
            keep: false,
        };
        let mut s = s.to_string();
        let error_message = format!("Can't parse text from string [{}]", s);
        for i in 0..s.len() {
            if &s[i..i+2] == "m=" {
                let mut j: usize;
                if s.chars().nth(i + 2).expect(&error_message) == '"' {
                    j = i + 3;
                    while s.chars().nth(j).expect(&error_message) != '"' {
                        text.text.push(s.chars().nth(j).expect(&error_message));
                        j += 1;
                    }
                    j += 1
                } else {
                    j = i + 2;
                    while j < s.len() && s.chars().nth(j).expect(&error_message) != ' ' {
                        text.text.push(s.chars().nth(j).expect(&error_message));
                        j += 1;
                    }
                }
                text.text = text.text.replace(";", "\n");
                s = [s[..i-1].to_string(), s[j..].to_string()].concat();
                break;
            }
        }
        for token in s.split_whitespace() {
            if token.starts_with("c=") {
                let mut iter = token[3..token.len() - 1].split(",");
                text.center.x = iter.next().expect(&error_message).parse().expect(&error_message);
                text.center.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("s=") {
                text.font = token[2..].trim().parse().expect(&error_message);
            } else if token.starts_with("a=") {
                text.alignment = (token.chars().nth(2).expect(&error_message), token.chars().nth(3).expect(&error_message));
            } else if token.starts_with("col=") {
                let mut iter = token[5..token.len() - 1].split(",");
                let r = iter.next().expect(&error_message).parse().expect(&error_message);
                let g = iter.next().expect(&error_message).parse().expect(&error_message);
                let b = iter.next().expect(&error_message).parse().expect(&error_message);
                let a = match iter.next() {
                    Some(x) => x.parse().expect(&error_message),
                    None => 255,
                };
                text.color = Color::rgba8(r, g, b, a);
            } else if token.starts_with("t=") {
                text.tags.push(String::from(token[2..].trim()));
            } else if token == "k" {
                text.keep = true;
            }
        }
        text
    }
}

impl Figure for MText {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point) {
        let font = self.font * scale;

        let text = ctx.text();
        let layout = text
            .new_text_layout(self.text.clone())
            .font(FontFamily::SYSTEM_UI, font)
            .text_color(self.color.clone())
            .build()
            .unwrap();

        let text_size = layout.size();

        let mut center = transform(self.center);
        if self.alignment.0 == 'B' {
            center.x += text_size.width / 2.;
        } else if self.alignment.0 == 'E' {
            center.x -= text_size.width / 2.;
        }
        if self.alignment.1 == 'B' {
            center.y -= font * K_VERTICAL_AL;
        } else if self.alignment.1 == 'E' {
            center.y += font * K_VERTICAL_AL;
        }

        let mut text_pos = center;
        text_pos.x -= text_size.width / 2.0;
        text_pos.y -= text_size.height / 2.0;

        ctx.draw_text(&layout, text_pos);
    }

    fn draw_on_image(&self, img: Document, params: &SvgParams) -> Document {
        let center = (params.transform)(self.center);
        let mut y = params.size.height - center.y + self.font * 0.4;
        if self.alignment.1 == 'B' {
            y -= self.font * K_VERTICAL_AL;
        } else if self.alignment.1 == 'E' {
            y += self.font * K_VERTICAL_AL;
        }
        let text = SvgText::new()
            .add(SvgText2::new(&self.text))
            .set("x", center.x)
            .set("y", y)
            .set("fill", self.color_to_string(&self.color))
            .set("font-size", self.font)
            .set("text-anchor",       if self.alignment.0 == 'B' { "start" } else if self.alignment.0 == 'C' { "middle" } else { "end" })
            .set("opacity", self.color.as_rgba().3 as f64)
            .set("font-family", "system-ui");
        img.add(text)
    }

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }

    fn is_keep(&self) -> bool {
        self.keep
    }
}

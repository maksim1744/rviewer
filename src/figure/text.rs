use crate::figure::Figure;
use crate::app_data::DrawProperties;

use druid::widget::prelude::*;
use druid::{Color, Point};

use druid::piet::{FontFamily, Text, TextLayoutBuilder, TextLayout};

use svg::Document;
use svg::node::element::Text as SvgText;
use svg::node::Text as SvgText2;

pub struct MText {
    center: Point,
    text: String,
    font: f64,
    color: Color,
    tags: Vec<String>,
}

impl MText {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut text = MText {
            center: Point::new(0.0, 0.0),
            text: String::new(),
            font: draw_properties.font,
            color: Color::rgb8(0 as u8, 0 as u8, 0 as u8),
            tags: Vec::new(),
        };
        let error_message = format!("Can't parse text from string [{}]", s);
        for token in s.split_whitespace() {
            if token.starts_with("c=") {
                let mut iter = token[3..token.len() - 1].split(",");
                text.center.x = iter.next().expect(&error_message).parse().expect(&error_message);
                text.center.y = iter.next().expect(&error_message).parse().expect(&error_message);
            } else if token.starts_with("m=") {
                text.text = token[2..].trim().replace(";", "\n");
            } else if token.starts_with("s=") {
                text.font = token[2..].trim().parse().expect(&error_message);
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
            }
        }
        text
    }
}

impl Figure for MText {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point) {
        let center = transform(self.center);
        let font = self.font * scale;

        let text = ctx.text();
        let layout = text
            .new_text_layout(self.text.clone())
            .font(FontFamily::SYSTEM_UI, font)
            .text_color(self.color.clone())
            // .alignment(TextAlignment::Start)
            .build()
            .unwrap();

        let text_size = layout.size();

        let mut text_pos = center;
        text_pos.x -= text_size.width / 2.0;
        text_pos.y -= text_size.height / 2.0;

        ctx.draw_text(&layout, text_pos);
    }

    fn draw_on_image(&self, img: Document, scale: f64) -> Document {
        let text = SvgText::new()
            .add(SvgText2::new(&self.text))
            .set("x", self.center.x)
            .set("y", self.center.y)
            .set("fill", self.color_to_string(&self.color))
            .set("font-size", self.font);
        img.add(text)
    }

    fn get_tags(&self) -> std::slice::Iter<'_, std::string::String> {
        self.tags.iter()
    }
}

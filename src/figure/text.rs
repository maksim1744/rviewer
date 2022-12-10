use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::parse::Params;
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::Point;

use druid::piet::{FontFamily, Text, TextLayout, TextLayoutBuilder};

use svg::node::element::Text as SvgText;
use svg::node::Text as SvgText2;
use svg::Document;

// used for vertical alignment
const K_VERTICAL_AL: f64 = 0.5;

pub struct MText {
    center: Point,
    text: String,
    font: f64,
    alignment: (char, char),
    common: CommonParams,
}

impl MText {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let mut s = s.to_string();
        let error_message = format!("Can't parse text from string [{}]", s);
        let mut text = String::new();
        for i in 0..s.len() {
            if &s[i..i + 2] == "m=" {
                let mut j: usize;
                if s.chars().nth(i + 2).expect(&error_message) == '"' {
                    j = i + 3;
                    while s.chars().nth(j).expect(&error_message) != '"' {
                        text.push(s.chars().nth(j).expect(&error_message));
                        j += 1;
                    }
                    j += 1
                } else {
                    j = i + 2;
                    while j < s.len() && s.chars().nth(j).expect(&error_message) != ' ' {
                        text.push(s.chars().nth(j).expect(&error_message));
                        j += 1;
                    }
                }
                text = text.replace(";", "\n");
                s = [s[..i - 1].to_string(), s[j..].to_string()].concat();
                break;
            }
        }
        let params = Params::from_str(&s);
        Self {
            center: params.get("c").unwrap_or(Point::new(0.0, 0.0)),
            text,
            font: params.get("s").unwrap_or(draw_properties.font),
            alignment: params.get("a").unwrap_or(('C', 'C')),
            common: CommonParams::new(&params, draw_properties),
        }
    }
}

impl Figure for MText {
    fn draw(&self, ctx: &mut PaintCtx, scale: f64, transform: &dyn Fn(Point) -> Point) {
        let font = self.font * scale;

        let text = ctx.text();
        let layout = text
            .new_text_layout(self.text.clone())
            .font(FontFamily::SYSTEM_UI, font)
            .text_color(self.common.color.clone())
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
            .set("fill", self.color_to_string())
            .set("font-size", self.font)
            .set(
                "text-anchor",
                if self.alignment.0 == 'B' {
                    "start"
                } else if self.alignment.0 == 'C' {
                    "middle"
                } else {
                    "end"
                },
            )
            .set("opacity", self.common.color.as_rgba().3 as f64)
            .set("font-family", "system-ui");
        img.add(text)
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }
}

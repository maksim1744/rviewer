use crate::app_data::DrawProperties;
use crate::figure::{CommonParams, Figure};
use crate::svg_params::SvgParams;

use druid::widget::prelude::*;
use druid::{Color, Point};

use druid::piet::{FontFamily, Text, TextLayout, TextLayoutBuilder};

use svg::Document;

pub struct MMessage {
    message_ind: usize,
    text: String,
    common: CommonParams,
}

impl MMessage {
    pub fn from_string(s: &str, draw_properties: &mut DrawProperties) -> Self {
        let message = MMessage {
            message_ind: draw_properties.was_messages,
            text: String::from(&s[4..]),
            common: CommonParams::default(),
        };
        draw_properties.was_messages += 1;
        message
    }
}

impl Figure for MMessage {
    fn draw(&self, ctx: &mut PaintCtx, _scale: f64, _transform: &dyn Fn(Point) -> Point) {
        let text = ctx.text();
        let layout = text
            .new_text_layout(self.text.clone())
            .font(FontFamily::MONOSPACE, 10.0)
            .text_color(Color::rgb8(255 as u8, 255 as u8, 255 as u8))
            // .alignment(TextAlignment::Start)
            .build()
            .unwrap();

        let text_size = layout.size();

        ctx.draw_text(&layout, Point::new(0.0, self.message_ind as f64 * text_size.height));
    }

    fn draw_on_image(&self, img: Document, _params: &SvgParams) -> Document {
        img
    }

    fn common(&self) -> &CommonParams {
        &self.common
    }
}

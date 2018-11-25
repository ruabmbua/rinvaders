use crate::rendering::{Pso, Renderable, PixelScreen, Pos, CssColor, CssFont};
use std::fmt::Write;

pub struct Score {
    pos: Pos,
    pso: Pso,
    bad_pso: Pso,
    value: i32,
    name: String,
    label_buf: String,
}

impl Score {
    pub fn new(color: CssColor, bad_color: CssColor, font: CssFont, name: String, pos: Pos) -> Self {
        let mut r = Self {
            pos,
            pso: Pso {
                fill_color: Some(color),
                font: Some(font),
            },
            bad_pso: Pso {
                fill_color: Some(bad_color),
                font: Some(font),
            },
            name,
            value: 0,
            label_buf: String::new(),
        };
        r.refresh_buffer();
        r
    }

    pub fn add(&mut self, val: i32) {
        self.value += val;
        self.refresh_buffer();
    }

    fn refresh_buffer(&mut self) {
        self.label_buf.clear();
        write!(self.label_buf, "{}: {}", self.name, self.value).unwrap();
    }
}


impl Renderable for Score {
    fn draw(&self, pxs: &mut PixelScreen) {
        if self.value < 0 {
            self.bad_pso.bind(pxs);
        } else {
            self.pso.bind(pxs);
        }
        pxs.draw_text(&self.label_buf, self.pos);
    }
}
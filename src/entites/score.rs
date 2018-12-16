//! Score module.
//!
//! Keeps the score, and draws score to the screen.

use crate::rendering::{CssColor, CssFont, PixelScreen, Pos, Pso, Renderable};
use std::fmt::Write;

/// State for the score. Its pos on screen, PSO for drawing, PSO for drawing negative score, current value, name of the score field,
/// and buffer for the current label content.
pub struct Score {
    pos: Pos,
    pso: Pso,
    bad_pso: Pso,
    value: i32,
    name: String,
    label_buf: String,
}

impl Score {
    /// Create new score with *color* for normal color, *bad_color* for negative scores with font, name and position.
    pub fn new(
        color: CssColor,
        bad_color: CssColor,
        font: CssFont,
        name: String,
        pos: Pos,
    ) -> Self {
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

    /// Add points to the score (or remove with negative value).
    pub fn add(&mut self, val: i32) {
        self.value += val;
        self.refresh_buffer();
    }

    /// Refresh the label buffer by taking the current score value and the label name, and writing it to a cleared
    /// **label_buf**
    fn refresh_buffer(&mut self) {
        self.label_buf.clear();
        write!(self.label_buf, "{}: {}", self.name, self.value).unwrap();
    }
}

impl Renderable for Score {
    /// Draw score to screen with the correct PSO depending on its score value.
    fn draw(&self, pxs: &PixelScreen) {
        if self.value < 0 {
            self.bad_pso.bind(pxs);
        } else {
            self.pso.bind(pxs);
        }
        pxs.draw_text(&self.label_buf, self.pos);
    }
}

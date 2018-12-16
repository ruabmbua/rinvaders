//! Fps counter module.
//!
//! The fps counter measures current frames per second by being updated with the current ts every
//! game loop.
//!
//! It is also able to draw itself to the screen.

use crate::rendering::{CssColor, CssFont, PixelScreen, Pos, Pso, Renderable};
use crate::utils::Timer;
use std::fmt::Write;

/// The fps counter.
///
/// Contains last frame timestamp, its PSO, a timer which provides the interval in which fps should be
/// drawn to the screen, and also a string buffer, where the current FPS are written to.
pub struct FpsCounter {
    last_frame_ts: u32,
    timer: Timer,
    print_buf: String,
    pso: Pso,
}

impl FpsCounter {
    /// Create new FPS counter.
    pub fn new() -> Self {
        Self {
            pso: Pso {
                fill_color: Some(CssColor::new(0, 0, 0)),
                font: Some(CssFont::monospace(20)),
            },
            last_frame_ts: 0,
            timer: Timer::interval(0, 500),
            print_buf: String::with_capacity("FPS: XX.XX".len()),
        }
    }

    /// Update function for the FPS counter. It needs per game loop update to measure FPS,
    /// and to check its redraw timer.
    pub fn update(&mut self, ts: u32) {
        let timer = &mut self.timer;
        let print_buf = &mut self.print_buf;
        let last_frame_ts = self.last_frame_ts;

        // Check if we need to redraw. This ignores the time deviation.
        // When yes, clear the print buffer, and write new text to it with the current FPS.
        // --------------------------------------------------------------------------------
        timer.check(ts, |_off| {
            print_buf.clear();
            write!(
                print_buf,
                "FPS: {:.2}",
                1000.0 / (ts - last_frame_ts) as f64
            )
            .unwrap();
        });

        self.last_frame_ts = ts;
    }
}

impl Renderable for FpsCounter {
    /// Render the current print buffer to the screen.
    fn draw(&self, pxs: &PixelScreen) {
        self.pso.bind(pxs);
        pxs.draw_text(&self.print_buf, Pos::new(0.0, 20.0));
    }
}

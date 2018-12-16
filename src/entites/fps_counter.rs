use crate::rendering::{CssColor, CssFont, PixelScreen, Pos, Pso, Renderable};
use std::fmt::Write;
use crate::utils::Timer;

pub struct FpsCounter {
	last_frame_ts: u32,
	timer: Timer,
	print_buf: String,
	pso: Pso,
}

impl FpsCounter {
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

	pub fn update(&mut self, ts: u32) {
		let timer = &mut self.timer;
		let print_buf = &mut self.print_buf;
		let last_frame_ts = self.last_frame_ts;

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
	fn draw(&self, pxs: &PixelScreen) {
		self.pso.bind(pxs);
		pxs.draw_text(&self.print_buf, Pos::new(0.0, 20.0));
	}
}

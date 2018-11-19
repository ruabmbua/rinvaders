use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};

lazy_static! {
    pub static ref EXPLOSION_PSO: Pso = Pso {
        fill_color: Some(CssColor::new(128, 128, 128)),
        ..Default::default()
    };
}

pub struct Explosion {
    pos_x: u32,
    pos_y: u32,
    progress: u8,
}

impl Explosion {
    pub fn new(pos_x: u32, pos_y: u32) -> Self {
        Self {
            pos_x,
            pos_y,
            progress: 0,
        }
    }

    pub fn tick(&mut self) {
        self.progress += 1;
    }

    pub fn needs_removal(&self) -> bool {
        if self.progress > 5 {
            true
        } else {
            false
        }
    }
}

impl Renderable for Explosion {
    fn draw(&self, pxs: &mut PixelScreen) {
        let frames = &[
            &[(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)][..],
            &[(1, 0), (0, 1), (2, 1), (1, 2)][..],
        ];

        let selected = frames[self.progress as usize % 2];

        for px in selected {
            pxs.draw_rect(
                Pos::new(
                    (self.pos_x + px.0) as f64 * 10.0,
                    (self.pos_y + px.1) as f64 * 10.0,
                ),
                10.0,
                10.0,
            );
        }
    }
}

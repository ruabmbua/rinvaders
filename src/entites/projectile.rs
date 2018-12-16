use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};
use crate::utils;

lazy_static! {
    pub static ref PROJECTILE_PSO: Pso = Pso {
        fill_color: Some(CssColor::new(255, 0, 0)),
        ..Default::default()
    };
}

pub struct Projectile {
    pub pos_x: u32,
    pub pos_y: u32,
    dir: (i32, i32),
}

impl Projectile {
    pub fn new(x: u32) -> Self {
        Self {
            pos_x: x,
            pos_y: 56,
            dir: (0, -1),
        }
    }

    pub fn tick(&mut self) {
        self.pos_x = utils::cap(self.pos_x as i32 + self.dir.0, 0, 79) as u32;
        self.pos_y = utils::cap(self.pos_y as i32 + self.dir.1, 0, 59) as u32;
    }

    pub fn needs_removal(&self) -> bool {
        self.pos_y == 0
    }
}

impl Renderable for Projectile {
    fn draw(&self, pxs: &PixelScreen) {
        pxs.draw_rect(
            Pos::new((self.pos_x) as f64 * 10.0, (self.pos_y) as f64 * 10.0),
            10.0,
            10.0,
        );
    }
}

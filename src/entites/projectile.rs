//! Projectile entity module.
//! 
//! Projectiles are spawned by the player, and they fly up towards enemies to kill them. When they hit an enemy, both die, and 
//! the player gets score. If the projectile reaches the top of the screen and goes into the void, it dies and the player
//! looses some of its score.

use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};
use crate::utils;

lazy_static! {
    /// Projectils PSO (red fill color).
    pub static ref PROJECTILE_PSO: Pso = Pso {
        fill_color: Some(CssColor::new(255, 0, 0)),
        ..Default::default()
    };
}

/// The projectile with pos on screen + direction in which to fly (unused).
pub struct Projectile {
    pub pos_x: u32,
    pub pos_y: u32,
    dir: (i32, i32),
}

impl Projectile {
    /// Create a new projectile at x pos.
    pub fn new(x: u32) -> Self {
        Self {
            pos_x: x,
            pos_y: 56,
            dir: (0, -1),
        }
    }

    /// Projectile tick will move it into *dir* direction, only untils it reaches the screen cap.
    pub fn tick(&mut self) {
        self.pos_x = utils::cap(self.pos_x as i32 + self.dir.0, 0, 79) as u32;
        self.pos_y = utils::cap(self.pos_y as i32 + self.dir.1, 0, 59) as u32;
    }

    /// Check if it needs to be removed, because it will enter the void.
    pub fn needs_removal(&self) -> bool {
        self.pos_y == 0
    }

    /// The PSO for **Projectile**.
    pub fn pso() -> &'static Pso {
        &PROJECTILE_PSO
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

//! Explosion entity module.
//!
//! The explosion spawns every time a enemy is destroyed on the position of the destroyed enemy / the involved projectile.
//! In comparison to some other entities, the explosing has an animation cycle with 2 animation steps.
//! The explosion lasts only for a specific time, and will be destroyed after it passed.

use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};

lazy_static! {
    pub static ref EXPLOSION_PSO: Pso = Pso {
        fill_color: Some(CssColor::new(128, 128, 128)),
        ..Default::default()
    };
}

/// Explosion entity.
///
/// Contains relevant state like position and animation progress.
pub struct Explosion {
    pos_x: u32,
    pos_y: u32,
    progress: u8,
}

impl Explosion {
    /// Create new explosion at position.
    pub fn new(pos_x: u32, pos_y: u32) -> Self {
        Self {
            pos_x,
            pos_y,
            progress: 0,
        }
    }

    /// Explosion tick (will make progress on animation).
    pub fn tick(&mut self) {
        self.progress += 1;
    }

    /// Check if the progress of the explosion has reached its end.
    pub fn needs_removal(&self) -> bool {
        if self.progress > 5 {
            true
        } else {
            false
        }
    }

    /// Explosion PSO.
    pub fn pso() -> &'static Pso {
        &EXPLOSION_PSO
    }
}

impl Renderable for Explosion {
    /// Render the animation of the explosion to the **PixelScreen**.
    ///
    /// This has two repeating animation steps, which depend on the **progress** field in the entity.
    fn draw(&self, pxs: &PixelScreen) {
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

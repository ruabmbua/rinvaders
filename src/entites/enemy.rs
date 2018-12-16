//! Enemy entity module.
//!
//! An enemy is one of those black things coming from the top of the screen. Their behaviour is simple:
//!
//!   * Move down one pixel (virtual pixel, not a real pixel on the canvas context) every enemy tick.
//!   * When the void at the bottom is reached, die and remove some points from the player score.
//!   * When shot by projectile, die and give player score some points.
//!   * Spawn on random location on top of the screen every enemy spawn tick.
//!   * There are several visual types of enemies. They each have a different bounding box for collision.

use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};
use crate::utils;
use self::EnemyType::*;

lazy_static! {
    /// Pso for enemies. Fill colored black pixels.
    pub static ref ENEMY_PSO: Pso = Pso {
        fill_color: Some(CssColor::new(0, 0, 0)),
        ..Default::default()
    };
}

/// Types of enemies (different visuals and bounding boxes)
#[derive(Copy, Clone)]
#[repr(u8)]
enum EnemyType {
    Star,
    Thin,
    Arrow,
}

impl EnemyType {
    /// All variants as a static slice.
    /// 
    /// TODO: Consider using not unsafe way to transmute random number into variant. This uses a
    /// byte const data per variant.
    const VARIANTS: &'static [Self] = &[Star, Thin, Arrow];

    /// Create a random variant.
    fn random() -> Self {
        let variants = Self::VARIANTS;
        let idx = utils::rand(variants.len() as u32 - 1);
        *variants.iter().skip(idx as usize).next().unwrap()
    }

    /// Returns a list of virtual to be drawn pixels for each variant of enemy.
    fn px_list(&self) -> &'static [(u32, u32)] {
        match self {
            Star => &[(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)],
            Thin => &[(0, 0), (0, 1)],
            Arrow => &[(0, 0), (2, 0), (1, 1)],
        }
    }

    /// Returns the bounding box for each variant of enemy.
    fn bounds(&self) -> (u32, u32) {
        match self {
            Star => (3, 3),
            Thin => (1, 2),
            Arrow => (3, 2),
        }
    }
}

/// The actual enemy entity. Contains position and enemy type.
pub struct Enemy {
    pos_y: u32,
    pos_x: u32,
    kind: EnemyType,
}

impl Enemy {
    /// Create new enemy on x-coordinate *pos_x*.
    pub fn new(pos_x: u32) -> Self {
        Self {
            pos_y: 0,
            pos_x,
            kind: EnemyType::random(),
        }
    }

    /// Create new enemy on random x-coordinate. This will check bounds to create enemies only there, where
    /// they can be hit by projectiles.
    pub fn new_random() -> Self {
        let kind = EnemyType::random();
        Self {
            pos_y: 0,
            pos_x: utils::cap(
                utils::rand(80),
                if kind.bounds().0 > 1 { 0 } else { 1 },
                80 - kind.bounds().0 - if kind.bounds().0 > 1 { 0 } else { 1 },
            ),
            kind,
        }
    }

    /// Enemy tick (updates the enemy, which is simple down movement).
    pub fn tick(&mut self) {
        self.pos_y += 1;
    }

    /// Checks, if this enemy has to be removed, because it touches the bottom of the screen. 
    pub fn needs_removal(&self) -> bool {
        if self.pos_y + self.kind.bounds().1 > 60 {
            true
        } else {
            false
        }
    }

    /// Check the enemy type bounding box against the given position. Returns true
    /// on intersection.
    /// 
    /// TODO: Convert to pixel perfect collision detection by reusing the **px_list**.
    pub fn intersects_with(&self, x: u32, y: u32) -> bool {
        x >= self.pos_x
            && x < self.pos_x + self.kind.bounds().0
            && y >= self.pos_y
            && y < self.pos_y + self.kind.bounds().1
    }

    /// The PSO for enemy drawing.
    pub fn pso() -> &'static Pso {
        &ENEMY_PSO
    }
}

impl Renderable for Enemy {
    /// Render the enemy pixel list to the **PixelScreen**.
    fn draw(&self, pxs: &PixelScreen) {
        let px_list = self.kind.px_list();

        for px in px_list {
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

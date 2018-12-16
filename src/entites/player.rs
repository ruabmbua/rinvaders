//! The player entity module.
//!
//! The player accesses input from the **input** module, and makes movements / actions according to its state.
//! There are also several timers in the **Player** entity, which limit movement speed etc...
//!
//! The player moves horizontally at the bottom of the screen and shoots projectiles towards enemies.

use super::Projectile;
use crate::input::Input;
use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};
use crate::utils::Timer;

/// The player entity type.
///
/// Contains its PSO, x-coord position, and timers.
pub struct Player {
    pso: Pso,
    pos: u32,
    movement_timer: Option<Timer>,
    shoot_timer: Option<Timer>,
}

impl Player {
    /// Create new player.
    pub fn new() -> Self {
        Self {
            pso: Pso {
                fill_color: Some(CssColor::new(0, 0, 0)),
                ..Default::default()
            },
            pos: 0,
            movement_timer: None,
            shoot_timer: None,
        }
    }

    /// Update the payer every game loop. It has its own ticks provided by timers.utils
    /// Needs access to projectile vector to spawn new projectiles.
    pub fn update(&mut self, ts: u32, input: &Input, projectiles: &mut Vec<Projectile>) {
        /// Do movement function. Modifies pos according to input module.
        ///
        /// Also returns if anything acually happened.
        fn do_movement(pos: &mut u32, input: &Input) -> bool {
            if input.left() && input.right() {
                false
            } else if *pos > 0 && input.left() {
                *pos -= 1;
                true
            } else if *pos < 77 && input.right() {
                *pos += 1;
                true
            } else {
                false
            }
        }

        let timer = &mut self.movement_timer;
        let pos = &mut self.pos;
        let shoot_timer = &mut self.shoot_timer;

        // Movement speed (every 80 ms).
        const SPEED: u32 = 80;

        // Match on timer state, and check if new timer is needed.
        // -------------------------------------------------------
        let need_timer = match timer {
            None => {
                // No timer = move now. This is for instant input reaction.
                // Ony if returns true set up a timer.
                // --------------------------------------------------------
                do_movement(pos, input)
            }
            Some(t) => {
                // When there is a timer, only execute movement on timer event. This limits movement speed.
                t.check(ts, |off| {
                    let mut did_something = false;

                    // Also when the deviation is big enough player may need multiple movements at once to correct deviation.
                    for _ in 0..(off + SPEED) / SPEED {
                        did_something = do_movement(pos, &input);
                    }
                    // Check if this did not actually do anything. When true, remove timer.
                    !did_something
                })
                .map(|did_nothing| {
                    if did_nothing {
                        *timer = None;
                    }
                });
                false
            }
        };

        // Set a new timer when requested.
        // -------------------------------
        if need_timer {
            self.movement_timer = Some(Timer::interval(ts, SPEED));
        }

        let need_timer = match shoot_timer {
            None => {
                if input.shoot() {
                    projectiles.push(Projectile::new(*pos + 1));
                    true
                } else {
                    false
                }
            }
            Some(t) => {
                t.check(ts, |_off| {
                    if input.shoot() {
                        projectiles.push(Projectile::new(*pos + 1));
                        false
                    } else {
                        true
                    }
                })
                .map(|remove_timer| {
                    if remove_timer {
                        *shoot_timer = None;
                    }
                });
                false
            }
        };

        // Also set new timer when required.
        // ---------------------------------
        if need_timer {
            self.shoot_timer = Some(Timer::interval(ts, SPEED));
        }
    }
}

impl Renderable for Player {
    /// Draw the player to the screen using its pixel list.
    /// This time bind the PSO in the draw call, because we do not batch multiple players.
    fn draw(&self, pxs: &PixelScreen) {
        let px_list = &[(1, 0), (0, 1), (1, 1), (2, 1), (0, 2), (2, 2)];

        self.pso.bind(pxs);

        for e in px_list {
            pxs.draw_rect(
                Pos::new((self.pos + e.0) as f64 * 10.0, (57 + e.1) as f64 * 10.0),
                10.0,
                10.0,
            );
        }
    }
}

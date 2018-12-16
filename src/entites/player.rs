use super::Projectile;
use crate::input::Input;
use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};
use crate::utils::Timer;

pub struct Player {
    pso: Pso,
    pos: u32,
    movement_timer: Option<Timer>,
    shoot_timer: Option<Timer>,
}

impl Player {
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

    pub fn update(&mut self, ts: u32, input: &Input, projectiles: &mut Vec<Projectile>) {
        fn do_movement(pos: &mut u32, input: &Input) {
            if input.left() && input.right() {
                return;
            } else if *pos > 0 && input.left() {
                *pos -= 1;
            } else if *pos < 77 && input.right() {
                *pos += 1;
            }
        }

        let timer = &mut self.movement_timer;
        let pos = &mut self.pos;
        let shoot_timer = &mut self.shoot_timer;

        const SPEED: u32 = 80;

        let need_timer = match timer {
            None => {
                do_movement(pos, input);
                true
            }
            Some(t) => {
                t.check(ts, |off| {
                    for _ in 0..(off + SPEED) / SPEED {
                        do_movement(pos, &input);
                    }
                    !input.left() && !input.right()
                }).map(|over| {
                    if over {
                        *timer = None;
                    }
                });
                false
            }
        };

        if need_timer {
            self.movement_timer = Some(Timer::interval(ts, SPEED));
        }

        let need_timer = match shoot_timer {
            None => {
                if input.shoot() {
                    projectiles.push(Projectile::new(*pos + 1));
                }
                true
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
                .map(|e| {
                    if e {
                        *shoot_timer = None;
                    }
                });
                false
            }
        };

        if need_timer {
            self.shoot_timer = Some(Timer::interval(ts, SPEED));
        }
    }
}

impl Renderable for Player {
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

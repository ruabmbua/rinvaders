use crate::rendering::{CssColor, PixelScreen, Pos, Pso, Renderable};
use crate::utils::rand;

lazy_static! {
    pub static ref ENEMY_PSO: Pso = Pso {
        fill_color: Some(CssColor::new(0, 0, 0)),
        ..Default::default()
    };
}

#[derive(Copy, Clone)]
#[repr(u32)]
enum EnemyType {
    #[allow(unused)]
    Star = 0,
    #[allow(unused)]
    Thin = 1,
    #[allow(unused)]
    Arrow = 2,
}

impl EnemyType {
    fn random() -> Self {
        unsafe { std::mem::transmute(rand(2)) }
    }

    fn px_list(&self) -> &'static [(u32, u32)] {
        match self {
            EnemyType::Star => &[(0, 0), (2, 0), (1, 1), (0, 2), (2, 2)],
            EnemyType::Thin => &[(0, 0), (0, 1)],
            EnemyType::Arrow => &[(0, 0), (2, 0), (1,1)],
        }
    }

    fn bounds(&self) -> (u32, u32) {
        match self {
            EnemyType::Star => (3, 3),
            EnemyType::Thin => (1, 2),
            EnemyType::Arrow => (3, 2),
        }
    }
}

pub struct Enemy {
    pos_y: u32,
    pos_x: u32,
    kind: EnemyType,
}

impl Enemy {
    pub fn new(pos_x: u32) -> Self {
        Self {
            pos_y: 0,
            pos_x,
            kind: EnemyType::random(),
        }
    }

    pub fn tick(&mut self) {
        self.pos_y += 1;
    }

    pub fn needs_removal(&self) -> bool {
        if self.pos_y + self.kind.bounds().1 > 60 {
            true
        } else {
            false
        }
    }

    pub fn intersects_with(&self, x: u32, y: u32) -> bool {
        x >= self.pos_x
            && x < self.pos_x + self.kind.bounds().0
            && y >= self.pos_y
            && y < self.pos_y + self.kind.bounds().1
    }
}

impl Renderable for Enemy {
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

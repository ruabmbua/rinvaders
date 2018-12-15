extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
#[macro_use]
extern crate lazy_static;
extern crate wee_alloc;

mod entites;
#[macro_use]
mod logging;
mod input;
mod rendering;
mod utils;

use self::rendering::{CssColor, CssFont, Pos, Renderable};
use self::utils::Timer;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, KeyboardEvent};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
	fps_counter: entites::FpsCounter,
	renderer: rendering::PixelScreen,
	player: entites::Player,
	input: input::Input,
	projectiles: Vec<entites::Projectile>,
	enemies: Vec<entites::Enemy>,
	game_tick: Timer,
	enemy_tick: u8,
	enemy_spawn_tick: u8,
	explosions: Vec<entites::Explosion>,
	explosion_tick: u8,
	score: entites::Score,
}

const TICK: u32 = 10;

#[wasm_bindgen]
impl Game {
	#[wasm_bindgen(constructor)]
	pub fn new(canvas: HtmlCanvasElement) -> Self {
		Self {
			fps_counter: entites::FpsCounter::new(),
			player: entites::Player::new(),
			renderer: rendering::PixelScreen::new(canvas),
			input: input::Input::new(),
			projectiles: vec![],
			enemies: vec![],
			game_tick: Timer::interval(0, TICK),
			enemy_tick: 0,
			enemy_spawn_tick: 0,
			explosions: vec![],
			explosion_tick: 0,
			score: entites::Score::new(
				CssColor::new(0, 50, 200),
				CssColor::new(200, 50, 0),
				CssFont::monospace(20),
				"Score".to_owned(),
				Pos::new(600.0, 20.0),
			),
		}
	}

	pub fn keyboard_event(&mut self, is_down: bool, e: KeyboardEvent) {
		self.input.keyboard_event(is_down, e);
	}

	pub fn render(&mut self) {
		self.renderer.clear();

		self::entites::PROJECTILE_PSO.bind(&mut self.renderer);
		for p in self.projectiles.iter() {
			p.draw(&mut self.renderer);
		}

		self::entites::ENEMY_PSO.bind(&mut self.renderer);
		for e in self.enemies.iter() {
			e.draw(&mut self.renderer);
		}

		self::entites::EXPLOSION_PSO.bind(&mut self.renderer);
		for x in self.explosions.iter() {
			x.draw(&mut self.renderer);
		}

		let display_list: &[&dyn Renderable] = &[
			&self.player as &dyn Renderable,
			&self.fps_counter as &dyn Renderable,
			&self.score as &dyn Renderable,
		];
		self.renderer.draw(&display_list);
	}

	pub fn update(&mut self, ts: u32) {
		self.fps_counter.update(ts);
		self.input.update(ts);
		self.player.update(ts, &self.input, &mut self.projectiles);

		let game_tick = &mut self.game_tick;
		let projectiles = &mut self.projectiles;
		let enemies = &mut self.enemies;
		let enemy_tick = &mut self.enemy_tick;
		let enemy_spawn_tick = &mut self.enemy_spawn_tick;
		let explosions = &mut self.explosions;
		let explosion_tick = &mut self.explosion_tick;
		let score = &mut self.score;

		game_tick.check(ts, |off| {
			for _ in 0..(off + TICK) / TICK {
				projectiles.retain(|e| {
					if e.needs_removal() {
						score.add(-1);
						false
					} else {
						true
					}
				});
				for p in projectiles.iter_mut() {
					p.tick();
				}

				if *enemy_tick == 10 {
					for e in enemies.iter_mut() {
						e.tick();
					}
					enemies.retain(|e| {
						if e.needs_removal() {
							score.add(-10);
							false
						} else {
							true
						}
					});
					*enemy_tick = 0;
				} else {
					*enemy_tick += 1;
				}

				if *explosion_tick == 5 {
					for x in explosions.iter_mut() {
						x.tick();
					}
					explosions.retain(|x| !x.needs_removal());
					*explosion_tick = 0;
				} else {
					*explosion_tick += 1;
				}

				if *enemy_spawn_tick == 200 {
					enemies.push(entites::Enemy::new(utils::rand(79)));
					*enemy_spawn_tick = 0;
				} else {
					*enemy_spawn_tick += 1;
				}

				// Collison check and removal
				// --------------------------
				projectiles.retain(|p| {
					if let Some(e) = enemies
						.iter()
						.enumerate()
						.find(|(_, e)| e.intersects_with(p.pos_x, p.pos_y))
						.map(|(i, _)| i)
					{
						enemies.remove(e);
						explosions.push(entites::Explosion::new(
							utils::cap(p.pos_x as i32 - 1, 0, 79) as u32,
							utils::cap(p.pos_y as i32 + 1, 0, 59) as u32,
						));
						score.add(5);
						false
					} else {
						true
					}
				});
			}
		});
	}

	pub fn set_gamepad_state(&mut self, left: bool, right: bool, shoot: bool) {
		self.input.set_gamepad_state(left, right, shoot);
	}
}

#[wasm_bindgen]
pub fn hello(name: &str) {
	log!("Hello, {}", name);
}

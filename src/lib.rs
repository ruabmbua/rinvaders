//! Main rust wasm module for the rinvaders game. This module contains the main
//! **Game** struct, which holds all the game state and provides update, input and
//! rendering functions for the game.

// Manually import deps (not required, but rls seems to work better that way).
// ---------------------------------------------------------------------------
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

// Global allocator. Will override the toolchain / system default.
// Shrinks the code size of the compiled wasm module.
// ---------------------------------------------------------------
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Main type that holds all game state and provides functionality to update and
/// render the game.
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

/// Game tick. All 10 ms.
/// ---------------------
const TICK: u32 = 10;

#[wasm_bindgen]
impl Game {
    /// Create an instance of the **Game** type. Requires *canvas* parameter which should be the DOM
    /// node of the canvas, into which the game should be rendered.
    ///
    /// Initializes the game.
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

    /// Forwards keyboard events to the **input** module.
    pub fn keyboard_event(&mut self, is_down: bool, e: KeyboardEvent) {
        self.input.keyboard_event(is_down, e);
    }

    /// Render the game. This will clear the canvas, and then redraw the whole scene.
    /// Does not need mutable access, because it immutably reads game date, and draws
    /// to the canvas. Canvas draw operations have some kind of interior mutability,
    /// which allows usage through non mutable references.
    pub fn render(&self) {
        // Clear the screen.
        // -----------------
        self.renderer.clear();

        // Draw all the projectile entites, and bind the PROJECTILE_PSO before doing so.
        // -----------------------------------------------------------------------------
        entites::Projectile::pso().bind(&self.renderer);
        for p in self.projectiles.iter() {
            p.draw(&self.renderer);
        }

        // Bind enemy PSO, and then draw all enemies.
        // ------------------------------------------
        entites::Enemy::pso().bind(&self.renderer);
        for e in self.enemies.iter() {
            e.draw(&self.renderer);
        }

        // Bind the explosion PSO, and then draw all explosions.
        // ----------------------------------------------------
        self::entites::Explosion::pso().bind(&self.renderer);
        for x in self.explosions.iter() {
            x.draw(&self.renderer);
        }

        // Set up a list with various other drawables, which all take care of PSO
        // binding themselves (they are unique, so binding PSO for a batch is not
        // an improvement).
        // Then draw the list by iterating over it.
        // ----------------------------------------------------------------------
        let display_list: &[&dyn Renderable] = &[
            &self.player as &dyn Renderable,
            &self.fps_counter as &dyn Renderable,
            &self.score as &dyn Renderable,
        ];
        self.renderer.draw(&display_list);
    }

    /// Update the game.
    ///
    /// This processes all input, and calculates the next state of the game depending on
    /// previous state, current input, and the current timestamp.
    pub fn update(&mut self, ts: u32) {
        // Update the FPS counter.
        // -----------------------
        self.fps_counter.update(ts);

        // Process input.
        // --------------
        self.input.update(ts);

        // Update the player depending on input. This needs
        // a mutable reference to projectiles, so it can spawn new ones on shoot.
        // ----------------------------------------------------------------------
        self.player.update(ts, &self.input, &mut self.projectiles);

        // Get all the mutable references for fields from **Game** into lifetimes scope.
        // This has to be done to prevent confusing the closures lifetime below with the lifetime
        // of the fields. I think it is related to the fact, that the closure does not get any gurantee
        // from `game_tick.check()` regarding outliving any of the mutable resources in **Game**.
        //
        // TODO: Maybe it is possible to annotate this update function, the Game type, or the closure
        // with lifetimes.
        let game_tick = &mut self.game_tick;
        let projectiles = &mut self.projectiles;
        let enemies = &mut self.enemies;
        let enemy_tick = &mut self.enemy_tick;
        let enemy_spawn_tick = &mut self.enemy_spawn_tick;
        let explosions = &mut self.explosions;
        let explosion_tick = &mut self.explosion_tick;
        let score = &mut self.score;

        // This will check if the gametick is fired. When yes, it executes the given closure.
        // The *off* parameter for the closure tells, how many ms the timer may have missed.
        // ----------------------------------------------------------------------------------
        game_tick.check(ts, |off| {
            // Calculate how many ticks we missed (should be 1 for no misses almost always),
            // and perform all of them right here in the loop body.
            // -----------------------------------------------------------------------------
            for _ in 0..(off + TICK) / TICK {
                // Check if projectile would move into the void. The *retain()* iterator is used,
                // to signal which projectiles survive, or die in this game tick.
                // ------------------------------------------------------------------------------
                projectiles.retain(|e| {
                    if e.needs_removal() {
                        score.add(-1); //<- We lose one point when the projectile missed all enemies.
                        false //<- Also please die.
                    } else {
                        true //<- Allowed to live another day.
                    }
                });

                // Now actually update the projectile.
                // -----------------------------------
                for p in projectiles.iter_mut() {
                    p.tick();
                }

                // When the enemy tick is here, update them all.
                // ---------------------------------------------
                if *enemy_tick == 10 {
                    for e in enemies.iter_mut() {
                        e.tick();
                    }
                    enemies.retain(|e| {
                        if e.needs_removal() {
                            //<- Check if they have to die because of the void.
                            score.add(-10); //<- We loose points if they get past our players defenses.
                            false
                        } else {
                            true
                        }
                    });
                    *enemy_tick = 0;
                } else {
                    *enemy_tick += 1;
                }

                // Update all those beautiful explosions when the tick says so.
                // ------------------------------------------------------------
                if *explosion_tick == 5 {
                    for x in explosions.iter_mut() {
                        x.tick();
                    }
                    explosions.retain(|x| !x.needs_removal()); //<- Eliminate the ones with enough progress.
                    *explosion_tick = 0;
                } else {
                    *explosion_tick += 1;
                }

                // Every enemy spawn tick, spawn new enemies (currently only one per spawn tick).
                // They are spawned along the horizontal axis on the top of the canvas.
                // ------------------------------------------------------------------------------
                if *enemy_spawn_tick == 200 {
                    enemies.push(entites::Enemy::new_random());
                    *enemy_spawn_tick = 0;
                } else {
                    *enemy_spawn_tick += 1;
                }

                // Collison check and removal for every projectile by retaining iterator.
                // ----------------------------------------------------------------------
                projectiles.retain(|p| {
                    if let Some(e) = enemies
                        .iter()
                        .enumerate()
                        // Find collisiony between current projectile and all enemies.
                        // -----------------------------------------------------------
                        .find(|(_, e)| e.intersects_with(p.pos_x, p.pos_y))
                        // Get only the index of the enemy.
                        // --------------------------------
                        .map(|(i, _)| i)
                    {
                        // Collision found. Remove enemy by index, create explosion on this position, and
                        // then mark the projectile for removal. Also add some points to the score.
                        // ------------------------------------------------------------------------------
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

    /// Set the current gamepad state. Axis movement and shoot.
    ///
    /// This will forward the input to the **input** module.
    pub fn set_gamepad_state(&mut self, left: bool, right: bool, shoot: bool) {
        self.input.set_gamepad_state(left, right, shoot);
    }
}

/// Test exported rust function (to wasm module).
#[wasm_bindgen]
pub fn hello(name: &str) {
    log!("Hello, {}", name);
}

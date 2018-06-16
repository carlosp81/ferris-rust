// Copyright © 2018
// "River Bartz"<bpg@pdx.edu>
// "Daniel Dupriest"<kououken@gmail.com>
// "Brandon Goldbeck"<rbartz@pdx.edu>
// This program is licensed under the "MIT License". Please see the file
// LICENSE in the source distribution of this software for license terms.

// Game engine creates
extern crate ggez;
extern crate rand;

// Modules and namespaces
use ggez::{Context, GameResult};
use ggez::event::{self, Keycode, Mod};
use ggez::{audio, graphics};
use std;
mod entity;
mod entity_spawner;
mod scores;
use self::entity_spawner::EntitySpawner;
use self::entity::{EntityType, Lifetime, Movement};
use self::scores::Scores;

// Constants
const ANIMATION_FRAMERATE: f64 = 2.283 * 2.0;
const BOSS_BULLET_COOLDOWN: i64 = 170;
const BOSS_BULLET_NUMBER: i64 = 4;
const DEFAULT_FONT: &str = "/font/PressStart2P.ttf";
const DEFAULT_FONT_SIZE: u32 = 20;
const DISABLE_SFX: bool = false;
const DRAW_BOUNDING_BOXES: bool = false;
const ENEMY_BULLET_COOLDOWN: i64 = 4_000;
const ENEMY_BULLET_SPEED: f32 = 400.0;
const ENEMY_FONT_SIZE: u32 = 12;
const ENEMY_LIFETIME: i64 = 100_000;
const ENEMY_NAMES: [&str;7] = [
	"NULL POINTER",
	"DANGLING REF",
	"SEGFAULT",
	"DOUBLE FREE",
	"INTEGER OVERFLOW",
	"DEADLOCK",
	"RACE CONDITION",
];
/// The closer this is to zero, the faster enemies will spawn at maximum difficulty
const MAX_DIFFICULTY: f32 = 0.15;
const MAX_UPGRADE_LEVEL: u32 = 12;
const PIXEL_SKIP: i32 = 2;
const PLAYER_BULLET_COOLDOWN: i64 = 200;
const PLAYER_BULLET_SPEED: f32 = 600.0;
const SHOW_INPUT_DEBUG: bool = false;
const SHUTOFF_LIFETIME: i64 = 500;
const SPLAT_LIFETIME: i64 = 500;
/// The game will slowly ramp up to maximum difficulty over this amount of time
const SECONDS_UNTIL_MAX_DIFFICULTY: u64 = 8 * 60; 

static mut GOD_MODE: bool = false;

/// Represents the state of player controls
struct Input {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    shoot: bool,
}

/// Game modes for switching between menu display and the main game loop.
pub enum GameMode {
    Menu,
    Game,
	Win,
}

/// The main game state object which contains all the resources
/// and variables needed by various functions.
pub struct MainState {
	/// Star field background
    background: graphics::Image,
	/// Time since last frame was rendered (in ms).
	delta_ms: u64,
	/// Time elapsed since beginning of game (in ms).
	elapsed_ms: u64,
	/// Vector of all drawable game entities.
    entities: Vec<entity::Entity>,
	/// Current game mode determining whether to display menu or game
	game_mode: GameMode,
	/// List of recent high scores.
	high_scores: Scores,
	/// Player input state
	input: Input,
	/// Hash map of text label graphics for enemy names
	labels: std::collections::HashMap<String, graphics::Text>,
	/// Means of exiting the game
	quit: bool,
	/// Random number generator passed to certain functions
	rng: rand::ThreadRng,
	/// Current player score
    score: i32,
	/// Font to use for player score
    score_font: graphics::Font,
	/// Hash map of all game sounds and music, indexed by string name
	sfx: std::collections::HashMap<&'static str, audio::Source>,
	/// Generator for game objects like enemies and bullets
	spawner: EntitySpawner,
	/// Reference time for when the game began
	start_time: std::time::SystemTime,
	/// Hash map of game entity textures, indexed by the enum `EntityType`.
	/// Each entry is a vector of `Image` objects, and vectors with more
	/// than one image will display as an animation.
	textures: std::collections::HashMap<entity::EntityType, Vec<graphics::Image>>,
	/// Game logo
	title: graphics::Image,
    gun_level: u32,
    shield_active: bool
}

/// This is the object ggez will update with the screen.
impl MainState {
	/// This function is run one time at the start of the game. It sets up
	/// and returns the game state.
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
		graphics::set_default_filter(ctx, graphics::FilterMode::Nearest);
		
        let score_font = graphics::Font::new(ctx, DEFAULT_FONT, DEFAULT_FONT_SIZE)?;
		
		// Set up main state
        let mut s = MainState {
            background: graphics::Image::new(ctx, "/texture/background_tiled.png").unwrap(),
			delta_ms: 0,
			elapsed_ms: 0,
            entities: Vec::new(),
			game_mode: GameMode::Menu,
			high_scores: Scores::new("scores.txt"),
			input: Input {
				left: false, 
				right: false, 
				up: false,
				down: false,
				shoot: false,
			},
			labels: std::collections::HashMap::new(),
			quit: false,
			rng: rand::thread_rng(),
            score: 0,
            gun_level: 0,
            shield_active: false,
            score_font,
			sfx: std::collections::HashMap::new(),
			spawner: EntitySpawner::new(ctx),
			start_time:  std::time::SystemTime::now(),
			textures: std::collections::HashMap::new(),
            title: graphics::Image::new(ctx, "/texture/title.png").unwrap(),
		};
		
		// Set up textures
		s.textures.insert(entity::EntityType::Player, vec![
			graphics::Image::new(ctx, "/texture/crab1.png").unwrap(),
			graphics::Image::new(ctx, "/texture/crab0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/crab2.png").unwrap(),
			graphics::Image::new(ctx, "/texture/crab0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/crab1.png").unwrap(),
			graphics::Image::new(ctx, "/texture/crab0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/crab3.png").unwrap(),
			graphics::Image::new(ctx, "/texture/crab0.png").unwrap(),
		]);
		s.textures.insert(entity::EntityType::Enemy, vec![
			graphics::Image::new(ctx, "/texture/enemy0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/enemy1.png").unwrap(),
		]);
		s.textures.insert(entity::EntityType::EnemyBlueScreen, vec![
			graphics::Image::new(ctx, "/texture/enemybluescreen0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/enemybluescreen1.png").unwrap()
		]);
		s.textures.insert(entity::EntityType::PlayerBullet, vec![graphics::Image::new(ctx, "/texture/player_bullet.png").unwrap()] );
		s.textures.insert(entity::EntityType::EnemyBullet, vec![graphics::Image::new(ctx, "/texture/enemy_bullet.png").unwrap()] );
		s.textures.insert(entity::EntityType::Powerbomb, vec![
			graphics::Image::new(ctx, "/texture/powerbomb0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/powerbomb1.png").unwrap(),
		]);
		s.textures.insert(entity::EntityType::Special, vec![
			graphics::Image::new(ctx, "/texture/special0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/special1.png").unwrap(),
		]);
		s.textures.insert(entity::EntityType::Splat, vec![graphics::Image::new(ctx, "/texture/splat.png").unwrap()] );
		s.textures.insert(entity::EntityType::Shutoff, vec![graphics::Image::new(ctx, "/texture/shutoff.png").unwrap()] );
		s.textures.insert(entity::EntityType::Life, vec![graphics::Image::new(ctx, "/texture/cpu.png").unwrap()] ); 
		s.textures.insert(entity::EntityType::Boss, vec![graphics::Image::new(ctx, "/texture/boss.png").unwrap()] );
		s.textures.insert(entity::EntityType::GunUpgrade, vec![
			graphics::Image::new(ctx, "/texture/gunupgrade0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/gunupgrade1.png").unwrap()
		]);
        s.textures.insert(entity::EntityType::Shield, vec![
			graphics::Image::new(ctx, "/texture/shield0.png").unwrap(),
			graphics::Image::new(ctx, "/texture/shield1.png").unwrap()
		]);
        
		// Set up music and sound effects
		s.sfx.insert("player_shot", audio::Source::new(ctx, "/sounds/player_shot.wav")?);
		s.sfx.insert("hit", audio::Source::new(ctx, "/sounds/hit.wav")?);
		s.sfx.insert("explode", audio::Source::new(ctx, "/sounds/explode.wav")?);
		s.sfx.insert("intro", audio::Source::new(ctx, "/sounds/intro.ogg")?);
		s.sfx.insert("bgm", audio::Source::new(ctx, "/sounds/Tejaswi-Hyperbola.ogg")?);
		s.sfx.insert("enemy_shot", audio::Source::new(ctx, "/sounds/enemy_shot.wav")?);
		s.sfx.insert("upgrade", audio::Source::new(ctx, "/sounds/upgrade.wav")?);
		s.sfx.insert("shield", audio::Source::new(ctx, "/sounds/shield.wav")?);
		s.sfx.insert("powerbomb", audio::Source::new(ctx, "/sounds/powerbomb.wav")?);
		s.sfx.insert("win", audio::Source::new(ctx, "/sounds/Tejaswi-Solstice.ogg")?);
		
		// Generate labels
		let entity_font = graphics::Font::new(ctx, DEFAULT_FONT, ENEMY_FONT_SIZE)?;
		for name in ENEMY_NAMES.iter() {
			let text = graphics::Text::new(ctx, name, &entity_font).unwrap();
			s.labels.insert(name.to_string(), text);
		}
		let bsod_text = graphics::Text::new(ctx, "BSOD", &entity_font).unwrap();
		s.labels.insert("BSOD".to_string(), bsod_text);

		// Begin playing intro music
        if !DISABLE_SFX {
            s.sfx["intro"].play().unwrap();
        }

        Ok(s)
    }
}
	
/// This function starts a new game
pub fn new_game(state: &mut MainState, ctx: &mut Context) {
    // Clear out old entities
    state.entities.clear();

	// Reset time
	state.elapsed_ms = 0;
	state.start_time = std::time::SystemTime::now();
	
	// Reset spawner
	state.spawner.reset();
	
    // Reset the score and powerups
    state.score = 0;
    state.gun_level = 1;
    state.shield_active = false;

    // Create a new player object
    let player = entity::Entity {
        angle: 0.0,
        bounds: graphics::Rect {
            x: 60.0,
            y: 40.0,
            w: 10.0,
            h: 18.0,
        },
        bullet_cooldown: PLAYER_BULLET_COOLDOWN,
        damage: 0,
        entity_type: entity::EntityType::Player,
        hp: 5,
        lifetime: Lifetime::Forever,
        movement: Movement::None,
        name: "Ferris".to_string(),
        seed: 0.0,
        timer: 0,
        vel: 400.0,
        x: (ctx.conf.window_mode.width as f32 / 2.0)
            - (state.textures[&entity::EntityType::Player][0].width() as f32 / 2.0),
        y: ctx.conf.window_mode.height as f32
            - state.textures[&entity::EntityType::Player][0].height() as f32,
    };

    state.entities.push(player);

    // Stop intro music and begin bgm
    if !DISABLE_SFX {
        state.sfx["intro"].stop();
        // The `.stop()` method for a ggez audio source doesn't seem to work
        // correctly, so this is an ugly method of stopping and restarting the
        // audio. Reload from disk and overwrite existing. Eeewww!
        *state.sfx.get_mut("bgm").unwrap() =
            audio::Source::new(ctx, "/sounds/Tejaswi-Hyperbola.ogg").expect("Could not load bgm");
        state.sfx["bgm"].play().unwrap();
    }
}

/// This function handles all entity-entity interactions when colliding
fn handle_collisions(state: &mut MainState) {
	let mut play_hit_sound = false;
	// Iterate through all entities
	for entity_idx in 0..state.entities.len() {
	
		match state.entities[entity_idx].entity_type {
		
			// In the case of player
			EntityType::Player => {
                for threat_idx in 0..state.entities.len() {
                    match state.entities[threat_idx].entity_type {
                        
						// When player collides with enemy
						EntityType::Enemy | EntityType::EnemyBlueScreen | EntityType::Boss | EntityType::Special => {
                            if colliding(state, entity_idx, threat_idx) {
							
								// If shield is active
								if state.shield_active {
									
									// Remove shield
									state.shield_active = false;
								} else {
								
									// Otherwise hurt player
									unsafe {
										if !GOD_MODE {
											state.entities[entity_idx].hp -= state.entities[threat_idx].damage;
											if state.gun_level > 1 {
												state.gun_level -= 1;
											}
										}
									}
								}
								
								// Destroy enemies other than boss
								if state.entities[threat_idx].entity_type != EntityType::Boss {
									state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								}
								
								// SFX
								play_hit_sound = true;
                            }
                        },
						
						// When player collides with enemy bullet
                        EntityType::EnemyBullet => {
                            if colliding(state, entity_idx, threat_idx) {
								
								// If shield is active
								if state.shield_active {
								
									// Disable shield
									state.shield_active = false;
								
								} else {
								
									// Otherwise hurt player
									unsafe {
										if !GOD_MODE {
											state.entities[entity_idx].hp -=
											state.entities[threat_idx].damage;
										}
									}
								}
								
								// Destroy bullet
                                state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
                                
								// SFX
								play_hit_sound = true;
                            }
                        },
						
						// When player collides with power bomb
                        EntityType::Powerbomb => {
                            if colliding(state, entity_idx, threat_idx) {
							
								// Destroy all bullets and enemies other than boss
                                for enemy_idx in 0..state.entities.len() {
                                    if state.entities[enemy_idx].entity_type == EntityType::Enemy
                                        || state.entities[enemy_idx].entity_type
                                            == EntityType::EnemyBlueScreen
										|| state.entities[enemy_idx].entity_type == EntityType::Special
                                        || state.entities[enemy_idx].entity_type
                                            == EntityType::EnemyBullet
                                    {
                                        state.entities[enemy_idx].lifetime =
                                            Lifetime::Milliseconds(0);
                                        state.entities[enemy_idx].hp = 0;
                                    }
                                }
								
								if !DISABLE_SFX {
									state.sfx["powerbomb"].play().unwrap();
								}
								
								// Kill powerbomb
                                state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
                            }
                        },
						
						// When player collides with gun upgrade
                        EntityType::GunUpgrade => {
                            if colliding(state, entity_idx, threat_idx) {
							
                                // Upgrade the player's gun
                                if state.gun_level < MAX_UPGRADE_LEVEL {
									state.gun_level += 1;
								}
									
								if !DISABLE_SFX {
									state.sfx["upgrade"].play().unwrap();
								}
								
								// Remove upgrade
                                state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
                            }
                        },
						
						// When player collides with shield
                        EntityType::Shield => {
                            if colliding(state, entity_idx, threat_idx) {
							
                                // Enable shield
                                state.shield_active = true;
								
								if !DISABLE_SFX {
									state.sfx["shield"].play().unwrap();
								}
								
								// Remove shield powerup
                                state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
                            }
                        },
                        
						_ => (),
                    }
				}
            },
			
			// In the case of an enemy or boss
			EntityType::Enemy | EntityType::EnemyBlueScreen | EntityType::Boss | EntityType::Special => {
				for threat_idx in 0..state.entities.len() {
					match state.entities[threat_idx].entity_type {
						
						// When an enemy collides with a player bullet
						EntityType::PlayerBullet => {
							if colliding(state, entity_idx, threat_idx) {
								
								// Hurt the enemy by bullet damage amount
								state.entities[entity_idx].hp -= state.entities[threat_idx].damage;
								
								// Kill the bullet
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								
								play_hit_sound = true;
                            }
						},
						
						_ => (),
					}
				}
			},

			_ => (),
		}
	}
	

	if !DISABLE_SFX && play_hit_sound {
		state.sfx["hit"].play().unwrap();
	}
}

/// Returns true if the two entities are colliding. Collision is calculated
/// using the `bounds` dimensions of the entity, not the sprite.
fn colliding(state: &mut MainState, a: usize, b: usize) -> bool{
	// If bounding boxes collide
	let e1_x = state.entities[a].x + state.entities[a].bounds.x;
	let e1_w = state.entities[a].bounds.w;
	let e1_y = state.entities[a].y + state.entities[a].bounds.y;
	let e1_h = state.entities[a].bounds.h;
	let e2_x = state.entities[b].x + state.entities[b].bounds.x;
	let e2_w = state.entities[b].bounds.w;
	let e2_y = state.entities[b].y + state.entities[b].bounds.y;
	let e2_h = state.entities[b].bounds.h;
	if e1_x < e2_x + e2_w &&
		e1_x + e1_w > e2_x &&
		e1_y < e2_y + e2_h &&
		e1_h + e1_y > e2_y {
			true
	}
	else {
		return false;
	}
}

/// Write high score
fn save_score(state: &mut MainState) {
	let user = std::env::var("USERNAME").unwrap();
    let total = state.elapsed_ms / 1000;
	let minutes = total / 60;
	let seconds = total % 60;
	let time = format!("{:02}:{:02}", minutes, seconds);
	state.high_scores.add_score(state.score, user.to_string(), time.to_string());
	state.high_scores.save("scores.txt");
}

/// Update the state's `elapsed_ms` and `delta_ms`.
fn update_time(state: &mut MainState) {
    let now = std::time::SystemTime::now();
    let difference = now.duration_since(state.start_time)
        .expect("Time went backwards");
    let current_ms = difference.as_secs() * 1000 + difference.subsec_nanos() as u64 / 1_000_000;
    state.delta_ms = match state.elapsed_ms {
        0 => 0,
        _ => current_ms - state.elapsed_ms,
    };
    state.elapsed_ms = current_ms;
}

/// We implement the `ggez:event::EventHandler` trait on `MainState`, which
/// requires callbacks for updating and drawing the game state each frame.
///
/// The `EventHandler` trait also contains callbacks for event handling
/// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {

	/// Update game objects and do game logic loop
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.quit {
            ctx.quit()?;
        }

        match self.game_mode {
            // If we are in the menu
			GameMode::Menu => {
                if self.input.shoot {
                    self.game_mode = GameMode::Game;
                    new_game(self, ctx);
                }
            },
			
			// If we are in the game
            GameMode::Game => {
                handle_collisions(self);

                // If the player died, gameover!
                if self.entities.len() == 0 || self.entities[0].entity_type != EntityType::Player {
                    self.game_mode = GameMode::Menu;
                    
					save_score(self);
					
                    // Pause game for a moment
                    let pause = std::time::Duration::from_millis(500);
                    std::thread::sleep(pause);

                    // Stop bgm and replay intro music
                    if !DISABLE_SFX {
                        self.sfx["bgm"].stop();
                        // The `.stop()` method for a ggez audio source doesn't seem to work
                        // correctly, so this is an ugly method of stopping and restarting the
                        // audio. Reload from disk and overwrite existing. Eeewww!
                        *self.sfx.get_mut("intro").unwrap() =
                            audio::Source::new(ctx, "/sounds/intro.ogg")
                                .expect("Could not load intro music");
                        self.sfx["intro"].play().unwrap();
                    }
                }

                match self.spawner.update(self.elapsed_ms, self.delta_ms) {
                    Some(e) => {
                        self.entities.push(e);
                    },
                    None => (),
                }

                // Run each entity's update function
                for i in 0..self.entities.len() {
                    let mut e = self.entities.remove(i);
                    e.update(self, ctx);
                    self.entities.insert(i, e);
                }

				// If player is firing
                if self.input.shoot {
                    if self.entities[0].bullet_cooldown == 0 {
						// Reset cooldown
						self.entities[0].bullet_cooldown = PLAYER_BULLET_COOLDOWN;
						
						let pi = std::f64::consts::PI;
						let angle_step = pi / 16.0;
						let player_tex = &self.textures[&entity::EntityType::Player][0];
						let bullet_tex = &self.textures[&entity::EntityType::PlayerBullet][0];
						for i in 0..self.gun_level {
							let angle = pi / 2.0 + (i as f64 - self.gun_level as f64 / 2.0) * angle_step + angle_step / 2.0;
							let x = self.entities[0].x + player_tex.width() as f32 / 2.0 - bullet_tex.width() as f32 + bullet_tex.width() as f32 / 2.0 + player_tex.width() as f32 / 2.0 * angle.cos() as f32;
							let y = self.entities[0].y + player_tex.height() as f32 / 2.0 - bullet_tex.height() as f32 / 2.0 - player_tex.width() as f32 / 2.0 * angle.sin() as f32;
							let mut bullet = self.spawner.player_bullet_spawner(x, y);
							bullet.movement = Movement::Linear(
								PLAYER_BULLET_SPEED * angle.cos() as f32,
								-PLAYER_BULLET_SPEED * angle.sin() as f32
							);
							self.entities.push(bullet);
						}						
                        
                        if !DISABLE_SFX {
                            // Nasty means of playing shot sounds quickly on the same channel.
                            *self.sfx.get_mut("player_shot").unwrap() =
                                audio::Source::new(ctx, "/sounds/player_shot.wav")
                                    .expect("Could not load enemy shot");
                            self.sfx["player_shot"].play().unwrap();
                        }
                    }
                }

				// Boolean to sound explosion if necessary
				let mut play_explosion_sound = false;
				
				// Create vector of dying entities
                let mut dying_entities: Vec<usize> = vec![];

                // Grab the dying entities.
                for all_idx in 0..self.entities.len() {
                    let e = &mut self.entities[all_idx];

                    let mut dying = match e.lifetime {
                        Lifetime::Forever => false,
                        Lifetime::Milliseconds(r) => r <= 0,
                    };

                    if !dying {
                        if e.hp <= 0 || e.y > ctx.conf.window_mode.height as f32 {
                            dying = true;
                        }
                    }

                    if dying {
                        // Check for any entities dying by low hp.
                        if e.hp <= 0 {
							match e.entity_type {
								EntityType::Enemy => {self.score += 10;},
								EntityType::EnemyBlueScreen => {self.score += 30;},
								EntityType::Boss => {self.score += 200;},
								EntityType::Special => {self.score += 50;},
								_ => (),
							}
							play_explosion_sound = true;
						}		
						
						
                        // 100% guarentee we can kill off the target by hp alone.
						e.hp = 0;
						dying_entities.push(all_idx);
					}
				}

				// Spawn some on_death effects.
				for i in 0..dying_entities.len() {
					let x = self.entities[dying_entities[i]].x;
					let y = self.entities[dying_entities[i]].y;
					match self.entities[dying_entities[i]].entity_type {
						entity::EntityType::Boss => self.entities.push(self.spawner.spawn_splat(x, y)),
						entity::EntityType::Enemy => self.entities.push(self.spawner.spawn_splat(x, y)),
						entity::EntityType::EnemyBlueScreen => self.entities.push(self.spawner.spawn_shutoff(x, y)),
						entity::EntityType::Special => {
							let mut item = self.spawner.spawn_item();
							item.x = x;
							item.y = y;
							self.entities.push(item);
						},
						_ => (), 
					}
				}

				// Now we can just kill off stuff if it doesnt have hp.
				self.entities.retain(|e| {
					e.hp > 0
				});

				// If at least one entity has died from low hp, we should make an explosion sound
				if !DISABLE_SFX && play_explosion_sound {
					// The `.stop()` method for a ggez audio source doesn't seem to work
					// correctly, so this is an ugly method of stopping and restarting the
					// audio. Reload from disk and overwrite existing. Eeewww!
					*self.sfx.get_mut("explode").unwrap() = audio::Source::new(ctx, "/sounds/explode.wav").expect("Could not load explode.wav");
					self.sfx["explode"].play().unwrap();
				}
				
        		// Keep bgm playing in a loop
				if !DISABLE_SFX && !self.sfx["bgm"].playing() {
					self.sfx["bgm"].play().unwrap();
				}
				
				// Win the game if time is up
				if self.elapsed_ms / 1000 > SECONDS_UNTIL_MAX_DIFFICULTY + 5 {
					self.game_mode = GameMode::Win;
					
					save_score(self);
					
					if !DISABLE_SFX {
						self.sfx["bgm"].stop();
						self.sfx["win"].play().unwrap();
					}
				}
			},
			
			// If we have won
			GameMode::Win => {
				
			},
		}
		
		update_time(self);

        Ok(())
    }

	/// Draw all the game entities and UI.
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
		graphics::clear(ctx);

		let window_width = ctx.conf.window_mode.width;
		let window_height = ctx.conf.window_mode.height;
		
		match self.game_mode {
			
			// If in the menu
			GameMode::Menu => {
			
				// Draw two layers of two background copies staggered according to elapsed_ms
				let background_y = ( (self.elapsed_ms/40%1920) as i32 / PIXEL_SKIP * PIXEL_SKIP ) as f32;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, background_y), 0.0)?;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, -1920.0 + background_y), 0.0)?;
				
				// Draw title
				graphics::draw(ctx, &self.title, graphics::Point2::new(229.0, 100.0), 0.0)?;
				
				// Draw "press spacebar" text blinking
				let mut text = graphics::Text::new(ctx, &format!("- PRESS SPACEBAR -"), &self.score_font).unwrap();
				if self.elapsed_ms % 1000 < 500 {
					
					graphics::draw(ctx, &text, graphics::Point2::new(400.0, 650.0), 0.0)?;
				}
					
				// Draw high scores
				text = graphics::Text::new(ctx, &format!("{:10} {:12} {:5}", "Score", "User", "Time"), &self.score_font).unwrap();
				graphics::draw(ctx, &text, graphics::Point2::new(200.0, 300.0), 0.0)?;
				let scores = self.high_scores.get_scores();
				for i in 0 .. scores.len() {
					let (score, name, time) = &scores[i];
					let score_text = format!("{:<10} {:10}   {}", score, &name, &time);
					let drawing_text = graphics::Text::new(ctx, &score_text, &self.score_font).unwrap();
					graphics::draw(ctx, &drawing_text, graphics::Point2::new(200.0, 330.0 + (i as f32) * 30_f32), 0.0)?;
				}
			},
			
			// If in the game loop
			GameMode::Game => {

				// Draw two layers of two background copies staggered according to elapsed_ms
				let background_y = ( (self.elapsed_ms/40%1920) as i32 / PIXEL_SKIP * PIXEL_SKIP ) as f32;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, background_y), 0.0)?;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, -1920.0 + background_y), 0.0)?;

				// Draw all entities
				for e in &mut self.entities {
					let pos = graphics::Point2::new((e.x as i32 / PIXEL_SKIP * PIXEL_SKIP ) as f32, (e.y as i32 / PIXEL_SKIP * PIXEL_SKIP) as f32);

					// If the texure is animated, grab the right frame, otherwise grab frame 0.
					let total_frames = self.textures[&e.entity_type].len();
					let texture = match total_frames {
						1 => &self.textures[&e.entity_type][0],
						_ => {
							let frame = (self.elapsed_ms as f64 / 1000.0 * ANIMATION_FRAMERATE)
								as usize % total_frames;
							&self.textures[&e.entity_type][frame]
						}
					};

					// Special drawing conditions start
					match e.entity_type {
						entity::EntityType::Player => {
							if self.shield_active {
								graphics::set_color(ctx, graphics::Color::new(0.3, 1.0, 0.3, 1.0))?
							}
						},
						entity::EntityType::Boss => match e.hp {
							0...10 => graphics::set_color(
								ctx,
								graphics::Color::new(1.0, 0.25, 0.25, 1.0),
							)?,
							10...20 => {
								graphics::set_color(ctx, graphics::Color::new(1.0, 0.5, 0.5, 1.0))?
							}
							_ => {
								graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?
							}
						},
						entity::EntityType::Splat | entity::EntityType::Shutoff => {
							let mut alpha: f32 = match e.lifetime {
								Lifetime::Forever => 1.0_f32,
								Lifetime::Milliseconds(r) => r as f32 / SPLAT_LIFETIME as f32,
							};
							graphics::set_color(
								ctx,
								graphics::Color::new(alpha, alpha, alpha, alpha),
							)?;
						},
						_ => {}
					}

					// Draw the entity sprite rotated around center of sprite if needed
					// Non-square sprites may not rotate correctly
					if e.angle == 0.0 {
						graphics::draw(ctx, texture, pos, e.angle)?;
					} else {
						let half_width = texture.width() as f64 / 2.0;
						let angle = -e.angle as f64 + (5.0 * std::f64::consts::PI / 4.0);
						let x = (half_width + half_width * (2.0_f64).sqrt() * angle.cos()) as f32;
						let y = (half_width + half_width * (2.0_f64).sqrt() * angle.sin()) as f32;
						graphics::draw(
							ctx,
							texture,
							graphics::Point2::new(pos.x + x, pos.y + y),
							-e.angle,
						)?;
					}

					// End drawing conditions: Reset drawing conditions
					graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;

					// If this is an enemy, include a name tag.
					if e.entity_type == entity::EntityType::Enemy ||
						e.entity_type == entity::EntityType::EnemyBlueScreen {
						
						// Dim label after a while
						match e.lifetime {
							Lifetime::Forever => (),
							Lifetime::Milliseconds(r) => {
								let fraction_of_life = ( ENEMY_LIFETIME as f32 - r as f32 ) / ENEMY_LIFETIME as f32;
								let mut alpha = 1.0 - fraction_of_life * 15.0;
								if alpha < 0.0 {
									alpha = 0.0;
								}
								graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, alpha))?;
							},
						};	

						// Calculate label position
						let offset = 30;
						let text_pos = graphics::Point2::new(
							((e.x as i32 + texture.width() as i32 + offset + 6) / PIXEL_SKIP * PIXEL_SKIP ) as f32, 
							((e.y as i32 - offset - 6) / PIXEL_SKIP * PIXEL_SKIP) as f32);
						
						// Draw the label
						graphics::draw(ctx, &self.labels[&e.name], text_pos, 0.0)?;
						
						// Draw a line connecting it to entity
						graphics::line(ctx, &[
							graphics::Point2::new(text_pos.x - 6.0, text_pos.y + self.labels[&e.name].height() as f32),
							graphics::Point2::new(text_pos.x - offset as f32, text_pos.y + offset as f32)
						], 4.0)?;
						
						// Reset color
						graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
					}
					
					// Draw collision boxes if they are enabled.
					if DRAW_BOUNDING_BOXES {
						graphics::rectangle(
							ctx,
							graphics::DrawMode::Line(1.0),
							graphics::Rect {
								x: e.x + e.bounds.x,
								y: e.y + e.bounds.y,
								w: e.bounds.w,
								h: e.bounds.h,
							},
						)?;
					}
				}

				// Draw the player's life graphics
				let player = &self.entities[0];
				let tex_width = self.textures[&EntityType::Life][0].width();
				if player.hp > 0 {
					for i in 0..player.hp {
						graphics::draw(
							ctx,
							&self.textures[&EntityType::Life][0],
							graphics::Point2::new(window_width as f32 - tex_width as f32 * 1.25 * i as f32 - tex_width as f32, 0.0), 0.0)?;
					}
				}

				// Draw "message text" for excitement
				if self.gun_level == MAX_UPGRADE_LEVEL {
					let mut text = graphics::Text::new(ctx, &format!("- RUST FULLY UPGRADED -"), &self.score_font).unwrap();
					let blink = (self.elapsed_ms as f64 / 1000.0 * ANIMATION_FRAMERATE) as usize % 4 < 2;
					if blink {
						graphics::draw(ctx, &text, graphics::Point2::new(window_width as f32 / 2.0 - text.width() as f32 / 2.0, window_height as f32 - text.height() as f32), 0.0)?;
					}				
				}
					
				// Generate the score text graphics and draw to screen
				let score = graphics::Text::new(ctx, &format!("Score: {}", 
					&self.score.to_string()), &self.score_font).unwrap();
				graphics::draw(ctx, &score, graphics::Point2::new(10.0, 10.0), 0.0)?;
			},
		
			// If in the win state
			GameMode::Win => {
				let pi = std::f64::consts::PI;
								
				// Draw Ferrises
				let total_frames = self.textures[&EntityType::Player].len();
				let ferris_rows = 6;
				let ferris_columns = 4;
				for i in 0..ferris_rows {
					for j in 0.. ferris_columns {
						let initial_angle = (self.elapsed_ms as f64 + i as f64 * 173.0 + j as f64 * 132.0) / 1000.0 * pi;
						let frame = ( (self.elapsed_ms as f64 / 1000.0 * ANIMATION_FRAMERATE) as usize + i ) % total_frames;
						let texture = &self.textures[&EntityType::Player][frame];
						let x = 50.0 * (2.0 * initial_angle).cos() as f32;
						let y = (50.0 * (2.0 * initial_angle).sin()).abs() as f32;
						let dance_offset = (window_width as f32 / 2.0 - texture.width() as f32 * 2.0) * (self.elapsed_ms as f64 / 4000.0 * pi).cos() as f32;
						let horiz_offset = window_width as f32 / ferris_columns as f32; 
						let vert_offset = window_height as f32 / ferris_rows as f32;
						graphics::draw_ex(
							ctx,
							texture,
							graphics::DrawParam {
								dest: graphics::Point2::new(
									x + dance_offset + j as f32 * horiz_offset,
									50.0 + i as f32 * vert_offset - y
								),
								rotation: 0.0,
								scale: graphics::Point2::new(2.0, 2.0),
								..Default::default()
							},
						)?;
					}
				}
				
				// Draw text
				let font = graphics::Font::new(ctx, DEFAULT_FONT, 16)?;
				let green_text = graphics::Text::new(ctx, &format!("Finished"), &font).unwrap();
				let green_text_width = green_text.width() as f32;
				let white_text = graphics::Text::new(ctx, &format!("release [optimized] target(s) in {} points", &self.score.to_string()), &font).unwrap();
				let white_text_width = white_text.width() as f32;
				graphics::set_color(ctx, graphics::Color::new(0.0, 1.0, 0.0, 1.0))?;
				graphics::draw(
					ctx,
					&green_text,
					graphics::Point2::new(
						window_width as f32 / 2.0 - (green_text_width + white_text_width) / 2.0,
						window_height as f32 / 2.0 - green_text.height() as f32 / 2.0
					),
					0.0
				)?;
				graphics::set_color(ctx, graphics::Color::new(0.8, 0.8, 0.8, 1.0))?;
				graphics::draw(
					ctx,
					&white_text,
					graphics::Point2::new(
						window_width as f32 / 2.0 - (green_text_width + white_text_width) / 2.0 + green_text_width + 10.0,
						window_height as f32 / 2.0 - green_text.height() as f32 / 2.0
					),
					0.0
				)?;
				graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
				
				// Draw rust logo
				let texture = &self.textures[&EntityType::PlayerBullet][0];
				let half_width = texture.width() as f64 * 3.0 / 2.0;
				let initial_angle = self.elapsed_ms as f64 / 1000.0 * pi;
				let angle = -initial_angle + (5.0 * pi / 4.0);
				let x = (half_width + half_width * (2.0_f64).sqrt() * initial_angle.cos()) as f32;
				let y = (half_width + half_width * (2.0_f64).sqrt() * initial_angle.sin()) as f32;
				graphics::draw_ex(
					ctx,
					texture,
					graphics::DrawParam {
						dest: graphics::Point2::new(0.0 + x, 0.0 + y),
						rotation: -angle as f32,
						scale: graphics::Point2::new(3.0, 3.0),
						..Default::default()
					},
				)?;
				graphics::draw_ex(
					ctx,
					texture,
					graphics::DrawParam {
						dest: graphics::Point2::new(window_width as f32 - texture.width() as f32 * 3.0 + x, 0.0 + y),
						rotation: -angle as f32,
						scale: graphics::Point2::new(3.0, 3.0),
						..Default::default()
					},
				)?;
				graphics::draw_ex(
					ctx,
					texture,
					graphics::DrawParam {
						dest: graphics::Point2::new(x, window_height as f32 - texture.height() as f32 * 3.0 + y),
						rotation: -angle as f32,
						scale: graphics::Point2::new(3.0, 3.0),
						..Default::default()
					},
				)?;
				graphics::draw_ex(
					ctx,
					texture,
					graphics::DrawParam {
						dest: graphics::Point2::new(window_width as f32 - texture.width() as f32 * 3.0 + x, window_height as f32 - texture.height() as f32 * 3.0 + y),
						rotation: -angle as f32,
						scale: graphics::Point2::new(3.0, 3.0),
						..Default::default()
					},
				)?;
			},
		}

		graphics::present(ctx);

		Ok(())
	}

	// Event is triggered when the player presses keydowns
	fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
		if SHOW_INPUT_DEBUG {
			println!(
				"Key pressed: {:?}, modifier {:?}, repeat: {}",
				keycode, keymod, repeat
			);
		}

		if keycode == ggez::event::Keycode::Left {
			self.input.left = true;
		}
		if keycode == ggez::event::Keycode::Right {
			self.input.right = true;
		}
		if keycode == ggez::event::Keycode::Up {
			self.input.up = true;
		}
		if keycode == ggez::event::Keycode::Down {
			self.input.down = true;
		}
		if keycode == ggez::event::Keycode::Space {
			self.input.shoot = true;
		}
		if keycode == ggez::event::Keycode::Escape {
			self.quit = true;
		}
	}

	// Event is triggered when player lifts up on a keys
	fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
		if SHOW_INPUT_DEBUG {
			println!(
				"Key released: {:?}, modifier {:?}, repeat: {}",
				keycode, keymod, repeat
			);
		}

		if keycode == ggez::event::Keycode::Left {
			self.input.left = false;
		}
		if keycode == ggez::event::Keycode::Right {
			self.input.right = false;
		}
		if keycode == ggez::event::Keycode::Up {
			self.input.up = false;
		}
		if keycode == ggez::event::Keycode::Down {
			self.input.down = false;
		}
		if keycode == ggez::event::Keycode::Space {
			self.input.shoot = false;
			self.entities[0].bullet_cooldown = 0;
		}
		if keycode == ggez::event::Keycode::B {
			self.spawner.cooldowns.insert(EntityType::Boss, 0);
		}
		if keycode == ggez::event::Keycode::E {
			self.spawner.cooldowns.insert(EntityType::Enemy, 0);
		}
		if keycode == ggez::event::Keycode::G {
			unsafe {
				if GOD_MODE == false {
					GOD_MODE = true;
				}
				else {
					GOD_MODE = false;
				}
			}
		}
		if keycode == ggez::event::Keycode::S {
			self.spawner.cooldowns.insert(EntityType::Special, 0);
		}
		if keycode == ggez::event::Keycode::W {
			self.elapsed_ms = SECONDS_UNTIL_MAX_DIFFICULTY * 1000 + 6000;
		}
	}
}
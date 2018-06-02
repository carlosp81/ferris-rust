/*
Copyright <2018> <River Bartz, Daniel Dupriest, Brandon Goldbeck>

Permission is hereby granted, free of charge, to any person obtaining a copy of this 
software and associated documentation files (the "Software"), to deal in the Software 
without restriction, including without limitation the rights to use, copy, modify, 
merge, publish, distribute, sublicense, and/or sell copies of the Software, and to 
permit persons to whom the Software is furnished to do so, subject to the following 
conditions:
The above copyright notice and this permission notice shall be included in all copies
or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR 
PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE 
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR 
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER 
DEALINGS IN THE SOFTWARE.
*/

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
use self::entity_spawner::EntitySpawner;
use self::entity::{Lifetime, EntityType, Movement};

// Constants
const BOSS_BULLET_COOLDOWN: i64 = 150;
const BOSS_BULLET_NUMBER: i64 = 3;
const BULLET_SPEED: f32 = 400.0;
const DEFAULT_FONT: &str = "/font/PressStart2P.ttf";
const DEFAULT_FONT_SIZE: u32 = 20;
const DISABLE_SFX: bool = false;
const DRAW_BOUNDING_BOXES: bool = false;
const ENEMY_BULLET_COOLDOWN: i64 = 2_000;
const ENEMY_FONT_SIZE: u32 = 12;
const ENEMY_NAMES: [&str;4] = [
	"NULL POINTER",
	"DANGLING REF",
	"SEGFAULT",
	"DOUBLE FREE",
];
const GOD_MODE: bool = false;
const PLAYER_BULLET_COOLDOWN: i64 = 250;
const SHOW_INPUT_DEBUG: bool = false;
const SHUTOFF_LIFETIME: i64 = 500;
const SPLAT_LIFETIME: i64 = 500;

static mut MAX_ENTITIES: i64 = 0;


// Struct to represent player controls
struct Input {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
	shoot: bool,
}

// Modes which control menu display / game loop
pub enum GameMode {
	Menu,
	Game,
}

// First we make a struct to contain the game's state
pub struct MainState {
    background: graphics::Image,
	delta_ms: u64,
	elapsed_ms: u64,
    entities: Vec<entity::Entity>,
	game_state: GameMode,
	high_scores: Vec<String>,
	input: Input,
	labels: std::collections::HashMap<String, graphics::Text>,
	quit: bool,
	rng: rand::ThreadRng,
    score: u32,
    score_font: graphics::Font,
    score_text: graphics::Text,
	sfx: std::collections::HashMap<&'static str, audio::Source>,
	spawner: EntitySpawner,	// This creates enemies and bullets
	start_time: std::time::SystemTime,
	textures: std::collections::HashMap<entity::EntityType, graphics::Image>,
}

// This is the object ggez will update with the screen.
impl MainState {
	// Run one time at the start of the game
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let score_font = graphics::Font::new(ctx, DEFAULT_FONT, DEFAULT_FONT_SIZE)?;
		let score_text = graphics::Text::new(ctx, "Score: ", &score_font)?;
		
        let mut s = MainState {
            background: graphics::Image::new(ctx, "/texture/background_tiled.png").unwrap(),
			delta_ms: 0,	//Elapsed time since last frame, in milliseconds
			elapsed_ms: 0,	//Elapsed time since state creation, in milliseconds
            entities: Vec::new(),
			game_state: GameMode::Menu,
			high_scores: Vec::new(),
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
            score_font,
            score_text,
			sfx: std::collections::HashMap::new(),
			spawner: EntitySpawner::new(ctx),
			start_time:  std::time::SystemTime::now(),
			textures: std::collections::HashMap::new(),
		};
		
		// Set up textures
		s.textures.insert(entity::EntityType::Player, graphics::Image::new(ctx, "/texture/crab.png").unwrap() );
		s.textures.insert(entity::EntityType::Enemy, graphics::Image::new(ctx, "/texture/enemy.png").unwrap() );
		s.textures.insert(entity::EntityType::EnemyBlueScreen, graphics::Image::new(ctx, "/texture/enemybluescreen.png").unwrap() );
		s.textures.insert(entity::EntityType::PlayerBullet, graphics::Image::new(ctx, "/texture/player_bullet.png").unwrap() );
		s.textures.insert(entity::EntityType::EnemyBullet, graphics::Image::new(ctx, "/texture/enemy_bullet.png").unwrap() );
		s.textures.insert(entity::EntityType::Powerup, graphics::Image::new(ctx, "/texture/powerup.png").unwrap() );
		s.textures.insert(entity::EntityType::Splat, graphics::Image::new(ctx, "/texture/splat.png").unwrap() );
		s.textures.insert(entity::EntityType::Shutoff, graphics::Image::new(ctx, "/texture/shutoff.png").unwrap() );
		s.textures.insert(entity::EntityType::Life, graphics::Image::new(ctx, "/texture/cpu.png").unwrap()); 
		s.textures.insert(entity::EntityType::Boss, graphics::Image::new(ctx, "/texture/boss.png").unwrap());
		
		// Set up music and sound effects
		s.sfx.insert("player_shot", audio::Source::new(ctx, "/sounds/player_shot.wav")?);
		s.sfx.insert("hit", audio::Source::new(ctx, "/sounds/hit.wav")?);
		s.sfx.insert("explode", audio::Source::new(ctx, "/sounds/explode.wav")?);
		s.sfx.insert("intro", audio::Source::new(ctx, "/sounds/intro.ogg")?);
		s.sfx.insert("bgm", audio::Source::new(ctx, "/sounds/Tejaswi-Hyperbola.ogg")?);
		        
		// Generate labels
		let entity_font = graphics::Font::new(ctx, DEFAULT_FONT, ENEMY_FONT_SIZE)?;
		for name in ENEMY_NAMES.iter() {
			let text = graphics::Text::new(ctx, name, &entity_font).unwrap();
			s.labels.insert(name.to_string(), text);
		}
		let bsod_text = graphics::Text::new(ctx, "BSOD", &entity_font).unwrap();
		s.labels.insert("BSOD".to_string(), bsod_text);

        if !DISABLE_SFX {
			s.sfx["intro"].play().unwrap();
		}

        Ok(s)
    }
}
	
// Call this to start a new game
pub fn new_game(state: &mut MainState, ctx: &mut Context) {
	
	// Clear out old entities
	state.entities.clear();

	// Reset the score
	state.score = 0;

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
    	vel: 375.0,
		x: (ctx.conf.window_mode.width as f32 / 2.0) - (state.textures[&entity::EntityType::Player].width() as f32 / 2.0),
        y: ctx.conf.window_mode.height as f32 - state.textures[&entity::EntityType::Player].height() as f32,
    };

	state.entities.push(player);

	// Stop intro music and begin bgm
	if !DISABLE_SFX {
		state.sfx["intro"].pause();
		state.sfx["bgm"].play().unwrap();
	}
}

// Handle entity-entity interactions.
fn handle_collisions(state: &mut MainState) {

	// Iterate through subject entities
	for entity_idx in 0..state.entities.len() {
		match state.entities[entity_idx].entity_type {

			EntityType::Player => {
				for threat_idx in 0..state.entities.len() {
					match state.entities[threat_idx].entity_type {
						EntityType::Enemy | EntityType::EnemyBlueScreen => {
							if colliding(state, entity_idx, threat_idx) {
								if !GOD_MODE {
									state.entities[entity_idx].hp -= state.entities[threat_idx].damage;
								}
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								if !DISABLE_SFX {
									state.sfx["hit"].play().unwrap();
								}
                            }
						},
						EntityType::Boss => {
							if colliding(state, entity_idx, threat_idx) {
								if !GOD_MODE {
									state.entities[entity_idx].hp -= state.entities[threat_idx].damage;
								}
								if !DISABLE_SFX {
									state.sfx["hit"].play().unwrap();
								}
							}
						},
						EntityType::EnemyBullet => {
							if colliding(state, entity_idx, threat_idx) {
								if !GOD_MODE {
									state.entities[entity_idx].hp -= state.entities[threat_idx].damage;
								}
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								if !DISABLE_SFX {
									state.sfx["hit"].play().unwrap();
								}
                            }
						},
						EntityType::Powerup => {
							if colliding(state, entity_idx, threat_idx) {
								// Right now, the only powerup we have will destroy all enemies on the screen.
								for enemy_idx in 0..state.entities.len() {
									if state.entities[enemy_idx].entity_type == EntityType::Enemy || state.entities[enemy_idx].entity_type == EntityType::EnemyBlueScreen ||
									state.entities[enemy_idx].entity_type == EntityType::EnemyBullet{
										state.entities[enemy_idx].lifetime = Lifetime::Milliseconds(0);
										state.entities[enemy_idx].hp = 0;
									}
								}
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								if !DISABLE_SFX {
									state.sfx["explode"].play().unwrap();
								}
                            }
						},
						_ => (),
					}
				}
			},

			EntityType::PlayerBullet => {
			},

			EntityType::EnemyBullet => (),

			// If we are an enemy (entity_idx)
			EntityType::Enemy | EntityType::EnemyBlueScreen | EntityType::Boss => {
				for threat_idx in 0..state.entities.len() {
					match state.entities[threat_idx].entity_type {
						// See if we hit the threat
						EntityType::PlayerBullet => {
							if colliding(state, entity_idx, threat_idx) {
								state.entities[entity_idx].hp -= state.entities[threat_idx].damage;
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								if state.entities[entity_idx].hp <= 0 {
									state.entities[entity_idx].lifetime = Lifetime::Milliseconds(0);
								}
								
								if !DISABLE_SFX {
									state.sfx["hit"].play().unwrap();
								}
                            }
						},
						_ => (),
					}
				}
			},

			_ => (),
		}
	}
}

// Check if a has hit b, and kill a if it does;
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
		false
	}
}


// Update state's elapsed ms and delta ms
fn update_time(state: &mut MainState) {
	let now = std::time::SystemTime::now();
	let difference = now.duration_since(state.start_time).expect("Time went backwards");
	let current_ms = difference.as_secs() * 1000 + difference.subsec_nanos() as u64 / 1_000_000;
	state.delta_ms = match state.elapsed_ms {
		0 => 0,
		_ => current_ms - state.elapsed_ms,
	};
	state.elapsed_ms = current_ms;
}


// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if self.quit {
			_ctx.quit()?;
		}

		// Output max entities for debugging
		let number = self.entities.len() as i64;
		unsafe {
			if number > MAX_ENTITIES {
				MAX_ENTITIES = number;
				//println!("Max entities = {}", number);
			}
		}
			
		match self.game_state {
			GameMode::Menu => {
				if self.input.shoot {
					self.game_state = GameMode::Game;
					new_game(self, _ctx);
				}
				
			},
			GameMode::Game => {
				
				handle_collisions(self);
				
				// If the player died, gameover!
				if self.entities.len() == 0 || self.entities[0].entity_type != EntityType::Player {
					self.game_state = GameMode::Menu;
					let user = std::env::var("USERNAME").unwrap();
					let text = self.score.to_string() + " " + &user;
					self.high_scores.push(text);
				}

				match self.spawner.update(self.delta_ms) {
					Some(e) => {
						self.entities.push(e);
					},
					None => (),
				}

				self.score_text = graphics::Text::new(_ctx, &format!("Score: {}", &self.score.to_string()), &self.score_font).unwrap();
			
				// Run thru the list of entities
				for i in 0..self.entities.len() {
					let mut e = self.entities.remove(i);
					e.update(self, _ctx);
					self.entities.insert(i, e);
				}

				// Set the score variable
				self.score_text = graphics::Text::new(_ctx, &format!("Score: {}", 
					&self.score.to_string()), &self.score_font).unwrap();

				if self.input.shoot {
					if self.entities[0].bullet_cooldown == 0 {
						// Reset cooldown.
						self.entities[0].bullet_cooldown = PLAYER_BULLET_COOLDOWN;
						// Spawn the bullet.
						let x = self.entities[0].x + (self.textures[&entity::EntityType::Player].width() as f32 / 2.0) - (self.textures[&entity::EntityType::PlayerBullet].width() as f32 / 2.0);
						let y = self.entities[0].y - (self.textures[&entity::EntityType::PlayerBullet].height() as f32 / 2.0);
						let pb = self.spawner.player_bullet_spawner(x, y);
						self.entities.push(pb);
						if !DISABLE_SFX {
							self.sfx["player_shot"].play()?;
						}
					}
				}
				
				let mut dying_entities: Vec<usize> = vec![];
				
                // Boolean to sound an explosion if necessary
				let mut do_explosion_sound = false;
                
				// Grab the dying entities.
				for all_idx in 0..self.entities.len() {
					let e = &mut self.entities[all_idx];

					let mut dying = match e.lifetime {
						Lifetime::Forever => false,
						Lifetime::Milliseconds(r) => r <= 0,
					};
					
					if !dying {
						if e.hp <= 0 || e.y > _ctx.conf.window_mode.height as f32 {
							dying = true;
						}
					}

					if dying {
						// Check for any entities dying by low hp.
						if e.hp <= 0 {
							// Gain score points
							self.score += 10;
							do_explosion_sound = true;
						}
                        
                        // 100% guarentee we can kill off the target by hp alone.
						e.hp = 0;
						dying_entities.push(all_idx);
					}
				
				}

				// If at least one entity has died from low hp, we should make an explosion sound
				if do_explosion_sound && !DISABLE_SFX {
					self.sfx["explode"].play().unwrap();
				}

				// Spawn some on_death effects.
				for i in 0..dying_entities.len() {
					let x = self.entities[dying_entities[i]].x;
					let y = self.entities[dying_entities[i]].y;
					match self.entities[dying_entities[i]].entity_type {
						entity::EntityType::Boss => self.entities.push(self.spawner.spawn_splat(x, y)),
						entity::EntityType::Enemy => self.entities.push(self.spawner.spawn_splat(x, y)),
						entity::EntityType::EnemyBlueScreen => self.entities.push(self.spawner.spawn_shutoff(x, y)),
						_ => (), 
					}
					
				}

				// Now we can just kill off stuff if it doesnt have hp.
				self.entities.retain(|e| {
					e.hp > 0
				});
		
        		// Keep bgm playing in a loop
				if !DISABLE_SFX && !self.sfx["bgm"].playing() {
					self.sfx["bgm"].play().unwrap();
				}
			}
		}
		update_time(self);

		

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::Color::new(0.0, 0.0, 0.0, 1.0));
		graphics::clear(ctx);

		match self.game_state {
			GameMode::Menu => {
				// Draw the background
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, 0.0), 0.0)?;
				
				let mut text = graphics::Text::new(ctx, &format!("Press Space to play game"), &self.score_font).unwrap();
				graphics::draw(ctx, &text, graphics::Point2::new(200.0, 200.0), 0.0)?;

				text = graphics::Text::new(ctx, &format!("High Scores:"), &self.score_font).unwrap();
				graphics::draw(ctx, &text, graphics::Point2::new(200.0, 250.0), 0.0)?;

				for i in 0 .. self.high_scores.len() {
					self.score_text = graphics::Text::new(ctx, &self.high_scores[i], &self.score_font).unwrap();
					graphics::draw(ctx, &self.score_text, graphics::Point2::new(200.0, 280.0 + (i as f32) * 30_f32), 0.0)?;
				}


			},
			GameMode::Game => {
				let _window_width = ctx.conf.window_mode.width;
				let _window_height = ctx.conf.window_mode.height;

				// Draw two layers of two background copies staggered according to elapsed_ms
				let background_y = ( (self.elapsed_ms/40%1920) as i32 / 2 * 2 ) as f32;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, background_y), 0.0)?;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, -1920.0 + background_y), 0.0)?;
				
				// Draw all entities
				for e in &mut self.entities {
					let pos = graphics::Point2::new((e.x as i32 / 4 * 4 ) as f32, (e.y as i32 / 4 * 4) as f32);
					let texture = &self.textures[&e.entity_type];

					// Special drawing conditions start
					match e.entity_type {
						entity::EntityType::Boss => {
							match e.hp {
								0 ... 5 => graphics::set_color(ctx, graphics::Color::new(1.0, 0.25, 0.25, 1.0))?,
								5 ... 10 => graphics::set_color(ctx, graphics::Color::new(1.0, 0.5, 0.5, 1.0))?,
								_ => graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?,
							}
						},
						entity::EntityType::EnemyBlueScreen => {
							match e.hp {
								0 ... 2 => graphics::set_color(ctx, graphics::Color::new(1.0, 0.25, 0.25, 1.0))?,
								_ => graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?,
							}
						},
						entity::EntityType::Splat | entity::EntityType::Shutoff => {
							let mut alpha : f32 = match e.lifetime {
								Lifetime::Forever => 1.0_f32,
								Lifetime::Milliseconds(r) => r as f32 / SPLAT_LIFETIME as f32,
							};
							graphics::set_color(ctx, graphics::Color::new(alpha, alpha, alpha, alpha))?;
						}
						_ => {}
					}
					
					// Draw the entity sprite rotated if needed
					if e.angle == 0.0 {
						graphics::draw(ctx, texture, pos, e.angle)?;
					}  
					else {
						let half_width = texture.width() as f64 / 2.0;
						let angle = -e.angle as f64 + (5.0 * std::f64::consts::PI / 4.0);
						let x = (half_width + half_width * (2.0_f64).sqrt() * angle.cos()) as f32;
						let y = (half_width + half_width * (2.0_f64).sqrt() * angle.sin()) as f32;
						graphics::draw(ctx, texture, graphics::Point2::new(pos.x + x, pos.y + y), -e.angle)?;
					}
				
					// End drawing conditions: Reset drawing conditions
					graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
					
					// If this is an enemy, include a name tag.
					if e.entity_type == entity::EntityType::Enemy ||
						e.entity_type == entity::EntityType::EnemyBlueScreen {
						let offset = 30;
						let text_pos = graphics::Point2::new(
							((e.x as i32 + texture.width() as i32 + offset) / 2 * 2 ) as f32, 
							((e.y as i32 - offset) / 2 * 2) as f32);
						
                        //let text_pos = graphics::Point2::new(
						//	e.x + texture.width() as f32 + offset, 
						//	e.y - offset);
                        
						graphics::draw(ctx, &self.labels[&e.name], text_pos, 0.0)?;
						graphics::line(ctx, &[
							graphics::Point2::new(text_pos.x - 5.0, text_pos.y + self.labels[&e.name].height() as f32),
							graphics::Point2::new(pos.x + texture.width() as f32, pos.y)], 4.0)?;
					}
					
					

					// Draw collision boxes if they are enabled.
					if DRAW_BOUNDING_BOXES {
					graphics::rectangle(ctx,
						graphics::DrawMode::Line(1.0),
						graphics::Rect {
							x: e.x + e.bounds.x,
							y: e.y + e.bounds.y,
							w: e.bounds.w,
							h: e.bounds.h}
						)?;
					}
				}

				// Draw the player's life graphics
				let player = &self.entities[0];

				for i in 0..player.hp + 1 {
					graphics::draw(
						ctx,
						&self.textures[&EntityType::Life],
						graphics::Point2::new(_window_width as f32 - (self.textures[&EntityType::Life].width() as f32 * 1.25 * i as f32), 0.0), 0.0)?;
				}
				
				// Draw player score
				graphics::draw(ctx, &self.score_text, graphics::Point2::new(10.0, 10.0), 0.0)?;
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
        if keycode ==  ggez::event::Keycode::Up {
            self.input.up = true;
            
        }
        if keycode ==  ggez::event::Keycode::Down {
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
        if keycode ==  ggez::event::Keycode::Up {
            self.input.up = false;
            
        }
        if keycode ==  ggez::event::Keycode::Down {
            self.input.down = false;
        }
		if keycode == ggez::event::Keycode::Space {
			self.input.shoot = false;
			self.entities[0].bullet_cooldown = 0;
		}
    }
}

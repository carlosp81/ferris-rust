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

const DEFAULT_FONT: &str = "/font/FiraSans-Regular.ttf";
const DEFAULT_FONT_SIZE: u32 = 30;
const PLAYER_BULLET_COOLDOWN: i64 = 250;
const BULLET_SPEED: f32 = 400.0;
const ENEMY_BULLET_COOLDOWN: i64 = 2_000;
const DRAW_BOUNDING_BOXES: bool = true;
const DISABLE_SFX: bool = true;

// Adjust this to start further ahead or behind in the spawn schedule
//const SCHEDULE_OFFSET: u64 = 0;
//const USE_BETA_SCHEDULER: bool = false;
const SHOW_INPUT_DEBUG: bool = true;

//const WINDOW_WIDTH: f32 = 1024.0;
//const WINDOW_HEIGHT: f32 = 1024.0;

//const ENEMY_SPAWN_MIN_TIME: u64 = 500; //500 is good
//const ENEMY_SPAWN_MAX_TIME: u64 = 5000; //5000 is good
//const POWERUP_DELAY: i64 = 15_000; 

struct Input {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
	shoot: bool,
}

// Menu states
pub enum MenuState {
	Menu,
	Game,
}

// First we make a structure to contain the game's state
pub struct MainState {
	spawner: EntitySpawner,
    score_text: graphics::Text,
    frames: usize,
    entities: Vec<entity::Entity>,
	input: Input,
    score: u32,
    score_font: graphics::Font,
	high_scores: Vec<u32>,
    background: graphics::Image,
	start_time: std::time::SystemTime,
	elapsed_ms: u64,
	delta_ms: u64,
	textures: std::collections::HashMap<entity::EntityType, graphics::Image>,
	bgm: audio::Source,
	rng: rand::ThreadRng,
	//last_spawned: u64,
	//schedule: Vec<(u64, entity::Entity)>,
	sfx: std::collections::HashMap<&'static str, audio::Source>,
	quit: bool,
	game_state: MenuState,
	life_texture: graphics::Image,

}

impl MainState {

    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let score_font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 32)?;
       
		let score_text = graphics::Text::new(ctx, "Score: ", &score_font)?;

        let mut s = MainState {
			spawner: EntitySpawner::new(ctx),
            score_text,
            frames: 0,
            entities: Vec::new(),
			input: Input {
				left: false, 
				right: false, 
				up: false,
				down: false,
				shoot: false,
			},
            score: 0,
            score_font,
			high_scores: Vec::new(),
            background: graphics::Image::new(ctx, "/texture/background_tiled.png").unwrap(),
			elapsed_ms: 0,	//Elapsed time since state creation, in milliseconds
			delta_ms: 0,	//Elapsed time since last frame, in milliseconds
			start_time:  std::time::SystemTime::now(),
			textures: std::collections::HashMap::new(),
			bgm: audio::Source::new(ctx, "/sounds/Tejaswi-Hyperbola.ogg")?,
			rng: rand::thread_rng(),
			sfx: std::collections::HashMap::new(),
			quit: false,
			game_state: MenuState::Menu,
			life_texture: graphics::Image::new(ctx, "/texture/cpu.png").unwrap(), 
		};
		
		// Set up textures
		s.textures.insert(entity::EntityType::Player, graphics::Image::new(ctx, "/texture/crab.png").unwrap() );
		s.textures.insert(entity::EntityType::Enemy, graphics::Image::new(ctx, "/texture/enemy.png").unwrap() );
		s.textures.insert(entity::EntityType::PlayerBullet, graphics::Image::new(ctx, "/texture/player_bullet.png").unwrap() );
		s.textures.insert(entity::EntityType::EnemyBullet, graphics::Image::new(ctx, "/texture/enemy_bullet.png").unwrap() );
		s.textures.insert(entity::EntityType::Powerup, graphics::Image::new(ctx, "/texture/powerup.png").unwrap() );
		s.textures.insert(entity::EntityType::Splat, graphics::Image::new(ctx, "/texture/splat.png").unwrap() );
		
		// Set up sound effects
		s.sfx.insert("player_shot", audio::Source::new(ctx, "/sounds/player_shot.wav")?);
		

		if !DISABLE_SFX {
			s.bgm.play()?;
		}
		
		//if USE_BETA_SCHEDULER {
			//schedule(& mut s, ctx);
		//}

        Ok(s)
    }
}

	
	// Call this to start a new game
	pub fn newgame(state: &mut MainState, ctx: &mut Context) {
		
		// Clear out old entities
		state.entities.clear();

		// Reset the score
		state.score = 0;

		// Create a new player object
		let player_font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", DEFAULT_FONT_SIZE);
		let player = entity::Entity {
			text: graphics::Text::new(ctx, "", &player_font.unwrap()).unwrap(),
            entity_type: entity::EntityType::Player,
		    x: (ctx.conf.window_mode.width as f32 / 2.0) - (state.textures[&entity::EntityType::Player].width() as f32 / 2.0),
            y: ctx.conf.window_mode.height as f32 - state.textures[&entity::EntityType::Player].height() as f32,
            hp: 5,
			dam: 0,
            vel: 375.0,
			bounds: graphics::Rect {
				x: 60.0,
				y: 40.0,
				w: 10.0,
				h: 18.0,
			},
			movement: Movement::None,
			lifetime: Lifetime::Forever,
			seed: 0.0,
			timer: 0,
			bullet_cooldown: PLAYER_BULLET_COOLDOWN,
			angle: 0.0,
        };
		
		state.entities.push(player);

	}
/*
// Setup the schedule
fn schedule(state: &mut MainState, ctx: &mut Context) {

	// Release a boss later?

	// Release a 10 enemies enemy on 12000 ms
 	for i in (1..10).rev() {
		//let enemy_font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 14);
		//state.schedule.push((12000 + ((i as u64) * 100_u64), gen_basic_enemy(100_f32 + (i as f32) * 100_f32 , -50_f32, 
		//	graphics::Text::new(ctx, name, &enemy_font.unwrap()).unwrap(), state.rng.gen_range(-1.0, 1.0))));
	}

	// Release like 5 enemies enemy on 5000 ms
	for i in (1..5).rev() {
		//let enemy_font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 14);
		//let name = ENEMY_NAMES[state.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
		//state.schedule.push((5000 + ((i as u64) * 100_u64), gen_basic_enemy(200_f32 + (i as f32) * 80_f32 , -50_f32, 
		//	graphics::Text::new(ctx, name, &enemy_font.unwrap()).unwrap(), state.rng.gen_range(-1.0, 1.0))));
	}


	// Release an enemy on 1000
	{
		//let enemy_font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 14);
		//let name = ENEMY_NAMES[state.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
		//state.schedule.push((1000, gen_basic_enemy(300_f32, -50_f32, 
		//	graphics::Text::new(ctx, name, &enemy_font.unwrap()).unwrap(), state.rng.gen_range(-1.0, 1.0))));
	}

	
}
*/
/*
fn scheduler(state: &mut MainState, ctx: &mut Context) {
	let mut cont : bool = true;

	while cont {
		
		cont = false;
		if let Some(entry) = state.schedule.last() {
			if entry.0 < state.elapsed_ms + SCHEDULE_OFFSET {
				cont = true;
			}
		}

		if cont {
			let (indx, ent) = state.schedule.pop().unwrap();
			println!("Releasing new enemy on schedule time: {:?}. It is time: {:?}", indx, state.elapsed_ms);
			state.entities.push(ent);
			cont = false;
		}
	}
	
}
*/

// Collision detection
fn collision_detection(state: &mut MainState) {

	// Iterate through subject entities
	for entity_idx in 0..state.entities.len() {
		match state.entities[entity_idx].entity_type {

			EntityType::Player => {
				for threat_idx in 0..state.entities.len() {
					match state.entities[threat_idx].entity_type {
						EntityType::Enemy => {
							if colliding(state, entity_idx, threat_idx) {
								state.entities[entity_idx].hp -= state.entities[threat_idx].dam;
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								//if state.entities[entity_idx].hp <= 0 {
									//state.entities[entity_idx].lifetime = Lifetime::Milliseconds(0);
								//}
							}
						},
						EntityType::EnemyBullet => {
							if colliding(state, entity_idx, threat_idx) {
								state.entities[entity_idx].hp -= state.entities[threat_idx].dam;
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								//if state.entities[entity_idx].hp <= 0 {
									//state.entities[entity_idx].lifetime = Lifetime::Milliseconds(0);
								//}
							}
						},
						EntityType::Powerup => {
							if colliding(state, entity_idx, threat_idx) {
								// Right now, the only powerup we have will destroy all enemies on the screen.
								for enemy_idx in 0..state.entities.len() {
									if state.entities[enemy_idx].entity_type == EntityType::Enemy {
										state.entities[enemy_idx].lifetime = Lifetime::Milliseconds(0);
										// Gain score points.
										state.score += 10;
									}
								}
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
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
			EntityType::Enemy => {
				for threat_idx in 0..state.entities.len() {
					match state.entities[threat_idx].entity_type {
						// See if we hit the threat
						EntityType::PlayerBullet => {
							if colliding(state, entity_idx, threat_idx) {
								state.entities[entity_idx].hp -= state.entities[threat_idx].dam;
								state.entities[threat_idx].lifetime = Lifetime::Milliseconds(0);
								if state.entities[entity_idx].hp <= 0 {
									state.entities[entity_idx].lifetime = Lifetime::Milliseconds(0);
								}
								// Gain score points
								state.score += 10;
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

		match self.game_state {
			MenuState::Menu => {
				if self.input.shoot {
					self.game_state = MenuState::Game;
					newgame(self, _ctx);
				}
				
			},
			MenuState::Game => {
				
				//if USE_BETA_SCHEDULER {
					//scheduler(self, _ctx);
				//} else {
					//enemy_spawner(self, _ctx);
				//}
				collision_detection(self);
				
				// Really crappy way to detect game over. Fix later.
				let mut found_player = false;
				
				for all_idx in 0..self.entities.len() {
					if self.entities[all_idx].entity_type == EntityType::Player {
						found_player = true;
						break;
					}
				}
				
				// If the player died, gameover!
				if !found_player {
					self.game_state = MenuState::Menu;
					self.high_scores.push(self.score);
				}

				match self.spawner.update(self.delta_ms, _ctx) {
					Some(e) => {
						self.entities.push(e);
					},
					None => (),
				}

				//self.score_tex.f //graphics::Text::new(_ctx, &format!("Score: {}", self.score), _ctx.default_font)?;

				self.score_text = graphics::Text::new(_ctx, &format!("Score: {}", &self.score.to_string()), &self.score_font).unwrap();
			
				// Run thru the list of entities
				for i in 0..self.entities.len() {
					let mut e = self.entities.remove(i);
					e.update(self, _ctx);
					self.entities.insert(i, e);
				}

				// Hacky way of showing health
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
						entity::EntityType::Enemy => self.entities.push(self.spawner.spawn_splat(x, y)),
						_ => (), 
					}
					
				}

				// Now we can just kill off stuff if it doesnt have hp.
				self.entities.retain(|e| {
					e.hp > 0
				});
		
			}
		}
		update_time(self);

		

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

		match self.game_state {
			MenuState::Menu => {
				// Draw the 2 background copies staggered according to elapsed_ms
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, 0.0 + (self.elapsed_ms/40%1920) as f32), 0.0)?;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, -1920.0 + (self.elapsed_ms/40 % 1920) as f32), 0.0)?;

				self.score_text = graphics::Text::new(ctx, &format!("Main Menu: Press Space to play game"), &self.score_font).unwrap();
				graphics::draw(ctx, &self.score_text, graphics::Point2::new(200.0, 200.0), 0.0)?;

				self.score_text = graphics::Text::new(ctx, &format!("Recent Scores:"), &self.score_font).unwrap();
				graphics::draw(ctx, &self.score_text, graphics::Point2::new(200.0, 250.0), 0.0)?;

				for i in 0 .. self.high_scores.len() {
					self.score_text = graphics::Text::new(ctx, &format!("Score: {}", self.high_scores[i]), &self.score_font).unwrap();
					graphics::draw(ctx, &self.score_text, graphics::Point2::new(200.0, 280.0 + (i as f32) * 30_f32), 0.0)?;
				}


			},
			MenuState::Game => {
				let _window_width = ctx.conf.window_mode.width;
				let _window_height = ctx.conf.window_mode.height;

				// Draw the 2 background copies staggered according to elapsed_ms
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, 0.0 + (self.elapsed_ms/40%1920) as f32), 0.0)?;
				graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, -1920.0 + (self.elapsed_ms/40 % 1920) as f32), 0.0)?;
				{
					// Draw the player's life graphics
					let player = &self.entities[0];

					for i in 0..player.hp + 1 {
						graphics::draw(
							ctx, 
							&self.life_texture, 
							graphics::Point2::new(_window_width as f32 - (self.life_texture.width() as f32 * 1.25 * i as f32), 0.0), 0.0)?;
					}
				}
				// Draw all entities
				for e in &mut self.entities {
					let pos = graphics::Point2::new(e.x, e.y);
					let texture = &self.textures[&e.entity_type];

					// Special drawing conditions start
					match e.entity_type {
						entity::EntityType::Enemy => {
							match e.hp {
								1 => graphics::set_color(ctx, graphics::Color::new(1.0, 0.1, 0.0, 1.0))?,
								2 => graphics::set_color(ctx, graphics::Color::new(0.9, 0.5, 0.0, 1.0))?,
								3 => graphics::set_color(ctx, graphics::Color::new(0.0, 1.0, 0.0, 1.0))?,
								_ => (),
							}
						},
						entity::EntityType::Player => {
							match e.hp {
								1 => graphics::set_color(ctx, graphics::Color::new(0.4, 0.0, 0.0, 0.9))?,
								2 => graphics::set_color(ctx, graphics::Color::new(0.6, 0.1, 0.1, 0.95))?,
								3 => graphics::set_color(ctx, graphics::Color::new(1.0, 0.7, 0.7, 1.0))?,
								4 => graphics::set_color(ctx, graphics::Color::new(1.0, 0.9, 0.9, 1.0))?,						
								_ => graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?,
							}
						},
						_ => {}
					}
					
					// Draw the entity sprite rotated if needed
					if e.angle == 0.0 {
						graphics::draw(ctx, texture, pos, e.angle)?;
					}  
					else {
						let half_width = texture.width() as f64 / 2.0;
						let angle = e.angle as f64 + (5.0 * std::f64::consts::PI / 4.0);
						let x = (half_width + half_width * (2.0_f64).sqrt() * angle.cos()) as f32;
						let y = (half_width + half_width * (2.0_f64).sqrt() * angle.sin()) as f32;
						graphics::draw(ctx, texture, graphics::Point2::new(e.x + x, e.y+ y), e.angle)?;
					}
				
					// End drawing conditions: Reset drawing conditions
					graphics::set_color(ctx, graphics::Color::new(1.0, 1.0, 1.0, 1.0))?;
					
					// If this is an enemy, include a name tag.
					if e.entity_type == entity::EntityType::Enemy {
						let offset = 30.0;
						let text_pos = graphics::Point2::new(
							e.x + texture.width() as f32 + offset, 
							e.y - offset);
						//	, e.y);
						graphics::draw(ctx, &e.text, text_pos, 0.0)?;
						graphics::line(ctx, &[
							graphics::Point2::new(text_pos.x - 5.0, text_pos.y + e.text.height() as f32),
							graphics::Point2::new(pos.x + texture.width() as f32, pos.y)], 1.0)?;
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

				graphics::draw(ctx, &self.score_text, graphics::Point2::new(10.0, 10.0), 0.0)?;
			},
		}

		
		graphics::present(ctx);

		self.frames += 1;
		if (self.frames % 100) == 0 {
			println!("FPS: {}", ggez::timer::get_fps(ctx));
		}
        

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

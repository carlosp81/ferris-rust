
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
const DISABLE_SFX: bool = false;

// Adjust this to start further ahead or behind in the spawn schedule
//const SCHEDULE_OFFSET: u64 = 0;
//const USE_BETA_SCHEDULER: bool = false;
const SHOW_INPUT_DEBUG: bool = false;

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

pub struct MenuState {
	
}

impl MenuState {
    pub fn new(ctx: &mut Context) -> GameResult<MenuState> {
		
        let s = MenuState {
		};
        Ok(s)
	}
}

impl event::EventHandler for MenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
	
	fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
	}
	
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
	}
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
            background: graphics::Image::new(ctx, "/texture/background_tiled.png").unwrap(),
			elapsed_ms: 0,	//Elapsed time since state creation, in milliseconds
			delta_ms: 0,	//Elapsed time since last frame, in milliseconds
			start_time:  std::time::SystemTime::now(),
			textures: std::collections::HashMap::new(),
			bgm: audio::Source::new(ctx, "/sounds/Tejaswi-Hyperbola.ogg")?,
			rng: rand::thread_rng(),
			//last_spawned: 0,
			//schedule: Vec::<(u64, entity::Entity)>::new(),
			sfx: std::collections::HashMap::new(),
			quit: false,
		};
		
		// Set up textures
		s.textures.insert(entity::EntityType::Player, graphics::Image::new(ctx, "/texture/crab.png").unwrap() );
		s.textures.insert(entity::EntityType::Enemy, graphics::Image::new(ctx, "/texture/enemy.png").unwrap() );
		s.textures.insert(entity::EntityType::PlayerBullet, graphics::Image::new(ctx, "/texture/player_bullet.png").unwrap() );
		s.textures.insert(entity::EntityType::EnemyBullet, graphics::Image::new(ctx, "/texture/enemy_bullet.png").unwrap() );
		s.textures.insert(entity::EntityType::Powerup, graphics::Image::new(ctx, "/texture/powerup.png").unwrap() );
		
		// Set up sound effects
		s.sfx.insert("player_shot", audio::Source::new(ctx, "/sounds/player_shot.wav")?);
		
		let player_font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", DEFAULT_FONT_SIZE)?;

		let player = entity::Entity {
			text: graphics::Text::new(ctx, "", &player_font)?,
            entity_type: entity::EntityType::Player,
		    x: (ctx.conf.window_mode.width as f32 / 2.0) - (s.textures[&entity::EntityType::Player].width() as f32 / 2.0),
            y: ctx.conf.window_mode.height as f32 - s.textures[&entity::EntityType::Player].height() as f32,
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
		
		s.entities.push(player);
		if !DISABLE_SFX {
			s.bgm.play()?;
		}
		
		//if USE_BETA_SCHEDULER {
			//schedule(& mut s, ctx);
		//}
	
        //let resolutions = ggez::graphics::get_fullscreen_modes(ctx, 0)?;
		
        //let (width, height) = resolutions[3];

		//ggez::graphics::set_resolution(ctx, width, height)?;
		//graphics::set_resolution(ctx, WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
		//ctx.conf.window_setup.resizable = true;
		//ctx.conf.window_mode.width = WINDOW_WIDTH as u32;
		//ctx.conf.window_mode.height = WINDOW_HEIGHT as u32;
		//graphics::set_screen_coordinates(ctx, graphics::Rect {
			//x: 0.0,
			//y: 0.0,
			//w: WINDOW_WIDTH / 2.0,
			//h: WINDOW_HEIGHT,

		//});
        Ok(s)
    }
}

// Spawns bullets for the player
fn player_bullet_spawner(state: &mut MainState, x: f32, y: f32) -> GameResult<()> {
	let bullet = entity::Entity {
		text: state.score_text.clone(),
		entity_type: entity::EntityType::PlayerBullet,
		x: x as f32 + (state.textures[&entity::EntityType::Player].width() as f32 / 2.0) - (state.textures[&entity::EntityType::PlayerBullet].width() as f32 / 2.0),
		y: y - (state.textures[&entity::EntityType::PlayerBullet].height() as f32 / 2.0),
		hp: 1,
		dam: 1,
		vel: 10.0,
		bounds: graphics::Rect {
			x: 0.0,
			y: 0.0,
			w: 50.0,
			h: 50.0,
		},
		
		movement: Movement::Linear(0.0, -BULLET_SPEED),
		//movement: Movement::Linear(0.0, -10_000.0),
		lifetime: Lifetime::Milliseconds(2_000),
		seed: 0.0,
		timer: 0,
		bullet_cooldown: 0,
		angle: 0.0,
	};
	state.entities.push(bullet);
	if !DISABLE_SFX {
		state.sfx["player_shot"].play()?;
	}
	Ok(())
}

// Spawns bullets for the enemy
fn enemy_bullet_spawner(state: &mut MainState, x: f32, y: f32) {
	let bullet = entity::Entity {
		text: state.score_text.clone(),
		entity_type: entity::EntityType::EnemyBullet,
		x: x as f32 + state.textures[&entity::EntityType::Enemy].width() as f32 / 2.0 - state.textures[&entity::EntityType::EnemyBullet].width() as f32 / 2.0,
		y: y + state.textures[&entity::EntityType::Enemy].height() as f32 / 2.0 - state.textures[&entity::EntityType::EnemyBullet].height() as f32 / 2.0,
		hp: 1,
		dam: 1,
		vel: 1000.0,
		bounds: graphics::Rect {
			x: 0.0,
			y: 0.0,
			w: 25.0,
			h: 25.0,
		},
		//movement: Movement::Linear(0.0, 7_000.0),
		movement: Movement::Linear(0.0, BULLET_SPEED),
		lifetime: Lifetime::Milliseconds(8_000),
		seed: 0.0,
		timer: 0,
		bullet_cooldown: 0,
		angle: 0.0,
	};
	state.entities.push(bullet);
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
		update_time(self);

		//if USE_BETA_SCHEDULER {
			//scheduler(self, _ctx);
		//} else {
			//enemy_spawner(self, _ctx);
		//}
		collision_detection(self);
		
		
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
			{

				// Update lifetimes
				let e = &mut self.entities[i];
				e.timer += self.delta_ms;
				e.lifetime = match e.lifetime {
					Lifetime::Forever => Lifetime::Forever,
					Lifetime::Milliseconds(remaining) => Lifetime::Milliseconds(remaining - self.delta_ms as i64),
				};

				// Process bullet cooldowns
				e.bullet_cooldown -= self.delta_ms as i64;
				if e.bullet_cooldown < 0 {
					e.bullet_cooldown = 0;
				}
			
				// Process movements
				let delta_time = self.delta_ms as f32 / 1000_f32;
				match e.movement {
					Movement::None => (),
					Movement::Linear(x,y) => e.translate(x * delta_time, y * delta_time),
					Movement::Generated(func) => {
						let (x, y) = func(e.timer, &mut self.rng, e.seed);
						e.translate(x * delta_time, y * delta_time);
					},
				}
			}

			match self.entities[i].entity_type {

				// Player only code
				// This handles the player movements
				entity::EntityType::Player => {
					let e = &mut self.entities[i];
					let vel= e.vel * (self.delta_ms as f32 / 1000_f32);
	
					match (self.input.up, self.input.right, self.input.down, self.input.left) {
						( true, false, false, false) => e.translate(0.0, -vel),
						( true,  true, false, false) => e.translate(vel*0.707, -vel*0.707),
						(false,  true, false, false) => e.translate(vel, 0.0),
						(false,  true,  true, false) => e.translate(vel*0.707, vel*0.707),
						(false, false,  true, false) => e.translate(0.0, vel),
						(false, false,  true,  true) => e.translate(-vel*0.707, vel*0.707),
						(false, false, false,  true) => e.translate(-vel, 0.0),
						( true, false, false,  true) => e.translate(-vel*0.707, -vel*0.707),
						_ => (),
					}

					// Limit player position to map.
					let window_width = _ctx.conf.window_mode.width as f32;
					let window_height = _ctx.conf.window_mode.height as f32;
					if e.x + e.bounds.x < 0.0 {
						e.x = 0.0 - e.bounds.x;
					}
					if e.x + e.bounds.x + e.bounds.w > window_width {
						e.x = window_width - (e.bounds.x + e.bounds.w);
					}
					if e.y + e.bounds.y < 0.0 {
						e.y = 0.0 - e.bounds.y;
					}
					if e.y + e.bounds.y + e.bounds.h > window_height {
						e.y = window_height - (e.bounds.y + e.bounds.h);
					}

					// Hacky way of showing health
					self.score_text = graphics::Text::new(_ctx, &format!("Score: {} || Health: {}", 
						&self.score.to_string(), e.hp), &self.score_font).unwrap();
					
				},

				// Enemy only code
				entity::EntityType::Enemy => {
					if self.entities[i].bullet_cooldown <= 0 {
						self.entities[i].bullet_cooldown = ENEMY_BULLET_COOLDOWN;
						let x = self.entities[i].x;
						let y = self.entities[i].y;
						enemy_bullet_spawner(self, x, y);
					}
				},

				// Boss only code
				entity::EntityType::Boss => (),

				// Player bullet code
				entity::EntityType::PlayerBullet => {
					let player_bullet = &mut self.entities[i];
					player_bullet.angle += self.delta_ms as f32 / 100.0;
				},

				// Enemy bullet code
				entity::EntityType::EnemyBullet => (),

				// Powerup codes
				entity::EntityType::Powerup => (),
			}
		}

		if self.input.shoot {
			if self.entities[0].bullet_cooldown == 0 {
				self.entities[0].bullet_cooldown = PLAYER_BULLET_COOLDOWN;
				let x = self.entities[0].x;
				let y = self.entities[0].y;
				player_bullet_spawner(self, x, y)?;
			}
		}
		
		// Handle dying entities
		self.entities.retain(|e| {
			let dying = match e.lifetime {
				Lifetime::Forever => e.hp <= 0,
				Lifetime::Milliseconds(r) => r <= 0 || e.hp <= 0,
			};
			if dying {
				match e.entity_type {
					_ => (),
				}
			}
			!dying
		});
		

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Drawables are drawn from their top-left corner.
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.score_text, dest_point, 0.0)?;
		
		// Draw the 2 background copies staggered according to elapsed_ms
		graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, 0.0 + (self.elapsed_ms/40%1920) as f32), 0.0)?;
		graphics::draw(ctx, &self.background, graphics::Point2::new(0.0, -1920.0 + (self.elapsed_ms/40 % 1920) as f32), 0.0)?;

		//let player_x = self.entities[0].x;
		//let player_y = self.entities[0].y;
		//println!("Player x = {}, Player y = {}", player_x / 10.0, player_y / 50.0 - 5.0);
		
		// Draw all entities
		for e in &mut self.entities {
			let pos = graphics::Point2::new(e.x, e.y);
			let texture = &self.textures[&e.entity_type];
			//let text_size_div_2 =  graphics::Point2::new(e.text.width() as f32 / 2.0, e.text.height() as f32 / 2.0);

			// Draw the entity sprite axis-aligned
			//graphics::draw(ctx, texture, pos, 0.0)?;
			
			// Normal drawing conditions
			

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
        graphics::draw(ctx, &self.score_text, graphics::Point2::new(0.0, 0.0), 0.0)?;
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
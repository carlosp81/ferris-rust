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

extern crate ggez;
extern crate rand;

use std;
use ggez::{graphics, audio};
use ggez::Context;
use game::{MainState, BOSS_BULLET_NUMBER, BOSS_BULLET_COOLDOWN, ENEMY_BULLET_COOLDOWN};
//use game::rand::Rng;

/// An enum for distinguishing game entity types
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum EntityType {
	Empty,
	Boss,
	EnemyBullet,
	PlayerBullet,
	Enemy,
	EnemyBlueScreen,
	Life,
	Player,
	Powerup,
	Splat,
	Shutoff,
}

/// Used to specify the lifetime of an entity.
/// Those with `Forever` will never expire, while
/// those with a `Milliseconds()` value will be
/// culled after that many milliseconds have elapsed.
#[derive(Debug)]
pub enum Lifetime {
	Forever,
	Milliseconds(i64),
}

/// An entity has one of three movement types:
/// - None: The entity is static on screen (text/effects)
/// - Linear: The entity has a constant x and y velocity.
/// - Generated: The entity will use the lambda function to generate an x
/// and y translation value every time it updates. The first parameter is
/// the ms elapsed since the entity spawned, the second is a random number
/// generator, and the third is a unique seed value between -1.0 and 1.0.
/// # Example
/// For an entity moving in  sine x direction
/// and stright down y direction.
/// ```
/// let g = Generated(|time,_rand,seed| {
///		let sine = ((time / 1000.0) as f64 ).sine() as f32;
///		( sine * 10.0, -10.0 )
///	}
/// ```
pub enum Movement {
	None,
	Linear(f32, f32),
	Generated(fn(u64,&mut rand::ThreadRng, f64)->(f32, f32)),
}

/// The entity structure is used to represent all
/// interactive game objects. All variables have above
/// default value, so you can create an entity with
/// only the parts you want to customize.
/// # Example
/// ```
/// let e = Entity {
/// x: 100.0,
/// y: 200.0,
/// entity_type: entity::EntityType::Enemy,
/// };
/// ```
pub struct Entity {
	pub angle: f32,
	pub bounds: graphics::Rect,
	pub bullet_cooldown: i64,
	pub damage: i32,
	pub entity_type: EntityType,
    pub hp: i32,
	pub lifetime: Lifetime,
	pub movement: Movement,
	pub name: String,
	pub seed: f64,
	pub timer: u64,
    pub vel: f32,
	pub x: f32,
    pub y: f32,	
}

/// This allows Entity struct to be created with only
/// part of their variables defined.
impl Default for Entity {
    fn default() -> Entity {
        Entity {
            angle: 0.0,
			bounds: graphics::Rect {
				x: 0.0,
				y: 0.0,
				w: 1.0,
				h: 1.0,
			},
			bullet_cooldown: 0,
			damage: 1,
			entity_type: EntityType::Empty,
			hp: 1,
			lifetime: Lifetime::Forever,
			movement: Movement::None,
			name: "empty".to_string(),
			seed: 1.0,
			timer: 0,
			vel: 0.0,
			x: 0.0,
			y: 0.0,
        }
    }
}

impl Entity {	
	/// This function moves an entity around by the pixels specified.
	/// # Example
	/// ```
	/// let e = Entity {
	/// 	x: 10.0,
	/// 	y: 10.0,
	/// };
	/// e.translate(1.0, -1.0);
	/// assert(e.x == 11.0 && e.y == 9.0);
	/// ```
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }

	/// This takes care of entity actions that happen independent of collisions.
	pub fn update(&mut self, state: &mut MainState, ctx: &mut Context) {
		
		let delta_ms = state.delta_ms;

		// Update lifetimes
		self.timer += delta_ms;
		self.lifetime = match self.lifetime {
			Lifetime::Forever => Lifetime::Forever,
			Lifetime::Milliseconds(remaining) => Lifetime::Milliseconds(remaining - delta_ms as i64),
		};

		// Process bullet cooldowns
		self.bullet_cooldown -= delta_ms as i64;
		if self.bullet_cooldown < 0 {
			self.bullet_cooldown = 0;
		}
	
		// Process movements
		let delta_time = delta_ms as f32 / 1000_f32;
		match self.movement {
			Movement::None => (),
			Movement::Linear(x,y) => self.translate(x * delta_time, y * delta_time),
			Movement::Generated(func) => {
				let (x, y) = func(self.timer, &mut state.rng.clone(), self.seed);
				self.translate(x * delta_time, y * delta_time);
				},
		}

		let mut shots_fired = false;
		
		match self.entity_type {

			// Player only code
			// This handles the player movements
			EntityType::Player => {
				
				let vel= self.vel * (delta_ms as f32 / 1000_f32);

				match (state.input.up, state.input.right, state.input.down, state.input.left) {
					( true, false, false, false) => self.translate(0.0, -vel),
					( true,  true, false, false) => self.translate(vel*0.707, -vel*0.707),
					(false,  true, false, false) => self.translate(vel, 0.0),
					(false,  true,  true, false) => self.translate(vel*0.707, vel*0.707),
					(false, false,  true, false) => self.translate(0.0, vel),
					(false, false,  true,  true) => self.translate(-vel*0.707, vel*0.707),
					(false, false, false,  true) => self.translate(-vel, 0.0),
					( true, false, false,  true) => self.translate(-vel*0.707, -vel*0.707),
					_ => (),
				}

				// Limit player position to map.
				let window_width = ctx.conf.window_mode.width as f32;
				let window_height = ctx.conf.window_mode.height as f32;

				if self.x + self.bounds.x < 0.0 {
					self.x = 0.0 - self.bounds.x;
				}
				if self.x + self.bounds.x + self.bounds.w > window_width {
					self.x = window_width - (self.bounds.x + self.bounds.w);
				}
				if self.y + self.bounds.y < 0.0 {
					self.y = 0.0 - self.bounds.y;
				}
				if self.y + self.bounds.y + self.bounds.h > window_height {
					self.y = window_height - (self.bounds.y + self.bounds.h);
				}

	
				
			},

			// Enemy only code
			EntityType::Enemy => {
				if self.bullet_cooldown <= 0 {
					shots_fired = true;
					self.bullet_cooldown = ENEMY_BULLET_COOLDOWN;
					let texture = &state.textures[&EntityType::Enemy][0];
					let bullet_x = self.x + texture.width() as f32 / 2.0 - 18.0;
					let bullet_y = self.y + texture.height() as f32;
					let eb = state.spawner.spawn_enemy_bullet(bullet_x, bullet_y, (3.0 * std::f64::consts::PI / 2.0) as f32);
					state.entities.push(eb);
				}
			},
			
			EntityType::EnemyBlueScreen => {
				if self.bullet_cooldown <= 0 {
					shots_fired = true;
					self.bullet_cooldown = ENEMY_BULLET_COOLDOWN;
					let texture = &state.textures[&EntityType::EnemyBlueScreen][0];
					let bullet_x = self.x + texture.width() as f32 / 2.0 - 18.0;
					let bullet_y = self.y + texture.height() as f32;
					{
						let eb = state.spawner.spawn_enemy_bullet(bullet_x, bullet_y, (5.0 * std::f64::consts::PI / 4.0) as f32);
						state.entities.push(eb);
					}
					{
						let eb = state.spawner.spawn_enemy_bullet(bullet_x, bullet_y, (3.0 * std::f64::consts::PI / 2.0) as f32);
						state.entities.push(eb);
					}
					{
						let eb = state.spawner.spawn_enemy_bullet(bullet_x, bullet_y, (7.0 * std::f64::consts::PI / 4.0) as f32);
						state.entities.push(eb);
					}

				}
			},

			EntityType::Boss => {
				if self.bullet_cooldown <= 0 {
					shots_fired = true;
					self.bullet_cooldown = BOSS_BULLET_COOLDOWN;
					
					let increment = (std::f64::consts::PI * 2.0 / BOSS_BULLET_NUMBER as f64) as f32;
					for i in 0..BOSS_BULLET_NUMBER {
						let angle = self.angle + increment * i as f32;
						let bullet_x = ( self.x + self.bounds.x + self.bounds.w / 2.0 ) + (self.bounds.x * 2.0 + self.bounds.w) / 2.0 * angle.cos();
						let bullet_y = ( self.y + self.bounds.y + self.bounds.h / 2.0 ) - (self.bounds.x * 2.0 + self.bounds.w) / 2.0 * angle.sin();
						let eb = state.spawner.spawn_enemy_bullet(bullet_x, bullet_y, angle);
						state.entities.push(eb);
					}
				}
				self.angle += delta_ms as f32 / 600.0;
			},
			
			// Player bullet code
			EntityType::PlayerBullet => {
				self.angle += delta_ms as f32 / 100.0;
			},

			_ => (),
		}
		
		if shots_fired {
			// Nasty means of playing enemy shot sounds quickly on the same channel. 
			*state.sfx.get_mut("enemy_shot").unwrap() = audio::Source::new(ctx, "/sounds/enemy_shot.wav").expect("Could not load enemy shot");
			state.sfx["enemy_shot"].play().unwrap();
		}
		
	}
}
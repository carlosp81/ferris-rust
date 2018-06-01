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
use ggez::{Context,graphics};
use self::rand::Rng;
use game::entity::{Lifetime, Movement, Entity, EntityType};
use game::{ENEMY_NAMES, BULLET_SPEED, SPLAT_LIFETIME, SHUTOFF_LIFETIME};
use std;

const ENEMY_COOLDOWN: i64 = 1_500;
const ENEMY_COOLDOWN_BLUESCREEN: i64 = 6_000;
const ENEMY_COOLDOWN_BOSS: i64 = 60_000;
const POWERUP_COOLDOWN: i64 = 10_000;

pub struct EntitySpawner {
	pub _screen_height: u32,
	pub screen_width: u32,
	pub rng: rand::ThreadRng,
    pub cooldowns: std::collections::HashMap<EntityType, i64>,
}

impl EntitySpawner {
    pub fn new(ctx: &Context) -> EntitySpawner {        
        let mut p = EntitySpawner {
			_screen_height: ctx.conf.window_mode.height,
			screen_width: ctx.conf.window_mode.width,
            rng: rand::thread_rng(),
            cooldowns: std::collections::HashMap::new(),
        };

        p.cooldowns.insert(EntityType::Enemy, ENEMY_COOLDOWN );
        p.cooldowns.insert(EntityType::EnemyBlueScreen, ENEMY_COOLDOWN_BLUESCREEN );
		p.cooldowns.insert(EntityType::Boss, ENEMY_COOLDOWN_BOSS );
		p.cooldowns.insert(EntityType::Powerup, POWERUP_COOLDOWN );

        p
    }

    pub fn spawn_splat(&self, x: f32, y: f32) -> Entity {
        let splat = Entity {
			name: "splat".to_string(),
            entity_type: EntityType::Splat,
            x: x,
            y: y,
            hp: 1,
            damage: 0,
            vel: 0.0,
            bounds: graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: 80.0,
                h: 80.0,
            },
            movement: Movement::None,
            lifetime: Lifetime::Milliseconds(SPLAT_LIFETIME),
            seed: 0.0,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };
        splat
    }

    pub fn spawn_shutoff(&self, x: f32, y: f32) -> Entity {
        let shutoff = Entity {
			name: "shutoff".to_string(),
            entity_type: EntityType::Shutoff,
            x: x,
            y: y,
            hp: 1,
            damage: 0,
            vel: 0.0,
            bounds: graphics::Rect {
                x: 0.0,
                y: 0.0,
                w: 80.0,
                h: 80.0,
            },
            movement: Movement::None,
            lifetime: Lifetime::Milliseconds(SHUTOFF_LIFETIME),
            seed: 0.0,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };
        shutoff
    }
    // Spawns bullets for the player
    pub fn player_bullet_spawner(&self, x: f32, y: f32) -> Entity {
		let mut bullet = Entity::default();
		bullet.x = x;
		bullet.y = y;
		bullet.bounds = graphics::Rect{
		    x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
		bullet.movement = Movement::Linear(0.0, -BULLET_SPEED);
        bullet.lifetime = Lifetime::Milliseconds(2_000);
		bullet.entity_type = EntityType::PlayerBullet;
		bullet.name = "player_bullet".to_string();
		
        bullet
    }

    // Spawns bullets for the enemy
    pub fn spawn_enemy_bullet(&self, x: f32, y: f32, angle: f32) -> Entity {
        let mut bullet = Entity::default();
        bullet.x = x;
        bullet.y = y;
		bullet.bounds = graphics::Rect{
		    x: 0.0,
            y: 0.0,
            w: 25.0,
            h: 25.0,
        };
		bullet.movement = Movement::Linear(angle.cos() * BULLET_SPEED, -angle.sin() * BULLET_SPEED);
        bullet.lifetime = Lifetime::Milliseconds(8_000);
		bullet.entity_type = EntityType::EnemyBullet;
		bullet.name = "player_bullet".to_string();
		
        bullet
    }

    pub fn spawn_enemy(&self, seed: f64, name: &str, enemy_type: u8) -> Entity {
        // Default entity
		let mut e = Entity {
			name: name.to_string(),
            entity_type: EntityType::Enemy,
            x: 0.0,
            y: 0.0,
            hp: 3,
            damage: 1,
            vel: 0.0,
        	bounds: graphics::Rect {
				x: 18.0,
				y: 5.0,
				w: 44.0,
				h: 60.0,
			},
			movement: Movement::Generated(
				|t,r,s|{
 					(
					    ( ( (t as f64) / 1000.0 + s * 1000.0 ).sin() + r.gen_range(-3.0, 3.0) ) as f32 * 60_f32,
 						(1.0 + ( (t as f64) / 900.0 + s * 100.0).sin() ) as f32 * 60_f32
 					)
 				}
			),
			lifetime: Lifetime::Milliseconds(100_000),
			seed,
			timer: 0,
			bullet_cooldown: 0,
			angle: 0.0,
        };

        // Certain enemies recieve different traits
        match enemy_type {
			// Blue screen
            2 => {
				e.name = "BSOD".to_string();
				e.entity_type = EntityType::EnemyBlueScreen;
                e.hp = 5; 
                e.movement = Movement::Generated(
                    |t,r,s|{
                        (
                            ( ( (t as f64) / 1000.0 + s * 1000.0 ).sin() + (t as f64).sin() * 2_f64 ) as f32 * 60_f32,
                            (1.0 + ( (t as f64) / 900.0 + s * 100.0).sin() + r.gen_range(0.1, 3.0)  ) as f32 * 20_f32
                        )
                    }
                );
            },
			3 => {
				e.name = "ANSI C".to_string();
				e.entity_type = EntityType::Boss;
				e.hp = 25;
				e.movement = Movement::Generated(
				    |t,_r,s|{
                        (
                            ( (t as f64) / 1000.0 + s * 1000.0 ).sin() as f32 * 60.0,
							20.0
                        )
                    }
                );
				e.bounds = graphics::Rect {
					x: 30.0,
					y: 20.0,
					w: 140.0,
					h: 130.0,
				};
			},

            _ => ()
        }
        
        // Return powerup entity option type.
        e
    }

    pub fn spawn_powerup(&self) -> Entity {
        let e = Entity {
			name: "power bomb".to_string(),
            entity_type: EntityType::Powerup,
            x: 0.0,
            y: 0.0,
            hp: 1,
            damage: 1,
            vel: 10.0,
        	bounds: graphics::Rect {
				x: 0.0,
				y: 0.0,
				w: 64.0,
				h: 64.0,
			},
			movement: Movement::Linear(0.0, 50.0),
			lifetime: Lifetime::Milliseconds(100_000),
			seed: 0.0,
			timer: 0,
			bullet_cooldown: 0,
			angle: 0.0,
        };
        // Return powerup entity option type.
        e
    }

    // Update the cooldowns on all entity types that have them. If a cooldown triggers, 
    // spawn that entity and return it.
    pub fn update(&mut self, delta_ms: u64) -> Option<Entity> {
        
        // We dont really care about matching the player type, so we use that as a dummy.
        let mut entity_type: EntityType = EntityType::Player;

        for (k, v) in self.cooldowns.iter_mut() {
            *v -= delta_ms as i64;
            if *v <= 0 {
                entity_type = k.clone();
            }
        }

        match entity_type {
            EntityType::Enemy => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, ENEMY_COOLDOWN);
                
                // Create enemy name and seed.
                let name = ENEMY_NAMES[self.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
                let seed: f64 = self.rng.gen_range(-1.0, 1.0);
                
                // Create enemy.
                let mut entity = self.spawn_enemy(seed, name, 1);
                entity.x = self.rng.gen_range(0.0, self.screen_width as f32);
                entity.y = -45.0;
                return Some(entity);
            },
            EntityType::EnemyBlueScreen => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, ENEMY_COOLDOWN_BLUESCREEN);
                
                // Create enemy name and seed.
                let name = ENEMY_NAMES[self.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
                let seed: f64 = self.rng.gen_range(-1.0, 1.0);
                
                // Create enemy.
                let mut entity = self.spawn_enemy(seed, name, 2);
                entity.x = self.rng.gen_range(0.0, self.screen_width as f32);
                entity.y = -45.0;
                return Some(entity);
            },
			EntityType::Boss => {
				// Reset cooldown.
                self.cooldowns.insert(entity_type, ENEMY_COOLDOWN_BOSS);
                
                // Create seed.
                let seed: f64 = self.rng.gen_range(-1.0, 1.0);
                
                // Create enemy.
                let mut entity = self.spawn_enemy(seed, "ANSI C", 3);
                entity.x = self.screen_width as f32 / 2.0;
                entity.y = -200.0;
                return Some(entity);
			},
            EntityType::Powerup => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, POWERUP_COOLDOWN);

                // Create powerup.
                let mut powerup = self.spawn_powerup();
                powerup.x = self.rng.gen_range(0.0, self.screen_width as f32);
                powerup.y = -45.0;
                return Some(powerup);
            },
            _ => (),
        }                

        None
    }
}

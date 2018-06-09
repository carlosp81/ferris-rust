// Copyright © 2018
// "River Bartz"<bpg@pdx.edu>
// "Daniel Dupriest"<kououken@gmail.com>
// "Brandon Goldbeck"<rbartz@pdx.edu>
// This program is licensed under the "MIT License". Please see the file
// LICENSE in the source distribution of this software for license terms.

extern crate ggez;
extern crate rand;
use ggez::{graphics, Context};
use self::rand::Rng;
use game::entity::{Lifetime, Movement, Entity, EntityType};
use game::{ENEMY_NAMES, ENEMY_BULLET_SPEED, PLAYER_BULLET_SPEED, SPLAT_LIFETIME, SHUTOFF_LIFETIME, ENEMY_LIFETIME, SECONDS_UNTIL_MAX_DIFFICULTY};
use std;

const ENEMY_COOLDOWN: i64 = 1_000;
const ENEMY_COOLDOWN_BLUESCREEN: i64 = 6_000;
const ENEMY_COOLDOWN_BOSS: i64 = 60_000;
const POWERBOMB_COOLDOWN: i64 = 45_000;
const UPGRADE_COOLDOWN: i64 = 25_000;
const SHIELD_COOLDOWN: i64 = 35_000;

/// This keeps track of cooldowns for various entity types and spawns when necessary
pub struct EntitySpawner {
    pub _screen_height: u32,
    pub screen_width: u32,
    pub rng: rand::ThreadRng,
    pub cooldowns: std::collections::HashMap<EntityType, i64>,
}

impl EntitySpawner {
	/// Create a new entity spawner.
    pub fn new(ctx: &Context) -> EntitySpawner {        
        let mut p = EntitySpawner {
            _screen_height: ctx.conf.window_mode.height,
            screen_width: ctx.conf.window_mode.width,
            rng: rand::thread_rng(),
            cooldowns: std::collections::HashMap::new(),
        };
		// Set up the basic cooldowns

        p.cooldowns.insert(EntityType::Enemy, ENEMY_COOLDOWN);
        p.cooldowns
            .insert(EntityType::EnemyBlueScreen, ENEMY_COOLDOWN_BLUESCREEN);
        p.cooldowns.insert(EntityType::Boss, ENEMY_COOLDOWN_BOSS);
        p.cooldowns.insert(EntityType::Powerbomb, POWERBOMB_COOLDOWN);
        p.cooldowns.insert(EntityType::GunUpgrade, UPGRADE_COOLDOWN);
        p.cooldowns.insert(EntityType::Shield, SHIELD_COOLDOWN);

        p
    }

	/// Generates a binary splat entity
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

	/// Generates a screen shutoff entity
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
	
    /// Spawns bullets for the player
    pub fn player_bullet_spawner(&self, x: f32, y: f32) -> Entity {
        let mut bullet = Entity::default();
        bullet.x = x;
        bullet.y = y;
        bullet.bounds = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: 50.0,
            h: 50.0,
        };
		bullet.movement = Movement::Linear(0.0, -PLAYER_BULLET_SPEED);
        bullet.lifetime = Lifetime::Milliseconds(2_000);
        bullet.entity_type = EntityType::PlayerBullet;
        bullet.name = "player_bullet".to_string();

        bullet
    }

    /// Spawns bullets for the enemy
    pub fn spawn_enemy_bullet(&self, x: f32, y: f32, angle: f32) -> Entity {
        let mut bullet = Entity::default();
        bullet.x = x;
        bullet.y = y;
        bullet.bounds = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: 25.0,
            h: 25.0,
        };
		bullet.movement = Movement::Linear(angle.cos() * ENEMY_BULLET_SPEED, -angle.sin() * ENEMY_BULLET_SPEED);
        bullet.lifetime = Lifetime::Milliseconds(8_000);
        bullet.entity_type = EntityType::EnemyBullet;
        bullet.name = "player_bullet".to_string();

        bullet
    }

	/// Spawns an enemy entity of a specific type.
    pub fn spawn_enemy(&self, seed: f64, name: &str, enemy_type: EntityType) -> Entity {
        // Default entity
        let mut e = Entity {
            name: name.to_string(),
            entity_type: EntityType::Enemy,
            x: 0.0,
            y: 0.0,
            hp: 1,
            damage: 1,
            vel: 0.0,
            bounds: graphics::Rect {
                x: 18.0,
                y: 5.0,
                w: 44.0,
                h: 60.0,
            },
            movement: Movement::Generated(|t, r, s| {
                (
                    (((t as f64) / 1000.0 + s * 1000.0).sin() + r.gen_range(-3.0, 3.0)) as f32
                        * 60_f32,
                    (1.0 + ((t as f64) / 900.0 + s * 100.0).sin()) as f32 * 60_f32,
                )
            }),
            lifetime: Lifetime::Milliseconds(ENEMY_LIFETIME),
            seed,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };

        // Certain enemies recieve different traits
        match enemy_type {
			// Blue screen
            EntityType::EnemyBlueScreen => {
				e.name = "BSOD".to_string();
				e.entity_type = EntityType::EnemyBlueScreen;
                e.hp = 4;
                e.movement = Movement::Generated(
                    |t,r,s|{
                        (
                            ( ( (t as f64) / 1000.0 + s * 300.0 ).sin() + (t as f64).sin() * 2_f64 ) as f32 * 60_f32,
                            (1.0 + ( (t as f64) / 900.0 + s * 100.0).sin() + r.gen_range(0.1, 3.0)  ) as f32 * 20_f32
                        )
                    }
                );
            },
			EntityType::Boss => {
				e.name = "ANSI C".to_string();
				e.entity_type = EntityType::Boss;
				e.hp = 40;
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

	/// Spawns a power bomb
    pub fn spawn_powerbomb(&self) -> Entity {
        let e = Entity {
            name: "power bomb".to_string(),
            entity_type: EntityType::Powerbomb,
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
            movement: Movement::Linear(0.0, 100.0),
            lifetime: Lifetime::Milliseconds(100_000),
            seed: 0.0,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };
        // Return powerbomb entity option type.
        e
    }


    pub fn spawn_gunupgrade(&self) -> Entity {
        let e = Entity {
            name: "gun upgrade".to_string(),
            entity_type: EntityType::GunUpgrade,
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
            movement: Movement::Linear(0.0, 100.0),
            lifetime: Lifetime::Milliseconds(100_000),
            seed: 0.0,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };
        // Return upgrade entity option type.
        e
    }

    pub fn spawn_shield(&self) -> Entity {
        let e = Entity {
            name: "firewall".to_string(),
            entity_type: EntityType::Shield,
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
            movement: Movement::Linear(0.0, 100.0),
            lifetime: Lifetime::Milliseconds(100_000),
            seed: 0.0,
            timer: 0,
            bullet_cooldown: 0,
            angle: 0.0,
        };
        // Return shield entity option type.
        e
    }

    /// Update the cooldowns on all entity types that have them. If a cooldown triggers,
    /// spawn that entity and return it.
    pub fn update(&mut self, elapsed_ms: u64, delta_ms: u64) -> Option<Entity> {
        // We dont really care about matching the player type, so we use that as a dummy.
        let mut entity_type: EntityType = EntityType::Player;

        for (k, v) in self.cooldowns.iter_mut() {
            *v -= delta_ms as i64;
            if *v <= 0 {
                entity_type = k.clone();
            }
        }

		let mut difficulty_factor = (SECONDS_UNTIL_MAX_DIFFICULTY - elapsed_ms / 1000) as f32 / SECONDS_UNTIL_MAX_DIFFICULTY as f32;
		if difficulty_factor < 0.1 {
			difficulty_factor = 0.1;
		}
		
        match entity_type {
            EntityType::Enemy => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, (ENEMY_COOLDOWN as f32 * difficulty_factor) as i64);

                // Create enemy name and seed.
                let name = ENEMY_NAMES[self.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
                let seed: f64 = self.rng.gen_range(-1.0, 1.0);

                // Create enemy.
                let mut entity = self.spawn_enemy(seed, name, EntityType::Enemy);
                entity.x = self.rng.gen_range(0.0, self.screen_width as f32);
                entity.y = -70.0;
                return Some(entity);
            }
            EntityType::EnemyBlueScreen => {
                // Reset cooldown.
                self.cooldowns
                    .insert(entity_type, (ENEMY_COOLDOWN_BLUESCREEN as f32 * difficulty_factor) as i64);

                // Create enemy name and seed.
                let name = ENEMY_NAMES[self.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
                let seed: f64 = self.rng.gen_range(-1.0, 1.0);

                // Create enemy.
                let mut entity = self.spawn_enemy(seed, name, EntityType::EnemyBlueScreen);
                entity.x = self.rng.gen_range(0.0, self.screen_width as f32);
                entity.y = -70.0;
                return Some(entity);
            }
            EntityType::Boss => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, (ENEMY_COOLDOWN_BOSS as f32 * difficulty_factor) as i64);

                // Create seed.
                let seed: f64 = self.rng.gen_range(-1.0, 1.0);

                // Create enemy.
                let mut entity = self.spawn_enemy(seed, "ANSI C", EntityType::Boss);
                entity.x = self.screen_width as f32 / 2.0;
                entity.y = -200.0;
                return Some(entity);
            }
            EntityType::Powerbomb => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, POWERBOMB_COOLDOWN);

                // Create Powerbomb.
                let mut powerbomb = self.spawn_powerbomb();
                powerbomb.x = self.rng.gen_range(0.0, self.screen_width as f32);
                powerbomb.y = -45.0;
                return Some(powerbomb);
            }
            EntityType::GunUpgrade => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, UPGRADE_COOLDOWN);

                // Create upgrade.
                let mut upgrade = self.spawn_gunupgrade();
                upgrade.x = self.rng.gen_range(0.0, self.screen_width as f32);
                upgrade.y = -45.0;
                return Some(upgrade);
            }
            EntityType::Shield => {
                // Reset cooldown.
                self.cooldowns.insert(entity_type, SHIELD_COOLDOWN);

                // Create shield.
                let mut shield = self.spawn_shield();
                shield.x = self.rng.gen_range(0.0, self.screen_width as f32);
                shield.y = -45.0;
                return Some(shield);
            }
            _ => (),
        }

        None
    }
}
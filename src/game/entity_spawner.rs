
extern crate ggez;
extern crate rand;
use ggez::{Context, GameResult};
use ggez::GameError;
use ggez::{audio, graphics};
use self::rand::Rng;
use game::MainState;
use game::entity::Lifetime;
use game::entity::Movement;
use game::entity::Entity;
use game::entity::EntityType;
use game::DEFAULT_FONT;
use std;

const ENEMY_FONT_SIZE: u32 = 18;
const ENEMY_COOLDOWN: i64 = 1_000;
const POWERUP_COOLDOWN: i64 = 30_000;
const ENEMY_NAMES: [&str;4] = [
	"NULL POINTER",
	"DANGLING REF",
	"SEGFAULT",
	"DOUBLE FREE",
];

pub struct EntitySpawner {
    //pub max_cooldown: i64,
    //pub current_cooldown: i64,
    pub text: graphics::Text,
    pub rng: rand::ThreadRng,
    pub cooldowns: std::collections::HashMap::<EntityType, i64>,
}

impl EntitySpawner {
    pub fn new(cooldown: i64, ctx: &mut Context) -> EntitySpawner {
        
        let font = graphics::Font::new(ctx, DEFAULT_FONT, 48);
        let text = graphics::Text::new(ctx, "", &font.unwrap()).unwrap();

        let mut p = EntitySpawner {
            //max_cooldown: cooldown,
            //current_cooldown: cooldown,
            text,
            rng: rand::thread_rng(),
            cooldowns: std::collections::HashMap::new(),
        };

        p.cooldowns.insert(EntityType::Enemy, ENEMY_COOLDOWN );
        p.cooldowns.insert(EntityType::Powerup, POWERUP_COOLDOWN );

        p
    }

    pub fn spawn_enemy(&mut self, ctx: &mut Context) -> Entity {
        let font = graphics::Font::new(ctx, DEFAULT_FONT, ENEMY_FONT_SIZE);
        let name = ENEMY_NAMES[self.rng.gen::<usize>() % ENEMY_NAMES.len()].clone();
		let text = graphics::Text::new(ctx, name, &font.unwrap()).unwrap();
		let e = Entity {
            text: text,
            entity_type: EntityType::Enemy,
            x: 0.0,
            y: 0.0,
            hp: 3,
            dam: 1,
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
			seed: self.rng.gen_range(-1.0, 1.0),
			timer: 0,
			bullet_cooldown: 0,
			angle: 0.0,
        };
        // Return powerup entity option type.
        e
    }

    pub fn spawn_powerup(&mut self) -> Entity {

        let e = Entity {
            text: self.text.clone(),
            entity_type: EntityType::Enemy,
            x: 0.0,
            y: 0.0,
            hp: 1,
            dam: 1,
            vel: 0.0,
        	bounds: graphics::Rect {
				x: 0.0,
				y: 0.0,
				w: 32.0,
				h: 32.0,
			},
			movement: Movement::Linear(0.0, 700.0),
			lifetime: Lifetime::Milliseconds(100_000),
			seed: self.rng.gen_range(-1.0, 1.0),
			timer: 0,
			bullet_cooldown: 0,
			angle: 0.0,
        };
        // Return powerup entity option type.
        e
    }

    pub fn update(&mut self, delta_ms: u64, ctx: &mut Context) -> Option<Entity> {
        // Spawn enemies
        let current = self.cooldowns[&EntityType::Enemy];
        self.cooldowns.insert(EntityType::Enemy, current - delta_ms as i64);
        if self.cooldowns[&EntityType::Enemy] <= 0 {
            self.cooldowns.insert(EntityType::Enemy, ENEMY_COOLDOWN);
            let mut e = self.spawn_enemy(ctx);
            e.x = self.rng.gen_range(0.0, ctx.conf.window_mode.width as f32);
            e.y = -30.0;
            return Some(e);
        }

        // Spawn powerups
        let current = self.cooldowns[&EntityType::Powerup];
        self.cooldowns.insert(EntityType::Powerup, current - delta_ms as i64);
        if self.cooldowns[&EntityType::Powerup] <= 0 {
            self.cooldowns.insert(EntityType::Powerup, POWERUP_COOLDOWN);
            let mut e = self.spawn_powerup();
            e.x = self.rng.gen_range(0.0, ctx.conf.window_mode.width as f32);
            e.y = -10.0;
            return Some(e);
        }

        /*
        for (k,v) in self.cooldowns.iter() {
            if v.0 <= 0 {
                v.0 = v.1;
                match k {
                    EntityType::Enemy => {
                        let mut e = self.spawn_enemy();
                        e.x = self.rng.gen_range(0.0, ctx.conf.window_mode.width as f32);
                        e.y = -30.0;
                        return Some(e);
                    },
                    EntityType::Powerup => {
                        let mut p = self.spawn_powerup();
                        p.x = self.rng.gen_range(0.0, ctx.conf.window_mode.width as f32);
                        p.y = -10.0;
                        return Some(p);
                    },
                    _ => (),
                }
            }
            v.0 -= delta_ms as i64;
        }*/
        None
    }
}
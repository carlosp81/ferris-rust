
extern crate ggez;
extern crate rand;

use ggez::graphics;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
	Boss,
	EnemyBullet,
	PlayerBullet,
	Enemy,
	Player,
	Powerup,
}

#[derive(Debug)]
pub enum Lifetime {
	Forever,
	Milliseconds(i64),
}

// An entity has one of three movement types:
// - None: The entity is static on screen (text/effects)
// - Linear: The entity has a constant x and y velocity.
// - Generated: The entity will use the lambda function to generate an x
// and y translation value every time it updates. The first parameter is
// the ms elapsed since the entity spawned, the second is a random number
// generator, and the third is a unique seed value between -1.0 and 1.0.
pub enum Movement {
	None,
	Linear(f32, f32),
	Generated(fn(u64,&mut rand::ThreadRng, f64)->(f32, f32)),
}

pub struct Entity {
	pub text: graphics::Text,
	pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub hp: u8,
	pub dam: u8,
    pub vel: f32,
	pub movement: Movement,
	pub bounds: graphics::Rect,
	pub lifetime: Lifetime,
	pub seed: f64,
	pub timer: u64,
	pub bullet_cooldown: i64,
	pub angle: f32,
}

impl Entity {
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}
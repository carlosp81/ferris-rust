


extern crate ggez;

use ggez::graphics;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EntityType {
	Boss,
	Bullet,
	Enemy,
	Player,
}

#[derive(Debug)]
pub enum Lifetime {
	Forever,
	Milliseconds(i64),
}

pub enum Movement {
	None,
	Linear(f32, f32),
	Generated(fn(u64)->(f32, f32)),
}

pub struct Entity {
	pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
    pub hp: u8,
    pub vel: f32,
	pub movement: Movement,
	pub bounds: graphics::Rect,
	pub lifetime: Lifetime,
	pub timer: u64,
}

impl Entity {
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}
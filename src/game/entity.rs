


extern crate ggez;

use ggez::graphics;

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

pub struct Entity {
	pub entity_type: EntityType,
    pub sprite: graphics::Image,
    pub x: f32,
    pub y: f32,
    pub hp: u8,
    pub vel: f32,
	pub bounds: graphics::Rect,
	pub lifetime: Lifetime,
	}

impl Entity {
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}



extern crate ggez;

use ggez::graphics;

pub enum EntityType {
	Boss,
	Bullet,
	Enemy,
	Player,
}

pub struct Entity {
	pub entity_type: EntityType,
    pub sprite: graphics::Image,
    pub x: f32,
    pub y: f32,
    pub hp: u8,
    pub vel: f32,
	pub bounds: (f32, f32),	// (width, height) of bounding box, centered on middle of image
}

impl Entity {
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}
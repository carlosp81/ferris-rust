


extern crate ggez;

use ggez::graphics;


pub struct Movement {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub struct Entity {
    pub sprite: graphics::Image,
    pub x: f32,
    pub y: f32,
    pub hp: u8,
    pub mov: Movement,
    pub vel: f32,

}


impl Entity {
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}


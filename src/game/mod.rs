

extern crate ggez;
extern crate rand;

use ggez::{Context, GameResult};
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ggez::graphics;
use std;

mod entity;

use self::entity::{Lifetime, EntityType, Movement};
use self::rand::Rng;

const DRAW_BOUNDING_BOXES: bool = true;

struct Input {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
	shoot: bool,
}

// First we make a structure to contain the game's state
pub struct MainState {
    score_text: graphics::Text,
    frames: usize,
    entities: Vec<entity::Entity>,
	input: Input,
    score: u32,
    font: graphics::Font,
    background: graphics::Image,
	elapsed_ms: u64,
	delta_ms: u64,
	textures: std::collections::HashMap::<entity::EntityType, graphics::Image>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 48)?;
        let score_text = graphics::Text::new(ctx, "Score: ", &font)?;
		
        let mut s = MainState {
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
            font,
            background: graphics::Image::new(ctx, "/texture/background_tiled.png").unwrap(),
			elapsed_ms: 0,	//Elapsed time since state creation, in milliseconds
			delta_ms: 0,	//Elapsed time since last frame, in milliseconds
			textures: std::collections::HashMap::new(),
		};
		
		s.textures.insert(entity::EntityType::Player, graphics::Image::new(ctx, "/texture/crab.png").unwrap() );
		s.textures.insert(entity::EntityType::Enemy, graphics::Image::new(ctx, "/texture/null_pointer_enemy.png").unwrap() );
		
		
		let mut player = entity::Entity {
            entity_type: entity::EntityType::Player,
		    x: (ctx.conf.window_mode.width as f32 / 2.0) - (s.textures[&entity::EntityType::Player].width() as f32 / 2.0),
            y: ctx.conf.window_mode.height as f32 - s.textures[&entity::EntityType::Player].height() as f32,
            hp: 100,
            vel: 250.0,
			bounds: graphics::Rect {
				x: 30.0,
				y: 10.0,
				w: 196.0,
				h: 151.0,
			},
			movement: Movement::None,
			lifetime: Lifetime::Forever,
			timer: 0,
        };
		let mut rng = rand::thread_rng();
		for i in 0..20 {		
			let mut enemy = entity::Entity {
				entity_type: entity::EntityType::Enemy,
				x: 0.0 + 35.0 * i as f32,
				y: 0.0,
				hp: 1,
				vel: 100.0,
				bounds: graphics::Rect {
					x: 10.0,
					y: 10.0,
					w: 80.0,
					h: 80.0,
				},
				movement: Movement::Linear(
					rng.gen_range(-600.0, 600.0),
					rng.gen_range(300.0, 1000.0),
				),
				lifetime: Lifetime::Milliseconds(10_000),
				timer: 0,
			};
			s.entities.push(enemy);
		}
		s.entities.push(player);
        Ok(s)
    }
}

// Update state's elapsed ms and delta ms
fn update_time(state: &mut MainState) {
	let now = std::time::SystemTime::now();
	let difference = now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
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
        
		update_time(self);
		
        //self.score_tex.f //graphics::Text::new(_ctx, &format!("Score: {}", self.score), _ctx.default_font)?;

        self.score_text = graphics::Text::new(_ctx, &format!("Score: {}", &self.score.to_string()), &self.font).unwrap();
		
		for e in &mut self.entities {
			e.timer += self.delta_ms;
			e.lifetime = match e.lifetime {
				Lifetime::Forever => Lifetime::Forever,
				Lifetime::Milliseconds(remaining) => Lifetime::Milliseconds(remaining - self.delta_ms as i64),
			};

			match e.movement {
				Movement::None => (),
				Movement::Linear(x,y) => e.translate(x / 1000_f32, y / 1000_f32),
				Movement::Generated(func) => {
					let (x, y) = func(e.timer);
					e.translate(x, y);
				},
			}
			match e.entity_type {
				entity::EntityType::Player => {
					let vel= e.vel * ((self.delta_ms as f32) / 1000_f32);
	
					if self.input.left {
						e.translate(-vel, 0.0);
					}
					if self.input.right {
						e.translate(vel, 0.0);
					}
					if self.input.up {
						e.translate(0.0, -vel);
					}
					if self.input.down {
						e.translate(0.0, vel);
					}
					if self.input.shoot {
						// TODO: Spawn bullets.
					}
				},
				entity::EntityType::Enemy => (),
				entity::EntityType::Boss => (),
				entity::EntityType::Bullet => (),
			}
		}
		
		// Kill off dead entities
		self.entities.retain(|e| match e.lifetime {
			Lifetime::Forever => true,
			Lifetime::Milliseconds(r) => r > 0,
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

		// Draw all entities
		for e in &mut self.entities {
			let pos = graphics::Point2::new(e.x, e.y);
			graphics::draw(ctx, &self.textures[&e.entity_type], pos, 0.0)?;
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
		
        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
    
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
        
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
    }
    
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
        
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
		}
    }
}
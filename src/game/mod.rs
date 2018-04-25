

extern crate ggez;


use ggez::{Context, GameResult};
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ggez::graphics;

mod entity;

// First we make a structure to contain the game's state
pub struct MainState {
    score_text: graphics::Text,
    frames: usize,
    player: entity::Entity,
    score: u32,
    font: graphics::Font,
    
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/font/FiraSans-Regular.ttf", 48)?;
        let score_text = graphics::Text::new(ctx, "Score: ", &font)?;

        let s = MainState {
            score_text,
            frames: 0,
            player: entity::Entity {
                sprite: graphics::Image::new(ctx, "/texture/crab.png").unwrap(),
                x: 0.0,
                y: 0.0,
                hp: 100,
                mov: entity::Movement {
                    left: false, 
                    right: false, 
                    up: false,
                    down: false},
                vel: 10.0,
            },
            score: 0,
            font,
        };
        Ok(s)
    }
    
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        
        //self.score_tex.f //graphics::Text::new(_ctx, &format!("Score: {}", self.score), _ctx.default_font)?;

        self.score_text = graphics::Text::new(_ctx, &format!("Score: {}", &self.score.to_string()), &self.font).unwrap();

        let vel= self.player.vel;
        if self.player.mov.left {
            self.player.translate(-vel, 0.0);
        }
        if self.player.mov.right {
            self.player.translate(vel, 0.0);
        }
        if self.player.mov.up {
            self.player.translate(0.0, -vel);
        }
        if self.player.mov.down {
            self.player.translate(0.0, vel);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Drawables are drawn from their top-left corner.
        let dest_point = graphics::Point2::new(10.0, 10.0);
        let player_pos = graphics::Point2::new(self.player.x, self.player.y);
        graphics::draw(ctx, &self.score_text, dest_point, 0.0)?;
        
        graphics::draw(ctx, &self.player.sprite, player_pos, 0.0)?;

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
            self.player.mov.left = true;
            
        }      
        if keycode == ggez::event::Keycode::Right {
            self.player.mov.right = true;
            
        }
        if keycode ==  ggez::event::Keycode::Up {
            self.player.mov.up = true;
            
        }
        if keycode ==  ggez::event::Keycode::Down {
            self.player.mov.down = true;
        }
    }
    
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!(
            "Key released: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
        
        if keycode == ggez::event::Keycode::Left {
            self.player.mov.left = false;
            
        }      
        if keycode == ggez::event::Keycode::Right {
            self.player.mov.right = false;
            
        }
        if keycode ==  ggez::event::Keycode::Up {
            self.player.mov.up = false;
            
        }
        if keycode ==  ggez::event::Keycode::Down {
            self.player.mov.down = false;
        }
    }
}
    

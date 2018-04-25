// Basic hello world example.

extern crate ggez;

use ggez::conf;
use ggez::{Context, GameResult};
use ggez::event::{self, Button, MouseState, Keycode, Mod, Axis};
use ggez::graphics;
use std::env;
use std::path;

mod game;

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("assets");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut game::MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}

/*
extern crate piston_window;
extern crate find_folder;
extern crate time;

use time::{Duration, PreciseTime};
use piston_window::*;

const WINDOW_WIDTH: f64 = 1024.0;
const WINDOW_HEIGHT: f64 = 1024.0;



pub struct Movement {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
}

pub struct Entity {
    pub tex: G2dTexture,
    pub x: f64,
    pub y: f64,
    pub hp: u8,
    pub mov: Movement,

}

impl Entity {
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

struct Game {
    score: u32,
    player: Entity,
    window: PistonWindow,

}

impl Game {

    fn restart(&mut self) {
        self.window.set_lazy(true);
        self.score = 0;

        self.player.x = WINDOW_WIDTH / 2.0 - (self.player.tex.get_width() as f64 / 2.0);
        self.player.y = WINDOW_HEIGHT - self.player.tex.get_height() as f64;
    }

    fn update(&mut self) {
        println!("Update");
        if self.player.mov.left {
            self.player.translate(-15.0, 0.0);
        }
        if self.player.mov.right {
            self.player.translate(15.0, 0.0);
        }
        if self.player.mov.up {
            self.player.translate(0.0, -15.0);
        }
        if self.player.mov.down {
            self.player.translate(0.0, 15.0);
        }


    }


    fn start_loop(&mut self) {

        let mut start = PreciseTime::now();
        let mut running = true;
    
        while running { 
            //if start.to(PreciseTime::now()) < Duration::nanoseconds(16) {
                //continue;
            //}
            start = PreciseTime::now();
            
            self.update();
            if let Some(e) = self.window.next() {
                  if let Some(button) = e.release_args() {
                    
                        println!("Release? {:?}", button);
                    use piston_window::Button::Keyboard;
                    use piston_window::Key;

                    if button == Keyboard(Key::Left) {
                        self.player.mov.left = false;
                        
                    }      
                    if button == Keyboard(Key::Right) {
                        self.player.mov.right = false;
                        
                    }
                    if button == Keyboard(Key::Up) {
                        self.player.mov.up = false;
                        
                    }
                    if button == Keyboard(Key::Down) {
                        self.player.mov.down = false;
                        
                    }
                }  
                if let Some(button) = e.press_args() {
                    use piston_window::Button::Keyboard;
                    use piston_window::Key;
                    
                    if button == Keyboard(Key::Left) {
                        self.player.mov.left = true;
                        println!("MOVE LEFT");
                    }      
                    if button == Keyboard(Key::Right) {
                        self.player.mov.right = true;
                        println!("MOVE RIGHT");
                    }
                    if button == Keyboard(Key::Up) {
                        self.player.mov.up = true;
                        println!("MOVE UP");
                    }
                    if button == Keyboard(Key::Down) {
                        self.player.mov.down = true;
                        println!("MOVE DOWN");
                    }
                    
                }
              
                
                {
                    let p : &Entity = &self.player;

                    self.window.draw_2d(&e, |c, g| {
                        println!("Draw");
                        // Render.
                        clear([1.0; 4], g);

                        let player_transform = c.rot_rad(0.0)
                                                .transform.trans(p.x, p.y);
                                                
                        image(&p.tex, player_transform, g);
                    });
                } 
            }
        }
    }



}


fn main() {
    let mut win: PistonWindow =
        WindowSettings::new("piston: image", [1024, 1024])
        .exit_on_esc(true)
        .opengl(OpenGL::V3_2)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets/texture").unwrap();

    let crab = assets.join("crab.png");
    

    let mut game = Game {
        score: 0,
        player: Entity {
            tex: Texture::from_path(
                        &mut win.factory,
                        &crab,
                        Flip::None,
                        &TextureSettings::new())
                        .unwrap(),
            x: 0.0,
            y: 0.0,
            hp: 100,
            mov: Movement {
                left: false, 
                right: false, 
                up: false,
                down: false},
        },
        window: win,
    };


    

    //println!("player x,y,hp {},{},{}", player.x, player.y, player.hp);

    
    //let mut scene = Scene::new();
    game.restart();
    game.start_loop();

    //game_loop(&mut window, &mut player);
    
}
*/
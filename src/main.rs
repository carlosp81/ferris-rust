extern crate piston_window;
extern crate find_folder;

use piston_window::*;



pub struct Player {
    pub tex: G2dTexture,
    pub x: f64,
    pub y: f64,
    pub hp: u8,

}

impl Player {
    fn change_position(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

struct Game {
    score: u32,
    player: Player,
    window: PistonWindow,
}

impl Game {

    fn restart(&mut self) {
        self.window.set_lazy(true);
        self.score = 0;
    }

    fn update(&mut self) {

     
    }


    fn start_loop(&mut self) {

        while let Some(e) = self.window.next() {    
            {
                let p : &Player = &self.player;

                self.window.draw_2d(&e, |c, g| {
                    // Render.
                    clear([1.0; 4], g);

                    let player_transform = c.rot_rad(0.0)
                                            .transform.trans(p.x, p.y);
                                            
                    image(&p.tex, player_transform, g);
                });
            }
            if let Some(button) = e.press_args() {
                use piston_window::Button::Keyboard;
                use piston_window::Key;

                if button == Keyboard(Key::Left) {
                    self.player.change_position(-5.0, 0.0);
                    println!("MOVE LEFT");
                }      
                if button == Keyboard(Key::Right) {
                    self.player.change_position(5.0, 0.0);
                    println!("MOVE RIGHT");
                }
                if button == Keyboard(Key::Up) {
                    self.player.change_position(0.0, -5.0);
                    println!("MOVE UP");
                }
                if button == Keyboard(Key::Down) {
                    self.player.change_position(0.0, 5.0);
                    println!("MOVE DOWN");
                }
            }
        }
    }
}


fn main() {
    let mut win: PistonWindow =
        WindowSettings::new("piston: image", [1024, 1024])
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets/texture").unwrap();

    let crab = assets.join("crab.png");
    
    let mut game = Game {
        score: 0,
        player: Player {
            tex: Texture::from_path(
                        &mut win.factory,
                        &crab,
                        Flip::None,
                        &TextureSettings::new())
                        .unwrap(),
            x: 0.0,
            y: 0.0,
            hp: 100,
        },
        window: win,
    };


    

    //println!("player x,y,hp {},{},{}", player.x, player.y, player.hp);

    
    //let mut scene = Scene::new();
    game.restart();
    game.start_loop();

    //game_loop(&mut window, &mut player);
    
}
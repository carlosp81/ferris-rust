/*
Copyright <2018> <River Bartz, Daniel Dupriest, Brandon Goldbeck>

Permission is hereby granted, free of charge, to any person obtaining a copy of this 
software and associated documentation files (the "Software"), to deal in the Software 
without restriction, including without limitation the rights to use, copy, modify, 
merge, publish, distribute, sublicense, and/or sell copies of the Software, and to 
permit persons to whom the Software is furnished to do so, subject to the following 
conditions:

The above copyright notice and this permission notice shall be included in all copies
or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR 
PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE 
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR 
OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER 
DEALINGS IN THE SOFTWARE.
*/
extern crate ggez;

use ggez::conf;
use ggez::{Context};
use ggez::event::{self};

use std::env;
use std::path;
use std::fs::File;

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
    //let c = conf::Conf::new();
    let mut file = match File::open("conf.toml") {
        Ok(f) => f,
        Err(e) => {
          println!("{:?}", e);
          std::process::exit(1);
        },
    };
    let c = match conf::Conf::from_toml_file(&mut file) {
        Ok(f) => f,
        Err(e) => {
          println!("{:?}", e);
          std::process::exit(1);
	},
    };

    let ctx = &mut Context::load_from_conf("Ferris Crustacean's Day Off", "ggez", c).unwrap();

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

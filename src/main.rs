// Copyright © 2018
// "River Bartz"<bpg@pdx.edu>
// "Daniel Dupriest"<kououken@gmail.com>
// "Brandon Goldbeck"<rbartz@pdx.edu>
// This program is licensed under the "MIT License". Please see the file
// LICENSE in the source distribution of this software for license terms.

extern crate ggez;

use ggez::conf;
use ggez::Context;
use ggez::event;

use std::env;
use std::path;
use std::fs::File;

mod game;

/// Our main function, which does three things:
///
/// * First, create a new `ggez::conf::Conf`
/// object which contains configuration info on things such
/// as screen resolution and window title.
/// * Second, create a `ggez::game::Game` object which will
/// do the work of creating our MainState and running our game.
/// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    // Load settings from conf.toml
    let mut file = match File::open("conf.toml") {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
    };
    let c = match conf::Conf::from_toml_file(&mut file) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1);
        }
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

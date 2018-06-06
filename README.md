# Ferris

Copyright (C) 2018 River Bartz, Daniel Dupriest, Brandon Goldbeck

A game written for an Intro to Rust course at Portland State University. Defend programming land from space bug infestation!

(See the wiki for features roadmap)

## Installation

### Download Windows binary

The game requires Visual C++ Runtime 2015, which can be downloaded from Microsoft at https://www.microsoft.com/en-us/download/details.aspx?id=52685 if you don't have it installed already.

1. Download the game's latest binary version from https://github.com/bgoldbeck/ferris-rust/releases.

### Compiling from source

The game should build under Windows or Linux with the stable version of Rust.

1. Clone the project using git.

`git clone https://github.com/bgoldbeck/cs410p-project.git`

2. Run with `--release` option for full speed.

`cargo run --release`

To create your own standalone executable the file structure must match the following:

- resources/
  - font/ & contents
  - sounds/ & contents
  - texture/ & contents
- conf.toml
- ferris.exe (or 'ferris')
- SDL2.dll

## How to play

`Space` to shoot, `up`, `down`, `left`, `right` to move.

Grab power bombs to clear the screen.

## Troubleshooting

* **SdlError("Could not create GL context")** - If you use open source MESA drivers for your video card, there may be a compatibility issue with versions 17.2 and up. See [this ggez issue](https://github.com/ggez/ggez/issues/194) for details. It seems downgrading to 17.1 may be a temporary fix. It is not clear if the issue has been resolved by version 18.0.4.
* **Shader support** - Some computers with integrated graphics may not support the necessary shaders for SDL2.
* **Resolution issues** - If you encounter screen resolution issues you might try editing `conf.toml` and changing the window mode line to read `fullscreen_type = "Desktop"`.

## Licensing

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.

## Media attribution

* Intro and background music - "[Hyperbola](http://sampleswap.org/mp3/song.php?id=432)" by [TranceAddict](http://sampleswap.org/artist/TranceAddict) - [creative commons license](https://creativecommons.org/licenses/by-sa/3.0/)
* Ferris sprite - based on artwork by [Karen Rustad Tölva](http://rustacean.net) - [public domain](https://creativecommons.org/publicdomain/zero/1.0/)

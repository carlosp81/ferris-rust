# Ferris

Copyright (C) 2018 River Bartz, Daniel Dupriest, Brandon Goldbeck

A game written for an Intro to Rust course at Portland State University. Defend programming land from space bug infestation!

(See the wiki for features roadmap)

## Installation

### Windows
The game requires Visual C++ Runtime 2015, which can be downloaded from Microsoft at https://www.microsoft.com/en-us/download/details.aspx?id=52685 if you don't have it installed already.

### Win64 binary

The latest binary version is available from at https://github.com/bgoldbeck/ferris-rust/releases.

#### File/directory layout

To run standalone the file structure must match the following:

- resources/
  - font/ & contents
  - sounds/ & contents
  - texture/ & contents
- conf.toml
- ferris.exe
- SDL2.dll

### Compiling from source

1. Clone the project using git.

`git clone https://github.com/bgoldbeck/cs410p-project.git`

2. Compile with `--release` option for full speed.

`cargo run --release`

### Linux
-- TODO

### Mac
Not supported



## How to play

`Space` to shoot, `up`, `down`, `left`, `right` to move.

Grab power bombs to clear the screen.

## Troubleshooting

If your executable crashes, your video card may not support the necessary shaders for SDL2. You might try editing `conf.toml` and changing the window mode line to read `fullscreen_type = "Desktop"`.

## Licensing

This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.

## Media attribution

* Intro and background music - "[Hyperbola](http://sampleswap.org/mp3/song.php?id=432)" by [TranceAddict](http://sampleswap.org/artist/TranceAddict) - [creative commons license](https://creativecommons.org/licenses/by-sa/3.0/)
* Ferris sprite - based on artwork by [Karen Rustad Tölva](http://rustacean.net) - [public domain](https://creativecommons.org/publicdomain/zero/1.0/)

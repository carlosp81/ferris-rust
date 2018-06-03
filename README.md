# Ferris
Copyright (C) 2018 River Bartz, Daniel Dupriest, Brandon Goldbeck

Defend programming land from space bug infestation!

## Installation

The game requires Visual C++ Runtime 2015, which can be downloaded from Microsoft at https://www.microsoft.com/en-us/download/details.aspx?id=52685 if you don't have it installed already.

### Win64 binary

The latest binary version is available from at https://github.com/bgoldbeck/ferris-rust/releases.

### Compiling from source

1. Clone the project using git.

`git clone https://github.com/bgoldbeck/cs410p-project.git`

2. Compile with `--release` option for full speed.

`cargo run --release`

#### File/directory layout

To run standalone the file structure must match the following:

- resources/
  - font/ & contents
  - sounds/ & contents
  - texture/ & contents
- conf.toml
- ferris.exe
- SDL2.dll

## How to Play

`Space` to shoot, `up`, `down`, `left`, `right` to move.

Grab power bombs to clear the screen.

## Troubleshooting

If your executable crashes, your video card may not support the necessary shaders for SDL2. You might try editing `conf.toml` and changing the window mode line to read `fullscreen_type = "Desktop"`.

## License
This program is licensed under the "MIT License". Please see the file LICENSE in the source distribution of this software for license terms.

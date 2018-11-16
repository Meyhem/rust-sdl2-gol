# rust-sdl2-gol
Rust Conways Game of Life built on SDL2

# Build & Run
1. clone repo
2. install SDL2 dev libraries (https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries)
3. cargo run

# Customize
Fiddle with variables at beginning of main

let mat_width = 192;          // number of cells horizontally

let mat_height = 102;         // number of cells vertically

let screen_width = 1920u16;   // width of window in pixels

let screen_height = 1080u16;  // height of window in pixels

let prob_fill = 0.05;         // Initial random fill (0.00-1.00) Probability of cell beign alive at start


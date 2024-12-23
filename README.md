# Flooder

## About

Flooder is a colorful game where a player tries to fill the board with a single 
color by changing the color of the tile in the upper-left corner. This new color 
spreads to all neighboring tiles that matched the original color. It’s similar to 
the Flood It mobile game. This program also has a colorblind mode, which uses 
shapes instead of flat colored tiles. It relies on the recursive flood fill algorithm 
to paint the board and works similarly to the “paint bucket” or “fill” tool in many 
painting applications.

## Running the project
* Install Rust: [rustup.rs](https://rustup.rs/)
* Clone the repository locally:
  * `git clone https:://github.com/AndrewRosenfrisk/flooder`
  * `cd flooder`
* Build the project with `cargo build`
* Run the project with `cargo run`

Based on the project detailed in the "[Big Book of Small Python Projects](https://inventwithpython.com/bigbookpython/project28.html)"

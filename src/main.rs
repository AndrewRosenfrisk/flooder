use core::fmt;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{
        Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    io::{stdin, stdout},
};

const HEART: &str = "♥";
const DIAMOND: &str = "♦";
const SPADE: &str = "♠";
const CLUB: &str = "♣";
const BALL: &str = "●";
const TRIANGLE: &str = "▲";
const BLOCK: &str = "█";
const LEFT_AND_RIGHT: &str = "─";
const UP_AND_DOWN: &str = "│";
const RIGHT_AND_DOWN: &str = "┌";
const DOWN_AND_LEFT: &str = "┐";
const UP_AND_RIGHT: &str = "└";
const UP_AND_LEFT: &str = "┘";

const HEIGHT: u16 = 14;
const WIDTH: u16 = 16;
const MOVE_LIMIT: u16 = 20;

fn main() {
    let settings: Settings;
    println!("Choose your mode: [S]hapes, [C]olors, or [B]oth:");

    'mode_input: loop {
        let mut mode_input = String::new();
        stdin().read_line(&mut mode_input).unwrap();

        match mode_input.trim().to_uppercase().as_str() {
            "S" => {
                settings = Settings { mode: Mode::SHAPE };
                break 'mode_input;
            }
            "C" => {
                settings = Settings { mode: Mode::COLOR };
                break 'mode_input;
            }
            "B" => {
                settings = Settings { mode: Mode::BOTH };
                break 'mode_input;
            }
            _ => {
                println!("Invalid entry. Please try again.");
                continue;
            }
        }
    }

    execute!(
        stdout(),
        MoveTo(0, 0),
        Clear(All),
        Clear(Purge),
        Hide,
        DisableLineWrap
    )
    .unwrap();

    let mut board = Board::new();
    let mut moves_left = MOVE_LIMIT;

    'main: loop {
        execute!(stdout(), MoveTo(0, 0), Clear(All), Clear(Purge)).unwrap();

        board.display_board(&settings);

        println!("\nMoves left: {}", moves_left);

        let input = get_player_input(&settings);

        if input == InputOptions::Quit {
            break 'main;
        }

        if settings.mode == Mode::COLOR {
            board.change_tile_by_color(0, 0, input.get_color().unwrap(), None);
        }
        if settings.mode == Mode::SHAPE || settings.mode == Mode::BOTH {
            board.change_tile_by_shape(0, 0, input.get_shape().unwrap(), None);
        }

        moves_left -= 1;
        let origin = board.board.get(&Point { x: 0, y: 0 }).unwrap().clone();
        let loss = board.board.iter().any(|(_point, tile)| *tile != origin);

        if !loss {
            board.display_board(&settings);
            println!("You have won!!!");
            break 'main;
        } else if moves_left == 0 {
            board.display_board(&settings);
            println!("You have run out of moves :(");

            break 'main;
        }
    }
}

#[derive(Debug)]
struct Settings {
    mode: Mode,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Tile {
    shape: InputShape,
    color: InputColor,
}

impl Tile {
    fn new() -> Tile {
        let mut rng = thread_rng();
        match rng.gen_range(0..=5) {
            0 => Tile {
                shape: InputShape::Heart,
                color: InputColor::Red,
            },
            1 => Tile {
                shape: InputShape::Triangle,
                color: InputColor::Green,
            },
            2 => Tile {
                shape: InputShape::Diamond,
                color: InputColor::Blue,
            },
            3 => Tile {
                shape: InputShape::Ball,
                color: InputColor::Yellow,
            },
            4 => Tile {
                shape: InputShape::Club,
                color: InputColor::Cyan,
            },
            _ => Tile {
                shape: InputShape::Spade,
                color: InputColor::Magenta,
            },
        }
    }
    fn color(color: InputColor) -> Tile {
        match color {
            InputColor::Red => Tile {
                shape: InputShape::Heart,
                color: InputColor::Red,
            },
            InputColor::Green => Tile {
                shape: InputShape::Triangle,
                color: InputColor::Green,
            },
            InputColor::Blue => Tile {
                shape: InputShape::Diamond,
                color: InputColor::Blue,
            },
            InputColor::Yellow => Tile {
                shape: InputShape::Ball,
                color: InputColor::Yellow,
            },
            InputColor::Cyan => Tile {
                shape: InputShape::Club,
                color: InputColor::Cyan,
            },
            InputColor::Magenta => Tile {
                shape: InputShape::Spade,
                color: InputColor::Magenta,
            },
        }
    }
    fn shape(shape: InputShape) -> Tile {
        match shape {
            InputShape::Heart => Tile {
                shape: InputShape::Heart,
                color: InputColor::Red,
            },
            InputShape::Triangle => Tile {
                shape: InputShape::Triangle,
                color: InputColor::Green,
            },
            InputShape::Diamond => Tile {
                shape: InputShape::Diamond,
                color: InputColor::Blue,
            },
            InputShape::Ball => Tile {
                shape: InputShape::Ball,
                color: InputColor::Yellow,
            },
            InputShape::Club => Tile {
                shape: InputShape::Club,
                color: InputColor::Cyan,
            },
            _ => Tile {
                shape: InputShape::Spade,
                color: InputColor::Magenta,
            },
        }
    }
    fn display_color(&self) -> Color {
        match self.color {
            InputColor::Red => Color::Red,
            InputColor::Green => Color::Green,
            InputColor::Blue => Color::Blue,
            InputColor::Yellow => Color::Yellow,
            InputColor::Cyan => Color::Cyan,
            InputColor::Magenta => Color::Magenta,
        }
    }
}

impl fmt::Display for InputShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            InputShape::Heart => HEART,
            InputShape::Triangle => TRIANGLE,
            InputShape::Diamond => DIAMOND,
            InputShape::Ball => BALL,
            InputShape::Club => CLUB,
            InputShape::Spade => SPADE,
            InputShape::Block => BLOCK,
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, PartialEq)]
enum Mode {
    COLOR,
    SHAPE,
    BOTH,
}

struct Board {
    board: HashMap<Point, Tile>,
}

impl Board {
    fn new() -> Board {
        let mut rng = thread_rng();
        let mut board = HashMap::new();
        for x in 0..=WIDTH {
            for y in 0..=HEIGHT {
                board.insert(Point { x, y }, Tile::new());
            }
        }

        for _ in 0..=(WIDTH * HEIGHT) {
            let x = rng.gen_range(0..(WIDTH - 2));
            let y = rng.gen_range(0..(HEIGHT - 1));
            let clone = board.get(&Point { x, y }).unwrap().clone();

            if let Some(tile) = board.get_mut(&Point { x: x + 1, y }) {
                *tile = clone;
            }
        }

        Board { board }
    }

    fn display_board(&self, settings: &Settings) {
        let header =
            RIGHT_AND_DOWN.to_owned() + &LEFT_AND_RIGHT.repeat(1 + WIDTH as usize) + DOWN_AND_LEFT;
        let footer =
            UP_AND_RIGHT.to_owned() + &LEFT_AND_RIGHT.repeat(1 + WIDTH as usize) + UP_AND_LEFT;
        // print header
        execute!(
            stdout(),
            MoveTo(0, 0),
            SetForegroundColor(Color::White),
            Print(header)
        )
        .unwrap();

        for y in 0..=HEIGHT {
            let left_margin = if y == 0 { ">" } else { UP_AND_DOWN };
            // Print left margin
            execute!(
                stdout(),
                MoveTo(0, y + 1),
                SetForegroundColor(Color::White),
                Print(left_margin)
            )
            .unwrap();

            for x in 0..=WIDTH {
                let node = self.board.get_key_value(&Point { x, y }).unwrap();
                //Print board node
                execute!(
                    stdout(),
                    MoveTo(node.0.x + 1, node.0.y + 1),
                    SetForegroundColor(
                        if settings.mode == Mode::COLOR || settings.mode == Mode::BOTH {
                            node.1.display_color()
                        } else {
                            Color::White
                        }
                    ),
                    Print(
                        if settings.mode == Mode::SHAPE || settings.mode == Mode::BOTH {
                            node.1.shape
                        } else {
                            InputShape::Block
                        }
                    )
                )
                .unwrap();
            }
            //Print right margin
            execute!(
                stdout(),
                SetForegroundColor(Color::White),
                Print(UP_AND_DOWN)
            )
            .unwrap();
        }
        //Print footer
        execute!(
            stdout(),
            MoveTo(0, HEIGHT + 2),
            SetForegroundColor(Color::White),
            Print(footer)
        )
        .unwrap();
    }

    fn change_tile_by_color(
        &mut self,
        x: u16,
        y: u16,
        search_color: InputColor,
        _replacement_tile: Option<InputColor>,
    ) {
        let origin = self.board.get(&Point { x: x, y: y }).unwrap().clone();
        if x == 0 && y == 0 && origin.color == search_color {
            //do nothing if origin's color matches the color to search and replace
            return;
        } else {
            //if we chose a non matching color, replace the origin's color with what we provided
            self.board
                .insert(Point { x: x, y: y }, Tile::color(search_color));

            //then check neighbors for match against the origin's color and repeat
            if x > 0 && self.board.get(&Point { x: x - 1, y }).unwrap().color == origin.color {
                self.change_tile_by_color(x - 1, y, search_color, Some(origin.color));
            }
            if y > 0 && self.board.get(&Point { x, y: y - 1 }).unwrap().color == origin.color {
                self.change_tile_by_color(x, y - 1, search_color, Some(origin.color));
            }
            if x < WIDTH && self.board.get(&Point { x: x + 1, y }).unwrap().color == origin.color {
                self.change_tile_by_color(x + 1, y, search_color, Some(origin.color));
            }
            if y < HEIGHT && self.board.get(&Point { x, y: y + 1 }).unwrap().color == origin.color {
                self.change_tile_by_color(x, y + 1, search_color, Some(origin.color));
            }
        }
    }

    fn change_tile_by_shape(
        &mut self,
        x: u16,
        y: u16,
        search_shape: InputShape,
        _replacement_tile: Option<InputShape>,
    ) {
        let origin = self.board.get(&Point { x: x, y: y }).unwrap().clone();
        if x == 0 && y == 0 && origin.shape == search_shape {
            //do nothing if origin's shape matches the shape to search and replace
            return;
        } else {
            //if we chose a non matching shape, replace the origin's shape with what we provided
            self.board
                .insert(Point { x: x, y: y }, Tile::shape(search_shape));

            //then check neighbors for match against the origin's shape and repeat
            if x > 0 && self.board.get(&Point { x: x - 1, y }).unwrap().shape == origin.shape {
                self.change_tile_by_shape(x - 1, y, search_shape, Some(origin.shape));
            }
            if y > 0 && self.board.get(&Point { x, y: y - 1 }).unwrap().shape == origin.shape {
                self.change_tile_by_shape(x, y - 1, search_shape, Some(origin.shape));
            }
            if x < WIDTH && self.board.get(&Point { x: x + 1, y }).unwrap().shape == origin.shape {
                self.change_tile_by_shape(x + 1, y, search_shape, Some(origin.shape));
            }
            if y < HEIGHT && self.board.get(&Point { x, y: y + 1 }).unwrap().shape == origin.shape {
                self.change_tile_by_shape(x, y + 1, search_shape, Some(origin.shape));
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum InputOptions {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Heart,
    Triangle,
    Diamond,
    Ball,
    Club,
    Spade,
    Quit,
}
impl InputOptions {
    fn get_color(&self) -> Option<InputColor> {
        match self {
            InputOptions::Red => Some(InputColor::Red),
            InputOptions::Green => Some(InputColor::Green),
            InputOptions::Blue => Some(InputColor::Blue),
            InputOptions::Yellow => Some(InputColor::Yellow),
            InputOptions::Cyan => Some(InputColor::Cyan),
            InputOptions::Magenta => Some(InputColor::Magenta),
            _ => None,
        }
    }
    fn get_shape(&self) -> Option<InputShape> {
        match self {
            InputOptions::Heart => Some(InputShape::Heart),
            InputOptions::Triangle => Some(InputShape::Triangle),
            InputOptions::Diamond => Some(InputShape::Diamond),
            InputOptions::Ball => Some(InputShape::Ball),
            InputOptions::Club => Some(InputShape::Club),
            InputOptions::Spade => Some(InputShape::Spade),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum InputShape {
    Heart,
    Triangle,
    Diamond,
    Ball,
    Club,
    Spade,
    Block,
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum InputColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

fn get_player_input(settings: &Settings) -> InputOptions {
    let mut response: Option<InputOptions>;
    print!("\nChoose one of the following ");
    if settings.mode == Mode::COLOR {
        println!("[R]ed, [G]reen, [B]lue, [Y]ellow, [C]yan, [M]agenta, or [Q]uit:");
    } else {
        println!("[H]eart, [T]riangle, [D]iamond, [B]all, [C]lub, [S]pade, or [Q]uit:");
    }

    'input: loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_uppercase();

        if settings.mode == Mode::COLOR {
            response = match input.as_str() {
                "R" => Some(InputOptions::Red),
                "G" => Some(InputOptions::Green),
                "B" => Some(InputOptions::Blue),
                "Y" => Some(InputOptions::Yellow),
                "C" => Some(InputOptions::Cyan),
                "M" => Some(InputOptions::Magenta),
                "Q" => Some(InputOptions::Quit),
                _ => None,
            };
        } else {
            response = match input.as_str() {
                "H" => Some(InputOptions::Heart),
                "T" => Some(InputOptions::Triangle),
                "D" => Some(InputOptions::Diamond),
                "B" => Some(InputOptions::Ball),
                "C" => Some(InputOptions::Club),
                "S" => Some(InputOptions::Spade),
                "Q" => Some(InputOptions::Quit),
                _ => None,
            };
        }

        if response.is_none() {
            println!("Invalid selection, please try again.");
            continue 'input;
        } else {
            break 'input;
        }
    }

    response.unwrap()
}

// The board.rs implements the tilemap/game board for minesweeper.

use cli_table::Table;
use colored::Colorize;
use rand::{distributions::Uniform, Rng};
use std::{collections::HashSet, fmt};

const SIZE_OF_BOARD: usize = 4;
const NO_OF_MINES: usize = 4;

// RenderCoordinates hold the bottom left position relative to the game window.
#[derive(Debug, Clone)]
pub struct Coordinates {
    x: f32,
    y: f32,
}

// GameObject is the type of cell based on what object it holds.
#[derive(Clone)]
pub enum GameObject {
    EMPTY,
    NEIGHBOUR,
    MINE,
}

// Cell is the single entity on the game board.
#[derive(Debug, Clone)]
pub struct BoardCell {
    coordinates: Coordinates,
    cell_type: GameObject,
}

// Board is the collection of all the cells.
pub struct Board(Vec<Vec<BoardCell>>);

impl BoardCell {
    fn new() -> Self {
        BoardCell {
            coordinates: Coordinates { x: 0., y: 0. },
            cell_type: GameObject::EMPTY,
        }
    }
}

impl Board {
    // Constructor for the board sets the game objects & initializes the game.
    pub fn new() -> Self {
        Board(vec![vec![BoardCell::new(); SIZE_OF_BOARD]; SIZE_OF_BOARD]).place_mines(NO_OF_MINES)
    }

    // Chooses random cells to place the mines on the board.
    fn place_mines(mut self, no_of_mines: usize) -> Self {
        let mut random_coord: HashSet<(usize, usize)> = HashSet::new();
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, SIZE_OF_BOARD - 1);

        while random_coord.len() < no_of_mines {
            let coord = (rng.sample(&range), rng.sample(&range));
            if random_coord.insert(coord) {
                self.0[coord.0][coord.1].cell_type = GameObject::MINE;
            }
        }
        println!("Log: hashsets => {:?}", random_coord);
        self
    }
}

impl fmt::Debug for GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MINE => write!(f, "{}", "*".red().bold()),
            Self::NEIGHBOUR => write!(f, "{}", "N".blue()),
            Self::EMPTY => write!(f, " "),
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table_vec: Vec<Vec<String>> = Vec::new();
        for row in &self.0 {
            let mut row_vec: Vec<String> = Vec::new();
            for column in row {
                row_vec.push(format!("{:?}", column.cell_type));
            }
            table_vec.push(row_vec);
        }
        write!(f, "{}", table_vec.table().display().unwrap())
    }
}

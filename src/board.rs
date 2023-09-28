// The board.rs implements the tilemap/game board for minesweeper.

use cli_table::Table;
use std::fmt;

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
    pub fn new() -> Self {
        Board(vec![vec![BoardCell::new(); 4]; 4])
    }
}

impl fmt::Debug for GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MINE => write!(f, "*"),
            Self::NEIGHBOUR => write!(f, "N"),
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

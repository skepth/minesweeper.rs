// The board.rs implements the tilemap/game board for minesweeper.

use cli_table::Table;
use colored::Colorize;
use rand::{distributions::Uniform, Rng};
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

// RenderCoordinates hold the bottom left position relative to the game window.
#[derive(Debug, Clone, PartialEq)]
struct Coordinates {
    x: f32,
    y: f32,
}

// GameObject is the type of cell based on what object it holds.
#[derive(Clone, PartialEq)]
enum GameObject {
    EMPTY,
    NEIGHBOUR(u32),
    MINE,
}

// Cell is the single entity on the game board.
#[derive(Debug, Clone, PartialEq)]
struct BoardCell {
    coordinates: Coordinates,
    cell_type: GameObject,
}

impl BoardCell {
    fn new() -> Self {
        BoardCell {
            coordinates: Coordinates { x: 0., y: 0. },
            cell_type: GameObject::EMPTY,
        }
    }
}

// Board is the collection of all the cells.
#[derive(PartialEq)]
pub struct Board(Vec<Vec<BoardCell>>);

impl Board {
    // Constructor for the board sets the game objects & initializes the game.
    fn new(size: usize) -> Self {
        Board(vec![vec![BoardCell::new(); size]; size])
    }
}

// Board buildler struct.
pub struct BoardBuilder {
    board_size: usize,
    no_of_mines: usize,
    mines: HashSet<(usize, usize)>,
    neighbours: HashMap<(usize, usize), u32>,
    board: Board,
}

impl BoardBuilder {
    // Constructor for the board sets the game objects & initializes the game.
    pub fn new(size: usize, mines: usize) -> Self {
        BoardBuilder {
            board_size: size,
            no_of_mines: mines,
            mines: HashSet::new(),
            neighbours: HashMap::new(),
            board: Board::new(size),
        }
    }

    // Chooses random cells to place the mines on the board.
    pub fn place_mines(mut self) -> Self {
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, self.board_size - 1);

        while self.mines.len() < self.no_of_mines {
            let coord = (rng.sample(&range), rng.sample(&range));
            if self.mines.insert(coord) {
                self.board.0[coord.0][coord.1].cell_type = GameObject::MINE;
            }
        }
        println!("Log: hashsets => {:?}", self.mines);
        self
    }

    // Places neighbours on the cells adjacent to mines.
    pub fn compute_neighbours(mut self) -> Self {
        for &(x, y) in self.mines.iter() {
            // (x-1, y-1)
            match self.board.0.get_mut(x.wrapping_sub(1)) {
                Some(row) => match row.get_mut(y.wrapping_sub(1)) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }

            // (x, y-1)
            match self.board.0.get_mut(x) {
                Some(row) => match row.get_mut(y.wrapping_sub(1)) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }

            // (x+1, y-1)
            match self.board.0.get_mut(x.wrapping_add(1)) {
                Some(row) => match row.get_mut(y.wrapping_sub(1)) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }

            // (x-1, y)
            match self.board.0.get_mut(x.wrapping_sub(1)) {
                Some(row) => match row.get_mut(y) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }

            // (x+1, y)
            match self.board.0.get_mut(x.wrapping_add(1)) {
                Some(row) => match row.get_mut(y) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }

            // (x-1, y+1)
            match self.board.0.get_mut(x.wrapping_sub(1)) {
                Some(row) => match row.get_mut(y.wrapping_add(1)) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }

            // (x, y+1)
            match self.board.0.get_mut(x) {
                Some(row) => match row.get_mut(y.wrapping_add(1)) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }

            // (x+1, y+1)
            match self.board.0.get_mut(x.wrapping_add(1)) {
                Some(row) => match row.get_mut(y.wrapping_add(1)) {
                    Some(cell) => match cell.cell_type {
                        GameObject::EMPTY => cell.cell_type = GameObject::NEIGHBOUR(1),
                        GameObject::NEIGHBOUR(count) => {
                            cell.cell_type = GameObject::NEIGHBOUR(count + 1)
                        }
                        GameObject::MINE => (),
                    },
                    None => (),
                },
                None => (),
            }
            //
        }

        self
    }

    // Builder patter for the board.
    pub fn build(self) -> Board {
        self.board
    }
}

impl fmt::Debug for GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MINE => write!(f, "{}", "*".red().bold()),
            Self::NEIGHBOUR(count) => write!(f, "{}", count.to_string().blue()),
            Self::EMPTY => write!(f, " "),
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table_vec: Vec<Vec<String>> = Vec::new();
        for (x, row) in self.0.iter().enumerate() {
            let mut row_vec: Vec<String> = Vec::new();
            for (y, column) in row.iter().enumerate() {
                row_vec.push(format!("{:?}\n({}, {})", column.cell_type, x, y));
            }
            table_vec.push(row_vec);
        }
        write!(f, "{}", table_vec.table().display().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn top_left_mine_neighbours() {
        let mut test_board_builder = BoardBuilder::new(4, 1);
        let input_mines_set: HashSet<(usize, usize)> = vec![(0, 0)].into_iter().collect();

        test_board_builder.board.0[0][0].cell_type = GameObject::MINE;
        test_board_builder.mines = input_mines_set;

        let mut want = Board::new(4);
        want.0[0][0].cell_type = GameObject::MINE;
        want.0[0][1].cell_type = GameObject::NEIGHBOUR(1);
        want.0[1][0].cell_type = GameObject::NEIGHBOUR(1);
        want.0[1][1].cell_type = GameObject::NEIGHBOUR(1);

        let got = test_board_builder.compute_neighbours().build();
        assert_eq!(
            want, got,
            "\n\nNeighbours are incorrect:\nWant:\n{:?}\n\nGot:\n{:?}",
            want, got
        );
    }

    #[test]
    fn top_right_mine_neighbours() {
        let mut test_board_builder = BoardBuilder::new(4, 1);
        let input_mines_set: HashSet<(usize, usize)> = vec![(0, 3)].into_iter().collect();

        test_board_builder.board.0[0][3].cell_type = GameObject::MINE;
        test_board_builder.mines = input_mines_set;

        let mut want = Board::new(4);
        want.0[0][3].cell_type = GameObject::MINE;
        want.0[0][2].cell_type = GameObject::NEIGHBOUR(1);
        want.0[1][2].cell_type = GameObject::NEIGHBOUR(1);
        want.0[1][3].cell_type = GameObject::NEIGHBOUR(1);

        let got = test_board_builder.compute_neighbours().build();
        assert_eq!(
            want, got,
            "\n\nNeighbours are incorrect:\nWant:\n{:?}\n\nGot:\n{:?}",
            want, got
        );
    }

    #[test]
    fn bottom_right_mine_neighbours() {
        let mut test_board_builder = BoardBuilder::new(4, 1);
        let input_mines_set: HashSet<(usize, usize)> = vec![(3, 3)].into_iter().collect();

        test_board_builder.board.0[3][3].cell_type = GameObject::MINE;
        test_board_builder.mines = input_mines_set;

        let mut want = Board::new(4);
        want.0[3][3].cell_type = GameObject::MINE;
        want.0[3][2].cell_type = GameObject::NEIGHBOUR(1);
        want.0[2][2].cell_type = GameObject::NEIGHBOUR(1);
        want.0[2][3].cell_type = GameObject::NEIGHBOUR(1);

        let got = test_board_builder.compute_neighbours().build();
        assert_eq!(
            want, got,
            "\n\nNeighbours are incorrect:\nWant:\n{:?}\n\nGot:\n{:?}",
            want, got
        );
    }

    #[test]
    fn bottom_left_mine_neighbours() {
        let mut test_board_builder = BoardBuilder::new(4, 1);
        let input_mines_set: HashSet<(usize, usize)> = vec![(3, 0)].into_iter().collect();

        test_board_builder.board.0[3][0].cell_type = GameObject::MINE;
        test_board_builder.mines = input_mines_set;

        let mut want = Board::new(4);
        want.0[3][0].cell_type = GameObject::MINE;
        want.0[2][0].cell_type = GameObject::NEIGHBOUR(1);
        want.0[2][1].cell_type = GameObject::NEIGHBOUR(1);
        want.0[3][1].cell_type = GameObject::NEIGHBOUR(1);

        let got = test_board_builder.compute_neighbours().build();
        assert_eq!(
            want, got,
            "\n\nNeighbours are incorrect:\nWant:\n{:?}\n\nGot:\n{:?}",
            want, got
        );
    }

    #[test]
    fn center_mine_neighbours() {
        let mut test_board_builder = BoardBuilder::new(4, 1);
        let input_mines_set: HashSet<(usize, usize)> = vec![(2, 1)].into_iter().collect();

        test_board_builder.board.0[2][1].cell_type = GameObject::MINE;
        test_board_builder.mines = input_mines_set;

        let mut want = Board::new(4);
        want.0[2][1].cell_type = GameObject::MINE;
        want.0[1][0].cell_type = GameObject::NEIGHBOUR(1);
        want.0[1][1].cell_type = GameObject::NEIGHBOUR(1);
        want.0[1][2].cell_type = GameObject::NEIGHBOUR(1);
        want.0[2][2].cell_type = GameObject::NEIGHBOUR(1);
        want.0[3][2].cell_type = GameObject::NEIGHBOUR(1);
        want.0[3][1].cell_type = GameObject::NEIGHBOUR(1);
        want.0[3][0].cell_type = GameObject::NEIGHBOUR(1);
        want.0[2][0].cell_type = GameObject::NEIGHBOUR(1);

        let got = test_board_builder.compute_neighbours().build();
        assert_eq!(
            want, got,
            "\n\nNeighbours are incorrect:\nWant:\n{:?}\n\nGot:\n{:?}",
            want, got
        );
    }
}

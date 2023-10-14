// game.rs renders the minesweeper game using bevy engine.

mod board;

const SIZE_OF_BOARD: usize = 9;
const NO_OF_MINES: usize = 10;

use bevy::prelude::*;

pub struct MinesweeperPlugin;

impl Plugin for MinesweeperPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_systems(Update, debug_board);
    }
}

fn debug_board(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::D) {
        let b = board::BoardBuilder::new(SIZE_OF_BOARD, NO_OF_MINES)
            .place_mines()
            .compute_neighbours()
            .build();
        println!("{:?}", b);
    }
}

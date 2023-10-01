use bevy::prelude::*;

mod board;

const SIZE_OF_BOARD: usize = 4;
const NO_OF_MINES: usize = 4;

fn debug_board(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::D) {
        let b = board::BoardBuilder::new(SIZE_OF_BOARD, NO_OF_MINES)
            .place_mines()
            .compute_neighbours()
            .build();
        println!("{:?}", b);
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, debug_board)
        .run()
}

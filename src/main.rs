use bevy::prelude::*;

mod board;

fn debug_board(keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::D) {
        let b = board::Board::new();
        println!("{:?}", b);
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, debug_board)
        .run()
}

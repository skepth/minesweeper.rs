use bevy::prelude::*;

mod game;

fn main() {
    App::new().add_plugins(game::MinesweeperPlugin).run()
}

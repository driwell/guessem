use bevy::prelude::*;
use guessem::{generate_number, keyboard_input_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, generate_number)
        .add_systems(Update, keyboard_input_system)
        .run();
}

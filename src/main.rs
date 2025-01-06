use bevy::prelude::*;
use guessem::{keyboard_input_system, setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.4, 0.2, 0.3)))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input_system)
        .run();
}

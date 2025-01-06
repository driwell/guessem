use bevy::prelude::*;
use guessem::hello;

fn main() {
    App::new().add_systems(Update, hello).run();
}

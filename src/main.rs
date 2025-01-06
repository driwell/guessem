use bevy::prelude::*;
use guessem::play;

fn main() {
    App::new().add_systems(Update, play).run();
}

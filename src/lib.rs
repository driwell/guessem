use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

use rand::Rng;

#[derive(Component)]
struct Number(i32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Computer;

pub fn generate_number(mut commands: Commands) {
    let random_number = rand::thread_rng().gen_range(1..=100);
    commands.spawn((Computer, Number(random_number)));
}

pub fn keyboard_input_system(mut events: EventReader<KeyboardInput>) {
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        match &event.logical_key {
            Key::Character(character) => {
                if character.parse::<i32>().is_ok() {
                    println!("pressed {character}");
                }
            }
            _ => continue,
        }
    }
}

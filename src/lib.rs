use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

use rand::Rng;

#[derive(Component)]
pub struct Text(String);

#[derive(Component)]
struct Number(i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Computer;

pub fn generate_number(mut commands: Commands) {
    let random_number = rand::thread_rng().gen_range(1..=100);
    commands.spawn((Computer, Number(random_number)));
    commands.spawn((Player, Text("".to_string())));
}

pub fn keyboard_input_system(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<&mut Text, With<Player>>,
) {
    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        match &event.logical_key {
            Key::Backspace => {
                for mut text in &mut query {
                    text.0.pop();
                    println!("{}", text.0);
                }
            }
            Key::Character(character) => {
                if character.parse::<i32>().is_ok() {
                    for mut text in &mut query {
                        text.0.push_str(character);
                        println!("{}", text.0);
                    }
                }
            }
            _ => continue,
        }
    }
}

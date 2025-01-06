use std::cmp::Ordering;

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
};

use rand::Rng;

#[derive(Component)]
pub struct Number(i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Computer;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let random_number = rand::thread_rng().gen_range(1..=100);
    commands.spawn((Computer, Number(random_number)));
    commands.spawn((Player, Text("".to_string())));
    commands.spawn((Player, Number(0)));
}

// TODO: Separate this into multiple stuff
pub fn keyboard_input_system(
    mut events: EventReader<KeyboardInput>,
    mut text: Query<&mut Text, With<Player>>,
    mut guess: Query<&mut Number, With<Player>>,
    mut number: Query<&mut Number, (With<Computer>, Without<Player>)>,
) {
    let mut text = text.single_mut();
    let mut guess = guess.single_mut();
    let number = number.single_mut();

    for event in events.read() {
        if !event.state.is_pressed() {
            continue;
        }

        match &event.logical_key {
            Key::Enter => {
                if text.0.is_empty() {
                    continue;
                }

                guess.0 = text.0.parse::<i32>().unwrap();

                match guess.0.cmp(&number.0) {
                    Ordering::Less => println!("Higher"),
                    Ordering::Greater => println!("Lower"),
                    Ordering::Equal => println!("Correct"),
                }

                text.0.clear();
            }
            Key::Backspace => {
                text.0.pop();
                println!("{}", text.0);
            }
            Key::Character(character) => {
                if character.parse::<i32>().is_ok() {
                    text.0.push_str(character);
                    println!("{}", text.0);
                }
            }
            _ => continue,
        }
    }
}

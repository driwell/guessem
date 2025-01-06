use std::cmp::Ordering;

use bevy::{
    input::keyboard::{Key, KeyboardInput},
    prelude::*,
    scene::ron::de::Position,
    sprite::Anchor,
};

use rand::Rng;

#[derive(Component)]
pub struct Number(i32);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Computer;

#[derive(Component)]
pub struct Prompt;

#[derive(Component)]
pub struct Message;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let random_number = rand::thread_rng().gen_range(1..=100);
    commands.spawn((Computer, Number(random_number)));

    commands.spawn((
        Prompt,
        Text2d::new(""),
        TextFont {
            font_size: 50.0,
            ..default()
        },
    ));

    commands.spawn((
        Message,
        Text2d::new(""),
        TextFont {
            font_size: 50.0,
            ..default()
        },
        Transform::from_xyz(0 as f32 * 0.0, -300.0, 0.0),
    ));

    commands.spawn((Player, Number(0)));
}

// TODO: Separate this into multiple stuff
pub fn keyboard_input_system(
    mut events: EventReader<KeyboardInput>,
    mut text: Query<&mut Text2d, With<Prompt>>,
    mut message: Query<&mut Text2d, (With<Message>, Without<Prompt>)>,
    mut guess: Query<&mut Number, With<Player>>,
    mut number: Query<&mut Number, (With<Computer>, Without<Player>)>,
) {
    let mut text = text.single_mut();
    let mut message = message.single_mut();
    let mut guess = guess.single_mut();
    let mut number = number.single_mut();

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
                    Ordering::Less => message.0 = "Higher".to_string(),
                    Ordering::Greater => message.0 = "Lower".to_string(),
                    Ordering::Equal => {
                        message.0 = "Correct".to_string();
                        number.0 = rand::thread_rng().gen_range(1..=100);
                    }
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

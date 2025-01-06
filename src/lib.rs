use std::cmp::Ordering;

use rand::Rng;

pub fn play() {
    let number = generate_number(1, 100);
    let guess = 42;
    println!("number: {number}, guess: {guess}");
    println!("{}", get_result(&guess, &number));
}

fn generate_number(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

fn get_result(guess: &i32, number: &i32) -> String {
    match guess.cmp(number) {
        Ordering::Greater => "Higher".to_string(),
        Ordering::Less => "Lower".to_string(),
        Ordering::Equal => "Correct".to_string(),
    }
}

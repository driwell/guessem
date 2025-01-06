use rand::Rng;

pub fn play() {
    let number = generate_number(1, 100);
    println!("generated number: {number}");
}

fn generate_number(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}

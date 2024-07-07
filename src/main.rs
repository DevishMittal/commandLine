use std::io;

fn main() {
    println!("Enter your weight on earth (Kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    print!("Your weight on Mars: {}", calc(weight));
}
fn calc(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
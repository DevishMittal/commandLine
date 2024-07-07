use std::io;

fn main() {
    println!("enter your weight on earth: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
    print!("Entered weight {}", calc(weight));
}
fn calc(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
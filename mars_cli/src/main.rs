use std::io;

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight/9.81) * 3.711
}

fn main() {
    println!("Enter your weight (Kg): ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);

    println!("\nWeight on Earth {}kg\nWeight on Mars {}kg ", weight, mars_weight);
}

use std::fs;

fn calculate_fuel(mass_str: &str) -> u32 {
    match mass_str.trim().parse::<u32>() {
        Ok(mass) => (mass / 3) - 2,
        Err(_) => 0,
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    println!("{:?}", contents.lines().map(calculate_fuel).sum::<u32>());
}

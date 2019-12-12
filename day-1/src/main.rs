use std::fs;

fn calculate_fuel(mass: i32) -> i32 {
    let fuel = (mass / 3) - 2;

    if fuel < 1 {
        0
    } else {
        fuel + calculate_fuel(fuel)
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    println!(
        "Part 1: {:?}",
        contents
            .lines()
            .map(|n| n.trim().parse::<i32>().unwrap_or(0))
            .map(|mass| (mass / 3) - 2)
            .sum::<i32>()
    );
    println!(
        "Part 2: {:?}",
        contents
            .lines()
            .map(|n| n.trim().parse::<i32>().unwrap_or(0))
            .map(calculate_fuel)
            .sum::<i32>()
    )
}

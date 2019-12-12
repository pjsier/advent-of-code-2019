use std::fs;

fn get_delta(step: &str) -> [i32; 2] {
    let dir = step.chars().next().unwrap();
    let amount = step
        .chars()
        .skip(1)
        .collect::<String>()
        .parse::<i32>()
        .unwrap_or(0);
    match dir {
        'U' => [0, -amount],
        'D' => [0, amount],
        'L' => [-amount, 0],
        'R' => [amount, 0],
        _ => [0, 0],
    }
}

fn step_range(a: i32, b: i32) -> Vec<i32> {
    if a < b {
        (a..=b).skip(1).collect()
    } else {
        (b..a).rev().collect()
    }
}

fn wire_coordinates(coords: &str) -> Vec<[i32; 2]> {
    let steps: Vec<[i32; 2]> = coords.split(',').map(get_delta).collect();
    let mut coordinates: Vec<[i32; 2]> = vec![[0, 0]];
    for step in steps {
        let last_coord = &coordinates.last().unwrap().clone();
        if step[0] != 0 {
            for x in step_range(last_coord[0], last_coord[0] + step[0]) {
                coordinates.push([x, last_coord[1]])
            }
        } else if step[1] != 0 {
            for y in step_range(last_coord[1], last_coord[1] + step[1]) {
                coordinates.push([last_coord[0], y])
            }
        }
    }
    coordinates
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let wire1 = wire_coordinates(lines[0]);
    let wire2 = wire_coordinates(lines[1]);

    for (steps1, wire1_coord) in wire1.iter().skip(1).enumerate() {
        for (steps2, wire2_coord) in wire2.iter().skip(1).enumerate() {
            if wire1_coord[0] == wire2_coord[0] && wire1_coord[1] == wire2_coord[1] {
                println!(
                    "Intersect at {:?} with total steps: {:?}",
                    wire1_coord,
                    steps1 + steps2 + 2
                );
            }
        }
    }
}

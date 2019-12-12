use std::convert::TryInto;
use std::fs;

fn process_intcodes(intcode_vec: &mut Vec<u32>) -> u32 {
    let mut n = 0;
    while n < intcode_vec.len() {
        let opcode = intcode_vec[n];
        match opcode {
            1 => {
                let intcode_arg1: usize = intcode_vec[n + 1].try_into().unwrap_or(0);
                let intcode_arg2: usize = intcode_vec[n + 2].try_into().unwrap_or(0);
                let intcode_pos: usize = intcode_vec[n + 3].try_into().unwrap_or(0);
                if vec![intcode_arg1, intcode_arg2, intcode_pos]
                    .iter()
                    .any(|n| n >= &intcode_vec.len())
                {
                    return 0;
                }
                intcode_vec[intcode_pos] = intcode_vec[intcode_arg1] + intcode_vec[intcode_arg2];
                n += 4
            }
            2 => {
                let intcode_arg1: usize = intcode_vec[n + 1].try_into().unwrap_or(0);
                let intcode_arg2: usize = intcode_vec[n + 2].try_into().unwrap_or(0);
                let intcode_pos: usize = intcode_vec[n + 3].try_into().unwrap_or(0);
                if vec![intcode_arg1, intcode_arg2, intcode_pos]
                    .iter()
                    .any(|n| n >= &intcode_vec.len())
                {
                    return 0;
                }
                intcode_vec[intcode_pos] = intcode_vec[intcode_arg1] * intcode_vec[intcode_arg2];
                n += 4
            }
            99 => break,
            _ => n += 1,
        }
    }
    intcode_vec[0]
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let intcode_vec: Vec<u32> = contents
        .split(',')
        .map(|s| s.parse::<u32>().unwrap_or(0))
        .collect();

    let mut part1_vec = intcode_vec.clone();
    // Custom replacement
    part1_vec[1] = 12;
    part1_vec[2] = 2;

    println!("Part 1 answer is: {:?}", process_intcodes(&mut part1_vec));

    let codes: Vec<[u32; 2]> = (0..100)
        .flat_map(|n| (0..100).map(move |i| [n, i]))
        .collect();
    for code in codes {
        let mut part2_vec = intcode_vec.clone();
        part2_vec[1] = code[0];
        part2_vec[2] = code[1];
        if process_intcodes(&mut part2_vec) == 19690720 {
            println!("Part two values are: {:?} {:?}", code[0], code[1]);
            println!("Part two answer is: {:?}", 100 * code[0] + code[1]);
            break;
        }
    }
}

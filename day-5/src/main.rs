use std::fs;
use std::io;

fn parse_instruction(instruction: isize) -> (isize, isize, isize, isize) {
    let opcode = instruction % 100;
    let mode_1 = instruction % 1000 / 100;
    let mode_2 = instruction % 10000 / 1000;
    let mode_3 = instruction % 100000 / 10000;
    (opcode, mode_1, mode_2, mode_3)
}

fn get_arg_val(intcode_slice: &mut [isize], arg: isize, mode: isize) -> isize {
    match mode {
        0 => {
            if (arg as usize) < intcode_slice.len() {
                return intcode_slice[arg as usize];
            }
            0
        }
        1 => arg,
        _ => 0,
    }
}

fn handle_instruction(
    intcode_slice: &mut [isize],
    index: usize,
    opcode: isize,
    mode_1: isize,
    mode_2: isize,
    mode_3: isize,
) -> (bool, isize, isize, usize) {
    let arg1 = intcode_slice[index + 1];
    let arg2 = intcode_slice[index + 2];
    let arg3 = intcode_slice[index + 3];
    let arg1_val = get_arg_val(intcode_slice, arg1, mode_1);
    let arg2_val = get_arg_val(intcode_slice, arg2, mode_2);

    match opcode {
        1 => (true, arg3, arg1_val + arg2_val, index + 4),
        2 => (true, arg3, arg1_val * arg2_val, index + 4),
        3 => {
            // Get user input
            let mut input = String::new();
            println!("Please enter input:");
            io::stdin()
                .read_line(&mut input)
                .expect("error: unable to read user input");
            (
                true,
                arg1,
                input.trim().parse::<isize>().unwrap_or(0),
                index + 2,
            )
        }
        4 => {
            println!("{:?}", arg1_val);
            (false, 0, 0, index + 2)
        }
        5 => {
            let place = if arg1_val != 0 {
                arg2_val as usize
            } else {
                index + 3
            };
            (false, 0, 0, place)
        }
        6 => {
            let place = if arg1_val == 0 {
                arg2_val as usize
            } else {
                index + 3
            };
            (false, 0, 0, place)
        }
        7 => (true, arg3, (arg1_val < arg2_val) as isize, index + 4),
        8 => (true, arg3, (arg1_val == arg2_val) as isize, index + 4),
        _ => (false, 0, 0, index + 1),
    }
}

fn process_intcodes(intcode_slice: &mut [isize]) -> isize {
    let mut n = 0;
    while n < intcode_slice.len() {
        let (opcode, m_1, m_2, m_3) = parse_instruction(intcode_slice[n]);
        match opcode {
            (1..=8) => {
                let (is_op, idx, val, place) =
                    handle_instruction(intcode_slice, n, opcode, m_1, m_2, m_3);
                if is_op {
                    intcode_slice[idx as usize] = val;
                }
                n = place;
            }
            99 => break,
            n => {
                println!("Alt code: {}", n);
                break;
            }
        }
    }
    intcode_slice[0]
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut intcode_slice: Vec<isize> = contents
        .split(',')
        .map(|s| s.parse::<isize>().unwrap_or(0))
        .collect();
    process_intcodes(intcode_slice.as_mut_slice());
}

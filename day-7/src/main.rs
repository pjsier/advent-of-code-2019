use io::BufReader;
use permutohedron::Heap;
use std::fs;
use std::io;
use std::io::BufRead;

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
    input_reader: &mut BufReader<&[u8]>,
    output_values: &mut Vec<isize>,
    index: usize,
    opcode: isize,
    mode_1: isize,
    mode_2: isize,
    mode_3: isize,
) -> (bool, isize, isize, usize) {
    let arg1 = intcode_slice[index + 1];
    let arg2 = intcode_slice[index + 2];
    let arg3 = if index + 3 < intcode_slice.len() {
        intcode_slice[index + 3]
    } else {
        0
    };
    let arg1_val = get_arg_val(intcode_slice, arg1, mode_1);
    let arg2_val = get_arg_val(intcode_slice, arg2, mode_2);

    match opcode {
        1 => (true, arg3, arg1_val + arg2_val, index + 4),
        2 => (true, arg3, arg1_val * arg2_val, index + 4),
        3 => {
            let mut input = String::new();
            input_reader
                .read_line(&mut input)
                .expect("error: unable to read input");
            let input_str = input.trim();
            let input_val = if input_str != "" {
                input_str.parse::<isize>().unwrap_or(0)
            } else {
                output_values.pop().unwrap_or(0)
            };
            (true, arg1, input_val, index + 2)
        }
        4 => {
            output_values.push(arg1_val);
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

fn process_intcodes(
    intcode_slice: &mut [isize],
    output_values: &mut Vec<isize>,
    inputs: Vec<isize>,
    start_idx: usize,
    part_2: bool,
) -> (isize, isize, usize) {
    let mut n = start_idx.clone();
    let input_str = inputs
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    let mut input_reader: BufReader<_> = BufReader::new(input_str.as_bytes());

    while n < intcode_slice.len() {
        let (opcode, m_1, m_2, m_3) = parse_instruction(intcode_slice[n]);
        match opcode {
            (1..=8) => {
                let (is_op, idx, val, place) = handle_instruction(
                    intcode_slice,
                    &mut input_reader,
                    output_values,
                    n,
                    opcode,
                    m_1,
                    m_2,
                    m_3,
                );
                if is_op {
                    intcode_slice[idx as usize] = val;
                } else if opcode == 4 {
                    return (intcode_slice[0], *output_values.last().unwrap_or(&0), place);
                }
                n = place;
            }
            99 => {
                return (-99, *output_values.last().unwrap_or(&0), n);
            }
            n => break,
        }
    }
    (intcode_slice[0], *output_values.last().unwrap_or(&0), n)
}

fn process_combination(intcode_slice: &mut [isize], inputs: Vec<isize>) -> isize {
    let mut last_output = 0;
    for input in inputs {
        let mut output_values: Vec<isize> = Vec::new();
        let output = process_intcodes(
            intcode_slice,
            &mut output_values,
            vec![input, last_output],
            0,
            false,
        );
        last_output = output.1;
    }
    last_output
}

fn process_combination_2(intcode_slice: Vec<isize>, inputs: Vec<isize>) -> isize {
    let mut last_output = 0;
    let mut idx = 0;
    let mut amp_vecs: Vec<Vec<isize>> = (0..5).map(|_| intcode_slice.clone()).collect();
    let mut amp_slices: Vec<&mut [isize]> = amp_vecs.iter_mut().map(|v| v.as_mut_slice()).collect();
    let mut output_values: Vec<isize> = Vec::new();
    let mut amp_idxs: &mut [usize] = &mut [0, 0, 0, 0, 0];

    loop {
        let input_vec = if idx < inputs.len() {
            vec![inputs[idx], last_output]
        } else {
            vec![]
        };
        let (output_num, output_val, amp_idx) = process_intcodes(
            &mut amp_slices[idx % 5],
            &mut output_values,
            input_vec,
            amp_idxs[idx % 5],
            true,
        );
        if output_num == -99 {
            break;
        }
        amp_idxs[idx % 5] = amp_idx;
        last_output = output_val;
        idx += 1;
    }
    last_output
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut intcode_vec: Vec<isize> = contents
        .split(',')
        .map(|s| s.parse::<isize>().unwrap_or(0))
        .collect();

    let mut input_vec = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut input_vec);

    let output = heap
        .map(|c| process_combination(intcode_vec.as_mut_slice(), c))
        .max();
    println!("Part 1 answer: {:?}", output.unwrap());

    let mut part_2_input_vec = vec![5, 6, 7, 8, 9];
    let part_2_heap = Heap::new(&mut part_2_input_vec);
    let part_2_output = part_2_heap
        .map(|c| process_combination_2(intcode_vec.clone(), c))
        .max();
    println!("Part 2 answer is: {:?}", part_2_output.unwrap());
}

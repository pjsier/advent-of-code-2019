use permutohedron::Heap;
use std::fmt::Debug;
use std::fs;
use std::io;

#[derive(Debug)]
enum IntcodeOperation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equal,
    Halt,
}

impl IntcodeOperation {
    pub fn from_num(num: isize) -> IntcodeOperation {
        match num {
            1 => IntcodeOperation::Add,
            2 => IntcodeOperation::Multiply,
            3 => IntcodeOperation::Input,
            4 => IntcodeOperation::Output,
            5 => IntcodeOperation::JumpIfTrue,
            6 => IntcodeOperation::JumpIfFalse,
            7 => IntcodeOperation::LessThan,
            8 => IntcodeOperation::Equal,
            99 => IntcodeOperation::Halt,
            _ => IntcodeOperation::Halt, // TODO: Not sure if this is valid?
        }
    }
}

enum IntcodeMode {
    Position,
    Immediate,
}

impl IntcodeMode {
    pub fn from_num(num: isize) -> IntcodeMode {
        match num {
            0 => IntcodeMode::Position,
            1 => IntcodeMode::Immediate,
            n => {
                panic!("Invalid number {} supplied", n);
            }
        }
    }
}

struct IntcodeInstruction {
    pub operation: IntcodeOperation,
    pub mode_1: IntcodeMode,
    pub mode_2: IntcodeMode,
    pub mode_3: IntcodeMode,
}

impl IntcodeInstruction {
    pub fn new(
        operation: IntcodeOperation,
        mode_1: IntcodeMode,
        mode_2: IntcodeMode,
        mode_3: IntcodeMode,
    ) -> IntcodeInstruction {
        IntcodeInstruction {
            operation,
            mode_1,
            mode_2,
            mode_3,
        }
    }

    pub fn from_num(num: isize) -> IntcodeInstruction {
        // println!("{:?}", num);
        IntcodeInstruction::new(
            IntcodeOperation::from_num(num % 100),
            IntcodeMode::from_num(num % 1000 / 100),
            IntcodeMode::from_num(num % 10000 / 1000),
            IntcodeMode::from_num(num % 100000 / 10000),
        )
    }
}

struct IntcodeComputer<'a> {
    intcodes: &'a mut [isize],
    // TODO: Might need to refactor
    pub inputs: Vec<isize>,
    pub outputs: Vec<isize>,
    pub index: usize,
    pause_on_output: bool,
}

impl<'a> IntcodeComputer<'a> {
    pub fn new(
        intcodes: &'a mut [isize],
        inputs: Vec<isize>,
        pause_on_output: bool,
    ) -> IntcodeComputer<'a> {
        IntcodeComputer {
            intcodes,
            inputs,
            outputs: Vec::new(),
            index: 0,
            pause_on_output,
        }
    }

    pub fn run(&mut self) -> IntcodeOperation {
        loop {
            let instruction = IntcodeInstruction::from_num(self.intcodes[self.index]);
            match instruction.operation {
                IntcodeOperation::Output => {
                    self.handle_instruction(instruction);
                    if self.pause_on_output {
                        return IntcodeOperation::Output;
                    }
                }
                IntcodeOperation::Halt => {
                    return IntcodeOperation::Halt;
                }
                _ => {
                    self.handle_instruction(instruction);
                }
            }
        }
    }

    fn get_arg_value(&mut self, arg: isize, mode: IntcodeMode) -> isize {
        match mode {
            IntcodeMode::Position => {
                if (arg as usize) < self.intcodes.len() {
                    return self.intcodes[arg as usize];
                }
                0
            }
            IntcodeMode::Immediate => arg,
        }
    }

    fn handle_instruction(&mut self, instruction: IntcodeInstruction) {
        let arg_1 = *self.intcodes.get(self.index + 1).unwrap_or(&0);
        let arg_2 = *self.intcodes.get(self.index + 2).unwrap_or(&0);
        let arg_3 = *self.intcodes.get(self.index + 3).unwrap_or(&0);
        let arg_1_val = self.get_arg_value(arg_1, instruction.mode_1);
        let arg_2_val = self.get_arg_value(arg_2, instruction.mode_2);
        let arg_3_val = self.get_arg_value(arg_3, instruction.mode_3);

        // println!("Index: {}", self.index);
        // println!("Op: {:?}", instruction.operation);
        // println!("Inputs: {:?}", self.inputs);
        // println!("Outputs: {:?}", self.outputs);
        // println!("Vec: {:?}", self.intcodes);
        match instruction.operation {
            IntcodeOperation::Add => {
                self.intcodes[arg_3 as usize] = arg_1_val + arg_2_val;
                self.index += 4;
            }
            IntcodeOperation::Multiply => {
                self.intcodes[arg_3 as usize] = arg_1_val * arg_2_val;
                self.index += 4;
            }
            IntcodeOperation::Input => {
                self.intcodes[arg_1 as usize] = if self.inputs.len() > 0 {
                    *self
                        .inputs
                        .drain(..1)
                        .collect::<Vec<isize>>()
                        .first()
                        .unwrap()
                } else {
                    self.outputs.pop().unwrap_or(0)
                };
                self.index += 2;
            }
            IntcodeOperation::Output => {
                self.outputs.push(arg_1_val);
                self.index += 2;
            }
            IntcodeOperation::JumpIfTrue => {
                self.index = match arg_1_val {
                    0 => self.index + 3,
                    _ => arg_2_val as usize,
                };
            }
            IntcodeOperation::JumpIfFalse => {
                self.index = match arg_1_val {
                    0 => arg_2_val as usize,
                    _ => self.index + 3,
                };
            }
            IntcodeOperation::LessThan => {
                self.intcodes[arg_3 as usize] = (arg_1_val < arg_2_val) as isize;
                self.index += 4;
            }
            IntcodeOperation::Equal => {
                self.intcodes[arg_3 as usize] = (arg_1_val == arg_2_val) as isize;
                self.index += 4;
            }
            IntcodeOperation::Halt => {}
        }
    }
}

fn process_combination(intcode_slice: &mut [isize], inputs: Vec<isize>) -> isize {
    let mut last_output = 0;
    let mut computer = IntcodeComputer::new(intcode_slice, Vec::new(), true);
    for input in inputs {
        let mut input_vec = vec![input, last_output];
        computer.index = 0;
        computer.inputs = input_vec;
        computer.run();
        last_output = *computer.outputs.last().unwrap_or(&0);
    }
    last_output
}

fn process_combination_2(intcode_slice: Vec<isize>, inputs: Vec<isize>) -> isize {
    let mut last_output = 0;
    let mut output_values: Vec<isize> = vec![];
    let mut amp_vecs: Vec<Vec<isize>> = (0..5).map(|_| intcode_slice.clone()).collect();
    let mut amp_slices: Vec<&mut [isize]> = amp_vecs.iter_mut().map(|v| v.as_mut_slice()).collect();
    let mut amp_computers: Vec<IntcodeComputer> = amp_slices
        .iter_mut()
        .map(|s| IntcodeComputer::new(*s, vec![], true))
        .collect();

    let mut idx = 0;
    loop {
        let input_vec = if idx < inputs.len() {
            vec![inputs[idx], last_output]
        } else {
            vec![]
        };

        let mut computer = &mut amp_computers[idx % 5];
        computer.inputs = input_vec;
        computer.outputs = output_values;
        match computer.run() {
            IntcodeOperation::Halt => break,
            _ => {}
        }
        output_values = computer.outputs.clone();
        last_output = *computer.outputs.last().unwrap_or(&0);
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

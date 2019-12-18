use std::clone::Clone;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::marker::Copy;

#[derive(Clone, Copy, Debug)]
enum IntcodeOperation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equal,
    RelativeBaseOffset,
    Halt,
}

impl IntcodeOperation {
    pub fn from_num(num: i128) -> IntcodeOperation {
        match num {
            1 => IntcodeOperation::Add,
            2 => IntcodeOperation::Multiply,
            3 => IntcodeOperation::Input,
            4 => IntcodeOperation::Output,
            5 => IntcodeOperation::JumpIfTrue,
            6 => IntcodeOperation::JumpIfFalse,
            7 => IntcodeOperation::LessThan,
            8 => IntcodeOperation::Equal,
            9 => IntcodeOperation::RelativeBaseOffset,
            99 => IntcodeOperation::Halt,
            _ => IntcodeOperation::Halt, // TODO: Not sure if this is valid?
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum IntcodeMode {
    Position,
    Immediate,
    Relative,
}

impl IntcodeMode {
    pub fn from_num(num: i128) -> IntcodeMode {
        match num {
            0 => IntcodeMode::Position,
            1 => IntcodeMode::Immediate,
            2 => IntcodeMode::Relative,
            n => {
                panic!("Invalid number {} supplied", n);
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
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

    pub fn from_num(num: i128) -> IntcodeInstruction {
        IntcodeInstruction::new(
            IntcodeOperation::from_num(num % 100),
            IntcodeMode::from_num(num % 1000 / 100),
            IntcodeMode::from_num(num % 10000 / 1000),
            IntcodeMode::from_num(num % 100000 / 10000),
        )
    }
}

struct IntcodeComputer {
    pub intcodes: Vec<i128>,
    // TODO: Might need to refactor
    pub inputs: Vec<i128>,
    pub outputs: Vec<i128>,
    pub index: usize,
    pub relative_base: i128,
    pause_on_output: bool,
}

impl IntcodeComputer {
    pub fn new(intcodes: Vec<i128>, inputs: Vec<i128>, pause_on_output: bool) -> IntcodeComputer {
        IntcodeComputer {
            intcodes,
            inputs,
            outputs: Vec::new(),
            index: 0,
            relative_base: 0,
            pause_on_output,
        }
    }

    pub fn run(&mut self) -> IntcodeOperation {
        let mut idx = 0;
        loop {
            let instruction = IntcodeInstruction::from_num(self.intcodes[self.index]);
            idx += 1;
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

    fn get_arg_value(&mut self, arg: i128, mode: IntcodeMode, dest: bool) -> i128 {
        match (mode, dest) {
            (IntcodeMode::Position, true) => arg,
            (IntcodeMode::Position, false) => {
                if arg < 0 {
                    return 0;
                } else if (arg as usize) >= self.intcodes.len() {
                    self.extend_intcodes((arg + 1) as usize);
                }
                self.intcodes[arg as usize]
            }
            (IntcodeMode::Immediate, _) => arg,
            (IntcodeMode::Relative, _) => {
                if (self.relative_base + arg) as usize >= self.intcodes.len() {
                    self.extend_intcodes((self.relative_base + arg + 1) as usize);
                }
                if dest {
                    return self.relative_base + arg;
                } else {
                    return self.intcodes[(self.relative_base + arg) as usize];
                }
            }
        }
    }

    fn handle_instruction(&mut self, instruction: IntcodeInstruction) {
        let arg_1 = *self.intcodes.get(self.index + 1).unwrap_or(&0);
        let arg_2 = *self.intcodes.get(self.index + 2).unwrap_or(&0);
        let arg_3 = *self.intcodes.get(self.index + 3).unwrap_or(&0);
        let arg_1_val = self.get_arg_value(arg_1, instruction.mode_1, false);
        let arg_2_val = self.get_arg_value(arg_2, instruction.mode_2, false);
        let arg_3_val = self.get_arg_value(arg_3, instruction.mode_3, true);

        match instruction.operation {
            IntcodeOperation::Add => {
                self.extend_intcodes((arg_3_val + 1) as usize);
                self.intcodes[arg_3_val as usize] = arg_1_val + arg_2_val;
                self.index += 4;
            }
            IntcodeOperation::Multiply => {
                self.extend_intcodes((arg_3_val + 1) as usize);
                self.intcodes[arg_3_val as usize] = arg_1_val * arg_2_val;
                self.index += 4;
            }
            IntcodeOperation::Input => {
                let arg_1_dest = self.get_arg_value(arg_1, instruction.mode_1, true);
                self.intcodes[arg_1_dest as usize] = if self.inputs.len() > 0 {
                    *self
                        .inputs
                        .drain(..1)
                        .collect::<Vec<i128>>()
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
                self.extend_intcodes((arg_3_val + 1) as usize);
                self.intcodes[arg_3_val as usize] = (arg_1_val < arg_2_val) as i128;
                self.index += 4;
            }
            IntcodeOperation::Equal => {
                self.extend_intcodes((arg_3_val + 1) as usize);
                self.intcodes[arg_3_val as usize] = (arg_1_val == arg_2_val) as i128;
                self.index += 4;
            }
            IntcodeOperation::RelativeBaseOffset => {
                self.relative_base += arg_1_val;
                self.index += 2;
            }
            IntcodeOperation::Halt => {}
        }
    }

    fn extend_intcodes(&mut self, len: usize) {
        if self.intcodes.len() < len {
            let diff = len - self.intcodes.len();
            self.intcodes.append(&mut vec![0; diff]);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut intcode_vec: Vec<i128> = contents
        .split(',')
        .map(|s| s.parse::<i128>().unwrap_or(0))
        .collect();
    let mut computer = IntcodeComputer::new(intcode_vec, vec![1], false);
    computer.run();
    println!("Part 1 answer: {:?}", computer.outputs.last().unwrap_or(&0));

    let mut intcode_vec_2: Vec<i128> = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i128>().unwrap_or(0))
        .collect();
    let mut computer_2 = IntcodeComputer::new(intcode_vec_2, vec![2], false);
    computer_2.run();
    println!(
        "Part 2 answer: {:?}",
        computer_2.outputs.last().unwrap_or(&0)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input() {
        let mut intcode_vec: Vec<i128> = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut computer = IntcodeComputer::new(intcode_vec, vec![], false);
        computer.run();
        assert_eq!(
            computer.outputs,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,]
        );
    }

    #[test]
    fn test_sample_input_2() {
        let mut intcode_vec: Vec<i128> = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let mut computer = IntcodeComputer::new(intcode_vec, vec![], false);
        computer.run();
        let last_output_str = computer.outputs.last().unwrap_or(&0).to_string();
        assert_eq!(last_output_str.chars().collect::<Vec<char>>().len(), 16);
    }

    #[test]
    fn test_sample_input_3() {
        let mut intcode_vec: Vec<i128> = vec![104, 1125899906842624, 99];
        let mut computer = IntcodeComputer::new(intcode_vec, vec![], false);
        computer.run();
        assert_eq!(*computer.outputs.last().unwrap_or(&0), 1125899906842624);
    }
}

use std::collections::VecDeque;

pub fn parse_program(input: &str) -> Vec<i32> {
    input.split(',').filter_map(|v| v.parse().ok()).collect()
}

pub trait Input {
    fn read_input(&mut self) -> Option<i32>;
}

impl<T> Input for T
where 
    T: FnMut() -> i32 
{
    fn read_input(&mut self) -> Option<i32> {
        Some(self())
    }
}

impl Input for VecDeque<i32> {
    fn read_input(&mut self) -> Option<i32> {
        self.pop_front()
    }
}

pub trait Output {
    fn write_output(&mut self, output: i32);
}

impl<T> Output for T
where
    T: FnMut(i32)
{
    fn write_output(&mut self, output: i32) {
        self(output);
    }
}

impl Output for VecDeque<i32> {
    fn write_output(&mut self, output: i32) {
        self.push_back(output);
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    parameters: Vec<Parameter>
}

impl Instruction {
    fn num_values(&self) -> usize {
        1 + self.opcode.num_parameters()
    }
}

pub struct Computer {
    pub memory: Vec<i32>,
    instruction_pointer: usize,
    halted: bool,
}

impl Computer {
    pub fn initialize(program: &[i32]) -> Computer {
        Computer {
            memory: program.to_vec(),
            instruction_pointer: 0,
            halted: false,
        }
    }

    pub fn run(&mut self) {
        self.run_with_io(&mut || 0, &mut |_| {});
    }

    pub fn run_with_io<I: Input, O: Output>(&mut self, input: &mut I, output: &mut O) {
        while !self.halted {
            let success = self.step(input, output);
            if !success {
                break;
            }
        }
    }

    fn step<I: Input, O: Output>(&mut self, input: &mut I, output: &mut O) -> bool {
        if self.halted {
            return false;
        }

        let instruction = self.read_instruction();
        self.execute_instruction(&instruction, input, output)
    }

    fn read_instruction(&self) -> Instruction {
        let value = &self.memory[self.instruction_pointer];
        let opcode = Opcode::parse(value % 100);

        let mut parameters = vec![];
        let mut modes = value / 100;
        for i in 1..=opcode.num_parameters() {
            let mode = ParameterMode::of(modes % 10);
            let parameter = Parameter {
                mode: mode,
                value: self.memory[self.instruction_pointer + i],
            };
            parameters.push(parameter);
            modes /= 10;
        }

        Instruction { opcode, parameters }
    }

    fn execute_instruction<I: Input, O: Output>(&mut self, instruction: &Instruction, input: &mut I, output: &mut O) -> bool {
        let initial_instruction_pointer = self.instruction_pointer;
        let parameters = &instruction.parameters;
        match instruction.opcode {
            Opcode::Add => {
                let value = self.read(&parameters[0]) + self.read(&parameters[1]);
                self.write(&parameters[2], value);
            },
            Opcode::Mul => {
                let value = self.read(&parameters[0]) * self.read(&parameters[1]);
                self.write(&parameters[2], value);
            },
            Opcode::Input => 
                if let Some(value) = input.read_input() {
                    self.write(&parameters[0], value);
                } else {
                    return false
                },
            Opcode::Output => output.write_output(self.read(&parameters[0])),
            Opcode::JumpIfTrue => {
                if self.read(&parameters[0]) != 0 {
                    self.jump_to(self.read(&parameters[1]));
                }
            },
            Opcode::JumpIfFalse => {
                if self.read(&parameters[0]) == 0 {
                    self.jump_to(self.read(&parameters[1]));
                }
            },
            Opcode::LessThan => {
                let val = self.read(&parameters[0]) < self.read(&parameters[1]);
                self.write(&parameters[2], val as i32);
            },
            Opcode::Equals => {
                let val = self.read(&parameters[0]) == self.read(&parameters[1]);
                self.write(&parameters[2], val as i32);
            },
            Opcode::Halt => self.halted = true,
        }
        if initial_instruction_pointer == self.instruction_pointer {
            self.instruction_pointer += instruction.num_values();
        }
        true
    }

    fn read(&self, parameter: &Parameter) -> i32 {
        match parameter.mode {
            ParameterMode::Position => self.memory[parameter.value as usize],
            ParameterMode::Immediate => parameter.value,
        }
    }

    fn write(&mut self, destination: &Parameter, value: i32) {
        match destination.mode {
            ParameterMode::Position => self.memory[destination.value as usize] = value,
            ParameterMode::Immediate => panic!()
        }
    }

    fn jump_to(&mut self, address: i32) {
        self.instruction_pointer = address as usize;
    }
}

#[derive(Debug)]
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl Opcode {
    fn parse(value: i32) -> Opcode {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => panic!("Invalid opcode! {}", value)
        }
    }

    fn num_parameters(&self) -> usize {
        match self {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::Input => 1,
            Opcode::Output => 1,
            Opcode::JumpIfTrue => 2,
            Opcode::JumpIfFalse => 2,
            Opcode::LessThan => 3,
            Opcode::Equals => 3,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position, Immediate
}

impl ParameterMode {
    fn of(input: i32) -> ParameterMode {
        match input {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Invalid parameter mode!"),
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
}

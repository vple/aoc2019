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
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffset,
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
            9 => Opcode::RelativeBaseOffset,
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
            Opcode::RelativeBaseOffset => 1,
            Opcode::Halt => 0,
        }
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position, Immediate, Relative
}

impl ParameterMode {
    fn of(input: i32) -> ParameterMode {
        match input {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Invalid parameter mode!"),
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
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
    memory: Vec<i32>,
    instruction_pointer: usize,
    halted: bool,
    relative_base: i32,
}

impl Computer {
    pub fn initialize(program: &[i32]) -> Computer {
        Computer {
            memory: program.to_vec(),
            instruction_pointer: 0,
            halted: false,
            relative_base: 0,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
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

    pub fn access(&mut self, address: i32) -> &mut i32 {
        if address < 0 {
            panic!("Must access address >= 0!");
        }
        if address as usize >= self.memory.len() {
            self.memory.resize(self.memory.len() * 2, 0);
        }
        self.memory.get_mut(address as usize).unwrap()
    }

    fn step<I: Input, O: Output>(&mut self, input: &mut I, output: &mut O) -> bool {
        if self.halted {
            return false;
        }

        let instruction = self.read_instruction();
        self.execute_instruction(&instruction, input, output)
    }

    fn read_instruction(&mut self) -> Instruction {
        let value = *self.access(self.instruction_pointer as i32);
        let opcode = Opcode::parse(value % 100);

        let mut parameters = vec![];
        let mut modes = value / 100;
        for i in 1..=opcode.num_parameters() {
            let mode = ParameterMode::of(modes % 10);
            let parameter = Parameter {
                mode: mode,
                value: *self.access((self.instruction_pointer + i) as i32),
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
                    self.instruction_pointer = self.read(&parameters[1]) as usize;
                }
            },
            Opcode::JumpIfFalse => {
                if self.read(&parameters[0]) == 0 {
                    self.instruction_pointer = self.read(&parameters[1]) as usize;
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
            Opcode::RelativeBaseOffset => self.relative_base += self.read(&parameters[0]),
            Opcode::Halt => self.halted = true,
        }
        if initial_instruction_pointer == self.instruction_pointer {
            self.instruction_pointer += instruction.num_values();
        }
        true
    }

    fn read(&mut self, parameter: &Parameter) -> i32 {
        match parameter.mode {
            ParameterMode::Position => *self.access(parameter.value),
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Relative => *self.access(self.relative_base + parameter.value),
        }
    }

    fn write(&mut self, destination: &Parameter, value: i32) {
        match destination.mode {
            ParameterMode::Position => *self.access(destination.value) = value,
            ParameterMode::Immediate => panic!(),
            ParameterMode::Relative => *self.access(self.relative_base + destination.value) = value,
        }
    }

    fn jump_to(&mut self, address: i32) {
        self.instruction_pointer = address as usize;
    }
}

use std::collections::VecDeque;

pub fn parse_program(input: &str) -> Vec<i32> {
    input.split(',').filter_map(|v| v.parse().ok()).collect()
}

pub struct Computer {
    memory: Vec<i32>,
    instruction_pointer: usize,
    halted: bool,
    inputs: VecDeque<i32>,
    outputs: Vec<i32>,
}

trait Input {
    fn read_input(&mut self) -> Option<i32>;
}

trait Output {
    fn write_output(&mut self, output: i32);
}

impl Computer {
    pub fn initialize(program: &[i32]) -> Computer {
        Computer {
            memory: program.to_vec(),
            instruction_pointer: 0,
            halted: false,
            inputs: VecDeque::new(),
            outputs: vec![],
        }
    }

    pub fn add_input(&mut self, input: i32) {
        self.inputs.push_back(input);
    }

    fn read_instruction(&self) -> Instruction {
        Instruction::parse(&self.memory[self.instruction_pointer..])
    }

    fn execute(&mut self, instruction: &Instruction) {
        let initial_instruction_pointer = self.instruction_pointer;
        let parameters = &instruction.parameters;
        match instruction.opcode {
            Opcode::Add => {
                let val = self.read(&parameters[0]) + self.read(&parameters[1]);
                self.write(&parameters[2], val);
            },
            Opcode::Mul => {
                let val = self.read(&parameters[0]) * self.read(&parameters[1]);
                self.write(&parameters[2], val);
            },
            Opcode::Input => {
                let input = self.inputs.pop_front().expect("No input provided!");
                self.write(&parameters[0], input);
            },
            Opcode::Output => {
                let output = self.read(&parameters[0]);
                self.outputs.push(output);
            },
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
            Opcode::Halt => {
                self.halted = true;
            },
        }
        if initial_instruction_pointer == self.instruction_pointer {
            self.instruction_pointer += instruction.num_values();
        }
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

    pub fn run(mut self) -> (Vec<i32>, Vec<i32>) {
        while !self.halted {
            let instruction = self.read_instruction();
            self.execute(&instruction);
        }
        (self.outputs, self.memory)
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
            _ => panic!()
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
struct Instruction {
    opcode: Opcode,
    parameters: Vec<Parameter>
}

impl Instruction {
    fn parse(memory: &[i32]) -> Instruction {
        let first_value = memory[0];
        let opcode = Opcode::parse(first_value % 100);
        let num_parameters = opcode.num_parameters();
        let parameters = Instruction::parse_parameters(num_parameters, first_value/100, &memory[1..1+num_parameters]);
        Instruction { opcode, parameters }
    }

    fn parse_parameters(n: usize, modes: i32, values: &[i32]) -> Vec<Parameter> {
        if n == 0 {
            return vec![];
        }
        let mode = ParameterMode::parse(modes % 10);
        let parameter = Parameter {
            mode: mode,
            value: values[0],
        };
        let mut result = vec![parameter];
        result.extend(Instruction::parse_parameters(n-1, modes/10, &values[1..]));
        result
    }

    fn num_values(&self) -> usize {
        1 + self.opcode.num_parameters()
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position, Immediate
}

impl ParameterMode {
    fn parse(input: i32) -> ParameterMode {
        match input {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
}

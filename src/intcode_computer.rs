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
        let raw_values: Vec<i32> = instruction.parameters.iter().map(|p| p.value).collect();
        let mode_values: Vec<i32> = instruction.parameters.iter().map(|p| p.mode_value(&self.memory)).collect();
        let initial_instruction_pointer = self.instruction_pointer;
        match instruction.opcode {
            Opcode::Add => {
                self.write(raw_values[2], mode_values[..2].iter().sum());
            },
            Opcode::Mul => {
                self.write(raw_values[2], mode_values[..2].iter().product());
            },
            Opcode::Input => {
                let input = self.inputs.pop_front().expect("No input provided!");
                self.write(raw_values[0], input);
            },
            Opcode::Output => {
                let output = instruction.parameters[0].mode_value(&self.memory);
                self.outputs.push(output);
            },
            Opcode::JumpIfTrue => {
                if mode_values[0] != 0 {
                    self.jump_to(mode_values[1]);
                }
            },
            Opcode::JumpIfFalse => {
                if mode_values[0] == 0 {
                    self.jump_to(mode_values[1]);
                }
            },
            Opcode::LessThan => {
                self.write_bool(raw_values[2], mode_values[0] < mode_values[1]);
            },
            Opcode::Equals => {
                self.write_bool(raw_values[2], mode_values[0] == mode_values[1]);
            },
            Opcode::Halt => {
                self.halted = true;
            },
        }
        if initial_instruction_pointer == self.instruction_pointer {
            self.instruction_pointer += instruction.num_values();
        }
    }

    fn write(&mut self, address: i32, value: i32) {
        self.memory[address as usize] = value;
    }

    fn write_bool(&mut self, address: i32, value: bool) {
        self.write(address, value as i32);
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
    POSITION, IMMEDIATE
}

impl ParameterMode {
    fn parse(input: i32) -> ParameterMode {
        match input {
            0 => ParameterMode::POSITION,
            1 => ParameterMode::IMMEDIATE,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    value: i32,
}

impl Parameter {
    fn mode_value(&self, memory: &[i32]) -> i32 {
        match self.mode {
            ParameterMode::IMMEDIATE => self.value,
            ParameterMode::POSITION => memory[self.value as usize],
        }
    }
}

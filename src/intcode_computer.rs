pub struct Computer {
    memory: Vec<i32>,
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

    fn read_instruction(&self) -> Instruction {
        Instruction::parse(&self.memory[self.instruction_pointer..])
    }

    fn execute(&mut self, instruction: &Instruction) {
        let destination_address = || instruction.destination().value as usize;
        match instruction.opcode {
            Opcode::ADD => {
                let sum = instruction.parameters[..2].iter().map(|p| p.mode_value(&self.memory)).sum();
                self.memory[destination_address()] = sum;
            },
            Opcode::MUL => {
                let product = instruction.parameters[..2].iter().map(|p| p.mode_value(&self.memory)).product();
                self.memory[destination_address()] = product;
            },
            Opcode::HALT => {
                self.halted = true;
            },
        }
        self.instruction_pointer += instruction.num_values();
    }

    pub fn run(mut self) -> i32 {
        while !self.halted {
            let instruction = self.read_instruction();
            self.execute(&instruction);
        }
        self.memory[0]
    }
}

enum Opcode {
    ADD,
    MUL,
    HALT
}

impl Opcode {
    fn parse(value: i32) -> Opcode {
        match value {
            1 => Opcode::ADD,
            2 => Opcode::MUL,
            99 => Opcode::HALT,
            _ => panic!()
        }
    }

    fn num_parameters(&self) -> usize {
        match self {
            Opcode::ADD => 3,
            Opcode::MUL => 3,
            Opcode::HALT => 0,
        }
    }
}

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

    fn destination(&self) -> &Parameter {
        self.parameters.last().expect("No destination parameter!")
    }
}

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
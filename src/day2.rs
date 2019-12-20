const TARGET: usize = 19690720;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<usize> {
    input.split(',').filter_map(|p| p.parse().ok()).collect()
}

#[aoc(day2, part1)]
fn part1(ints: &Vec<usize>) -> usize {
    let mut alarm_program = ints.to_vec();
    alarm_program[1] = 12;
    alarm_program[2] = 2;

    let mut program = Program {
        ints: alarm_program,
        step: 0
    };
    program.run()
}

#[aoc(day2, part2)]
fn part2(program: &Vec<usize>) -> usize {
    let size = program.len();
    for noun in 0..size {
        for verb in 0..size {
            let output = run_inputs(program, noun, verb);
            if output == TARGET {
                return 100 * noun + verb
            }
        }
    }
    panic!()
}

fn run_inputs(program: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut runnable = program.to_vec();
    runnable[1] = noun;
    runnable[2] = verb;

    let mut runnable = Program {
        ints: runnable,
        step: 0,
    };
    runnable.run()
}

#[derive(Debug)]
enum OpCode {
    ADD(usize, usize, usize), 
    MUL(usize, usize, usize), 
    HALT,
}

impl OpCode {
    fn parse(ints: &[usize]) -> OpCode {
        match ints[0] {
            1 => OpCode::ADD(ints[1], ints[2], ints[3]),
            2 => OpCode::MUL(ints[1], ints[2], ints[3]),
            99 => OpCode::HALT,
            _ => panic!()
        }
    }

    fn apply(&self, ints: &mut Vec<usize>) {
        match self {
            OpCode::ADD(p1, p2, p3) => ints[*p3] = ints[*p1] + ints[*p2],
            OpCode::MUL(p1, p2, p3) => ints[*p3] = ints[*p1] * ints[*p2],
            OpCode::HALT => (),
        }
    }
}

struct Program {
    ints: Vec<usize>,
    step: usize
}

impl Default for Program {
    fn default() -> Program {
        Program {
            ints: vec![99],
            step: 0
        }
    }
}

impl Program {
    fn opcode(&self) -> OpCode {
        let index = self.step * 4;
        OpCode::parse(&self.ints[index..index+4])
    }

    fn step(&mut self) {
        self.opcode().apply(&mut self.ints);
        self.step += 1;
    }

    fn run(&mut self) -> usize {
        loop {
            if let OpCode::HALT = self.opcode() {
                break self.ints[0]
            } else {
                self.step();
            }
        }
    }
}
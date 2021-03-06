use crate::intcode_computer::{self, Computer};

const TARGET: i64 = 19690720;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<i64> {
    intcode_computer::parse_program(input)
}

#[aoc(day2, part1)]
fn part1(ints: &[i64]) -> i64 {
    let mut alarm_program = ints.to_vec();
    alarm_program[1] = 12;
    alarm_program[2] = 2;

    let mut computer = Computer::initialize(&alarm_program);
    computer.run();
    *computer.access(0)
}

#[aoc(day2, part2)]
fn part2(program: &[i64]) -> i64 {
    let size = program.len();
    for noun in 0..size {
        for verb in 0..size {
            let mut program = program.to_vec();
            program[1] = noun as i64;
            program[2] = verb as i64;
            let mut computer = Computer::initialize(&program);
            computer.run();
            if *computer.access(0) == TARGET {
                return (100 * noun + verb) as i64
            }
        }
    }
    panic!()
}

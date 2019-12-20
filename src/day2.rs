use crate::intcode_computer::Computer;

const TARGET: i32 = 19690720;

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<i32> {
    input.split(',').filter_map(|p| p.parse().ok()).collect()
}

#[aoc(day2, part1)]
fn part1(ints: &[i32]) -> i32 {
    let mut alarm_program = ints.to_vec();
    alarm_program[1] = 12;
    alarm_program[2] = 2;

    let computer = Computer::initialize(&alarm_program);
    computer.run()
}

#[aoc(day2, part2)]
fn part2(program: &[i32]) -> i32 {
    let size = program.len();
    for noun in 0..size {
        for verb in 0..size {
            let mut program = program.to_vec();
            program[1] = noun as i32;
            program[2] = verb as i32;
            let output = Computer::initialize(&program).run();
            if output == TARGET {
                return (100 * noun + verb) as i32
            }
        }
    }
    panic!()
}

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<i32> {
    intcode_computer::parse_program(input)
}

#[aoc(day5, part1)]
fn part1(program: &[i32]) -> i32 {
    let mut computer = Computer::initialize(program);
    computer.add_input(1);
    let (outputs, _) = computer.run();
    *outputs.last().expect("No diagnostic!")
}

#[aoc(day5, part2)]
fn part2(program: &[i32]) -> i32 {
    let mut computer = Computer::initialize(program);
    computer.add_input(5);
    let (outputs, _) = computer.run();
    *outputs.last().expect("No diagnostic!")
}
use std::collections::VecDeque;

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day5)]
fn parse(input: &str) -> Vec<i32> {
    intcode_computer::parse_program(input)
}

#[aoc(day5, part1)]
fn part1(program: &[i32]) -> i32 {
    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    input.push_back(1);
    computer.run_with_io(&mut input, &mut output);
    output.pop_back().expect("No diagnostic!")
}

#[aoc(day5, part2)]
fn part2(program: &[i32]) -> i32 {
    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    input.push_back(5);
    computer.run_with_io(&mut input, &mut output);
    output.pop_back().expect("No diagnostic!")
}
use std::collections::VecDeque;

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<i64> {
    intcode_computer::parse_program(input)
}

#[aoc(day9, part1)]
fn part1(program: &[i64]) -> i64 {
    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    input.push_back(1);
    computer.run_with_io(&mut input, &mut output);
    output.pop_back().expect("No diagnostic!")
}

#[aoc(day9, part2)]
fn part2(program: &[i64]) -> i64 {
    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    input.push_back(2);
    computer.run_with_io(&mut input, &mut output);
    output.pop_back().expect("No diagnostic!")
}
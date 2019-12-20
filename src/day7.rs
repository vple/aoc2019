use itertools::Itertools;

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<i32> {
    intcode_computer::parse_program(input)
}

#[aoc(day7, part1)]
fn part1(program: &[i32]) -> i32 {
    (0..5)
        .permutations(5)
        .map(|p| {
            p.into_iter().map(|v| {
                let mut computer = Computer::initialize(program);
                computer.add_input(v);
                computer
            })
        })
        .map(|computers| {
            computers.into_iter().fold(0, |signal, mut c| {
                c.add_input(signal);
                let (outputs, _) = c.run();
                outputs[0]
            })
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
fn part2(program: &[i32]) -> i32 {
    let mut computer = Computer::initialize(program);
    computer.add_input(5);
    let (outputs, _) = computer.run();
    *outputs.last().expect("No diagnostic!")
}
use std::collections::VecDeque;

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<i64> {
    intcode_computer::parse_program(input)
}

#[aoc(day13, part1)]
fn part1(program: &[i64]) -> i64 {
    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    computer.run_with_io(&mut input, &mut output);
    println!("{:?}", output);
    output.iter().fold(
        (0, 0), 
        |acc, tile_id| {
            let triplet_index = (acc.0 + 1) % 3;
            if triplet_index == 0 && *tile_id == 2 {
                (triplet_index, acc.1 + 1)
            } else {
                (triplet_index, acc.1)
            }
        }
    ).1
}
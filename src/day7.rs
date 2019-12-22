use itertools::Itertools;
use std::collections::VecDeque;

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<i32> {
    intcode_computer::parse_program(input)
}

#[aoc(day7, part1)]
fn part1(program: &[i32]) -> i32 {
    (0..5)
        .permutations(5)
        .map(|p| run_series_amplifiers(program, &p))
        .max()
        .unwrap()
}

fn run_series_amplifiers(program: &[i32], phase_settings: &[i32]) -> i32 {
    let mut io: Vec<VecDeque<i32>> = (0..=phase_settings.len()).map(|_| VecDeque::new()).collect();
    let mut amplifiers: Vec<Computer> = (0..phase_settings.len()).map(|_| Computer::initialize(program)).collect();
    for i in 0..phase_settings.len() {
        io[i].push_back(phase_settings[i]);
    }
    io[0].push_back(0);
    for i in 0..phase_settings.len() {
        let mut output = 0;
        amplifiers[i].run_with_io(&mut io[i], &mut |o| output = o);
        io[i+1].push_back(output);
    }
    io[phase_settings.len()].pop_front().expect("No output!")
}

#[aoc(day7, part2)]
fn part2(program: &[i32]) -> i32 {
    (5..=9)
        .permutations(5)
        .map(|p| run_feedback_amplifiers(program, &p))
        .max()
        .unwrap()
}

fn run_feedback_amplifiers(program: &[i32], phase_settings: &[i32]) -> i32 {
    let mut io: Vec<VecDeque<i32>> = (0..phase_settings.len()).map(|_| VecDeque::new()).collect();
    let mut amplifiers: Vec<Computer> = (0..phase_settings.len()).map(|_| Computer::initialize(program)).collect();
    for i in 0..phase_settings.len() {
        io[i].push_back(phase_settings[i]);
    }
    io[0].push_back(0);
    while !amplifiers[phase_settings.len()-1].is_halted() {
        for i in 0..phase_settings.len() {
            let mut output = VecDeque::new();
            amplifiers[i].run_with_io(&mut io[i], &mut output);
            io[(i+1) % phase_settings.len()].append(&mut output);
        }
    }

    io[0].pop_front().expect("No output!")
}
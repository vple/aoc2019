use std::cmp::max;

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<i32> {
    input.lines().filter_map(|l| l.parse().ok()).collect()
}

#[aoc(day1, part1)]
fn part1(masses: &[i32]) -> i32 {
    masses.iter().map(|m| required_fuel(*m)).sum()
}

#[aoc(day1, part2)]
fn part2(masses: &[i32]) -> i32 {
    masses.iter().map(|m| recursive_required_fuel(*m)).sum()
}

fn required_fuel(mass: i32) -> i32 {
    max(mass / 3 - 2, 0)
}

fn recursive_required_fuel(mass: i32) -> i32 {
    let fuel = required_fuel(mass);
    match fuel {
        0 => 0,
        _ => fuel + recursive_required_fuel(fuel),
    }
}
#[aoc_generator(day4)]
fn parse(input: &str) -> (u32, u32) {
    let bounds: Vec<u32> = input.split('-').filter_map(|s| s.parse().ok()).collect();
    (bounds[0], bounds[1])
}

#[aoc(day4, part1)]
fn part1((min, max): &(u32, u32)) -> usize {
    find_passwords_1(*min, *max).len()
}

#[aoc(day4, part2)]
fn part2((min, max): &(u32, u32)) -> usize {
    find_passwords_2(*min, *max).len()
}

fn find_passwords_1(min: u32, max: u32) -> Vec<u32> {
    (min..=max)
        .map(|n| to_digits(n))
        .filter(|c| is_six_digits(c))
        .filter(|c| has_two_adjacent_digits(c))
        .filter(|c| is_non_decreasing(c))
        .map(|d| to_number(&d))
        .collect()
}

fn find_passwords_2(min: u32, max: u32) -> Vec<u32> {
    (min..=max)
        .map(|n| to_digits(n))
        .filter(|c| is_six_digits(c))
        .filter(|c| has_two_adjacent_digits(c))
        .filter(|c| is_non_decreasing(c))
        .filter(|c| has_exact_pair(c))
        .map(|d| to_number(&d))
        .collect()
}

fn is_six_digits(candidate: &Vec<u32>) -> bool {
    candidate.len() == 6
}

fn has_two_adjacent_digits(candidate: &Vec<u32>) -> bool {
    candidate.iter().fold((0, false), |acc, digit| (*digit, acc.1 || *digit == acc.0)).1
}

fn is_non_decreasing(candidate: &Vec<u32>) -> bool {
    candidate.iter().fold((0, true), |acc, digit| (*digit, acc.1 && *digit >= acc.0)).1
}

fn has_exact_pair(candidate: &Vec<u32>) -> bool {
    candidate.iter().any(|c| candidate.iter().filter(|x| *x == c).count() == 2)
}

fn to_digits(number: u32) -> Vec<u32> {
    number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
}

fn to_number(digits: &Vec<u32>) -> u32 {
    digits.iter().map(|d| d.to_string()).collect::<String>().parse().unwrap()
}
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day20)]
fn parse(input: &str) -> i32 {
    let passages: HashSet<Coordinate> = 
        input.lines().enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().filter(|(_, c)| *c == '.').map(move |(x, _)| (x, y)))
            .map(|(x, y)| Coordinate(x, y))
            .collect();

        1
}

#[aoc(day20, part1)]
fn part1(a: &i32) -> i32 {
    1
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Coordinate(usize, usize);

struct Portal(Coordinate, Coordinate);

struct Maze {
    passages: HashSet<Coordinate>,
    start: Coordinate,
    end: Coordinate,
    portals: HashMap<Coordinate, Coordinate>,
}
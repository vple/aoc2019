use std::collections::{HashMap, VecDeque};

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<i64> {
    intcode_computer::parse_program(input)
}

#[aoc(day11, part1)]
fn part1(program: &[i64]) -> usize {
    let mut state = RobotState {
        position: Coordinate(0, 0),
        direction: Direction::Up,
        painted: HashMap::new(),
    };

    run_robot(&mut state, program);
    state.painted.keys().len()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate(i64, i64);

enum Direction {
    Up, Right, Down, Left
}

impl Direction {
    fn rotate(&self, code: i64) -> Direction {
        match code {
            0 => self.rotate_left(),
            1 => self.rotate_right(),
            _ => panic!("Invalid code!"),
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

struct RobotState {
    position: Coordinate,
    direction: Direction,
    painted: HashMap<Coordinate, i64>,
}

impl RobotState {
    fn paint(&mut self, color: i64) {
        self.painted.insert(self.position.clone(), color);
    }

    fn rotate(&mut self, code: i64) {
        self.direction = self.direction.rotate(code);
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.position = Coordinate(self.position.0, self.position.1 - 1),
            Direction::Right => self.position = Coordinate(self.position.0 + 1, self.position.1),
            Direction::Down => self.position = Coordinate(self.position.0, self.position.1 + 1),
            Direction::Left => self.position = Coordinate(self.position.0 - 1, self.position.1),
        }
    }
}

fn run_robot(state: &mut RobotState, program: &[i64]) {
    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    while !computer.is_halted() {
        let color = *state.painted.get(&state.position).or(Some(&0)).unwrap();
        input.push_back(color);
        computer.run_with_io(&mut input, &mut output);
        let color = output.pop_front().unwrap();
        let rotate_code = output.pop_front().unwrap();
        state.paint(color);
        state.rotate(rotate_code);
        state.move_forward();
    }
}

#[aoc(day11, part2)]
fn part2(program: &[i64]) -> String {
    let mut state = RobotState {
        position: Coordinate(0, 0),
        direction: Direction::Up,
        painted: HashMap::new(),
    };
    state.painted.insert(Coordinate(0, 0), 1);

    run_robot(&mut state, program);

    let min_x = state.painted.keys().min_by_key(|k| k.0).unwrap().0;
    let max_x = state.painted.keys().max_by_key(|k| k.0).unwrap().0 + 1;
    let min_y = state.painted.keys().min_by_key(|k| k.1).unwrap().1;
    let max_y = state.painted.keys().max_by_key(|k| k.1).unwrap().1 + 1;

    let mut painting = vec![vec![0; (max_x - min_x) as usize]; (max_y - min_y) as usize];
    for (coordinate, color) in state.painted.iter() {
        let y = (coordinate.1 - min_y) as usize;
        let x = (coordinate.0 - min_x) as usize;
        painting[y][x] = *color;
    }

    let mut result = String::new();

    result.push_str("\n");
    for y in 0..((max_y - min_y) as usize) {
        for x in 0..((max_x - min_x) as usize) {
            if painting[y][x] == 0 {
                result.push_str(" ");
            } else {
                result.push_str("▓");
            }
        }
        result.push_str("\n");
    }
    result
}
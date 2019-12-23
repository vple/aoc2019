use std::cmp::Ordering;
use std::collections::VecDeque;

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<i64> {
    intcode_computer::parse_program(input)
}

#[aoc(day13, part1)]
fn part1(program: &[i64]) -> usize {
    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    computer.run_with_io(&mut input, &mut output);
    let (tiles, _) = read_output(&mut output);
    tiles.iter().filter(|t| t.kind == TileKind::Block).count()
}

#[derive(PartialEq)]
enum TileKind {
    Empty, Wall, Block, Paddle, Ball
}

#[allow(dead_code)]
struct Tile {
    x: i64,
    y: i64,
    kind: TileKind,
}

impl Tile {
    fn parse(triple: &[i64]) -> Tile {
        let kind = match triple[2] {
            0 => TileKind::Empty,
            1 => TileKind::Wall,
            2 => TileKind::Block,
            3 => TileKind::Paddle,
            4 => TileKind::Ball,
            _ => panic!(),
        };

        Tile {
            x: triple[0],
            y: triple[1],
            kind: kind,
        }
    }
}

fn read_output(output: &mut VecDeque<i64>) -> (Vec<Tile>, i64) {
    let mut tiles = vec![];
    let mut score = 0;
    while output.len() >= 3 {
        let triple: Vec<i64> = output.drain(0..3).collect();
        if triple[0] == -1 && triple[1] == 0 {
            score = triple[2];
        } else {
            tiles.push(Tile::parse(&triple));
        }
    }
    (tiles, score)
}

#[aoc(day13, part2)]
fn part2(program: &[i64]) -> i64 {
    let mut program = program.to_vec();
    program[0] = 2;

    beat_game(&program)
}

fn beat_game(program: &[i64]) -> i64 {
    let mut ball_x = 0;
    let mut paddle_x = 0;
    let mut score = 0;

    let mut computer = Computer::initialize(program);
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();

    while !computer.is_halted() {
        computer.run_with_io(&mut input, &mut output);
        let (tiles, latest_score) = read_output(&mut output);
        score = latest_score;

        ball_x = tiles.iter().filter(|t| t.kind == TileKind::Ball).fold(ball_x, |_acc, tile| tile.x);
        paddle_x = tiles.iter().filter(|t| t.kind == TileKind::Paddle).fold(paddle_x, |_acc, tile| tile.x);
        
        let joystick_input = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        input.push_back(joystick_input);
    }

    score
}
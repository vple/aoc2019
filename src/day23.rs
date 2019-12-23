use std::collections::{HashSet, VecDeque};

use crate::intcode_computer::{self, Computer};

#[aoc_generator(day23)]
fn parse(input: &str) -> Vec<i64> {
    intcode_computer::parse_program(input)
}

#[aoc(day23, part1)]
fn part1(program: &[i64]) -> i64 {
    let mut network = Network::initialize(program);
    run_until_nat(&mut network)
}

struct Network {
    inputs: Vec<VecDeque<i64>>,
    outputs: Vec<VecDeque<i64>>,
    computers: Vec<Computer>,
    idle: Vec<bool>,
    nat: Option<(i64, i64)>,
}

impl Network {
    fn initialize(program: &[i64]) -> Network {
        let mut inputs: Vec<VecDeque<i64>> = (0..50).map(|_| VecDeque::new()).collect();
        let mut outputs: Vec<VecDeque<i64>> = (0..50).map(|_| VecDeque::new()).collect();
        let mut computers: Vec<Computer> = (0..50).map(|_| Computer::initialize(program)).collect();
        (0..50).for_each(|i| inputs[i].push_back(i as i64));

        Network {
            inputs, 
            outputs, 
            computers,
            idle: vec![false; 50],
            nat: None,
        }
    }

    fn is_idle(&self) -> bool {
        self.idle.iter().all(|status| *status)
    }

    fn run_computer(&mut self, i: usize) {
        let mut idle = true;
        if self.inputs[i].len() == 0 {
            let mut input = VecDeque::new();
            input.push_back(-1);
            self.computers[i].run_with_io(&mut input, &mut self.outputs[i]);
        } else {
            idle = false;
            self.computers[i].run_with_io(&mut self.inputs[i], &mut self.outputs[i]);
        }

        if self.outputs[i].len() > 0 {
            idle = false;
        }
        while self.outputs[i].len() >= 3 {
            let packet: Vec<i64> = self.outputs[i].drain(0..3).collect();
            let (address, packet) = (packet[0] as usize, (packet[1], packet[2]));
            if address == 255 {
                self.nat = Some(packet);
            } else {
                let recipient = &mut self.inputs[address];
                recipient.push_back(packet.0);
                recipient.push_back(packet.1);
            }
        }
        self.idle[i] = idle;
    }

    fn run_until_idle(&mut self) {
        while !self.is_idle() {
            for i in 0..50 {
                self.run_computer(i);
            }
        }
    }

    fn resume_activity(&mut self) {
        if !self.is_idle() {
            return;
        }
        if let Some((x, y)) = self.nat {
            self.inputs[0].push_back(x);
            self.inputs[0].push_back(y);
            self.idle[0] = false;
        }
    }
}

fn run_until_nat(network: &mut Network) -> i64 {
    loop {
        for i in 0..50 {
            network.run_computer(i);
            if let Some((x, y)) = network.nat {
                return y;
            }
        }
    }
}

#[aoc(day23, part2)]
fn part2(program: &[i64]) -> i64 {
    let mut network = Network::initialize(program);
    run_until_redelivery(&mut network)
}

fn run_until_redelivery(network: &mut Network) -> i64 {
    let mut seen_y = HashSet::new();
    loop {
        network.run_until_idle();
        if let Some((x, y)) = network.nat {
            if seen_y.contains(&y) {
                return y;
            }
            seen_y.insert(y);
        }
        network.resume_activity();
    }
}
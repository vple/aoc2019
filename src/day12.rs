use num::Integer;
use regex::Regex;

#[aoc_generator(day12)]
fn parse(input: &str) -> System {
    let position_regex = Regex::new(r"=(-?\d+).*=(-?\d+).*=(-?\d+)").unwrap();
    let moons = 
        input.lines()
            .map(|l| position_regex.captures(l).unwrap())
            .map(|c| Position { x: c[1].parse().unwrap(), y: c[2].parse().unwrap(), z: c[3].parse().unwrap() })
            .map(|p| Moon { position: p, velocity: Velocity { x: 0, y: 0, z: 0 }})
            .collect();
    System { moons: moons }
}

#[derive(Clone, Debug)]
struct System {
    moons: Vec<Moon>
}

#[derive(Clone, Debug)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

#[derive(Clone, Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Debug)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

#[aoc(day12, part1)]
fn part1(system: &System) -> i64 {
    let mut system = system.clone();
    (0..1000).for_each(|_| system.step());
    system.total_energy()
}

impl System {
    fn step(&mut self) {
        let reference = self.moons.clone();
        self.moons.iter_mut().for_each(|m| m.apply_gravity(&reference));
        self.moons.iter_mut().for_each(|m| m.apply_velocity());
    }

    fn total_energy(&self) -> i64 {
        self.moons.iter().map(|m| m.kinetic_energy() * m.potential_energy()).sum()
    }
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.position = Position {
            x: self.position.x + self.velocity.x,
            y: self.position.y + self.velocity.y,
            z: self.position.z + self.velocity.z,
        }
    }

    fn apply_gravity(&mut self, moons: &[Moon]) {
        let new_velocity = 
            moons.iter().fold(
                (self.velocity.x, self.velocity.y, self.velocity.z), 
                |acc, moon| 
                    (acc.0 + (moon.position.x - self.position.x).signum(), 
                    acc.1 + (moon.position.y - self.position.y).signum(), 
                    acc.2 + (moon.position.z - self.position.z).signum())
            );
        self.velocity = Velocity {
            x: new_velocity.0,
            y: new_velocity.1,
            z: new_velocity.2,
        };
    }

    fn potential_energy(&self) -> i64 {
        return self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
    }

    fn kinetic_energy(&self) -> i64 {
        return self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
    }
}
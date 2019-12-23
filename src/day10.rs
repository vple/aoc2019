use multimap::MultiMap;
use num::Integer;
use std::collections::HashSet;
use std::cmp::Ordering;

#[aoc_generator(day10)]
fn parse(input: &str) -> AsteroidBelt {
    let asteroids =
        input.lines().enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| (x, y, c)))
            .filter_map(|(x, y, c)| {
                if c == '#' {
                    Some(Coordinate { 
                        x: x as i64, 
                        y: y as i64,
                    })
                } else {
                    None
                }
            })
            .collect();
    AsteroidBelt { asteroids }
}

#[derive(Debug, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct AsteroidBelt {
    asteroids: Vec<Coordinate>,
}

#[aoc(day10, part1)]
fn part1(asteroid_belt: &AsteroidBelt) -> usize {
    asteroid_belt.asteroids.iter()
        .map(|base| asteroid_belt.count_detectable(base))
        .max()
        .unwrap()
}

impl AsteroidBelt {
    fn count_detectable(&self, base: &Coordinate) -> usize {
        let mut angles = HashSet::new();
        for a in &self.asteroids {
            if base != a {
                angles.insert(base.angle_to(&a));
            }
        }
        angles.len()
    }

    fn chart(&self, base: &Coordinate) -> MultiMap<Angle, (&Coordinate, i64)> {
        let mut chart = MultiMap::new();
        for a in &self.asteroids {
            if base != a {
                chart.insert(base.angle_to(&a), (a, base.distance_to(&a)));
            }
        }
        chart
    }

    // fn vaporize_order(&self, base: &Coordinate) {
    //     let mut chart = self.chart(base);
    //     // let mut order = vec![];
    //     while chart.len() > 0 {
    //         let mut keys: Vec<&Angle> = chart.keys().collect();
    //         keys.sort_by(|a, b| Angle::compare(a, b));
    //         for key in keys.iter() {
    //             let mut asteroids = chart.get_vec_mut(key).unwrap();
    //             let closest = asteroids.iter().min_by_key(|c| c.1).unwrap();
    //             asteroids.retain(|e| e != closest);
    //         }
    //     }
    // }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Angle {
    dy: i64,
    dx: i64,
}

impl Angle {
    fn compare(a: &Angle, b: &Angle) -> Ordering {
        if a.dy == b.dy && a.dx == b.dx {
            return Ordering::Equal;
        }
        if a.quadrant() < b.quadrant() {
            return Ordering::Less;
        }
        if a.quadrant() > b.quadrant() {
            return Ordering::Greater;
        }

        let (cross_a, cross_b) = (a.dy * b.dx, a.dx * b.dy);
        if cross_a < cross_b {
            return Ordering::Less;
        } else if cross_a > cross_b {
            return Ordering::Greater;
        }

        if a.quadrant() % 2 == 1 {
            if a.dy.abs() > b.dy.abs() {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        } else {
            if a.dx.abs() > b.dx.abs() {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
    }

    fn of(dy: i64, dx: i64) -> Angle {
        if dy == 0 && dx == 0 {
            Angle { dy: 0, dx: 0 }
        } else {
            let gcd = dy.gcd(&dx);
            Angle { dy: dy/gcd, dx: dx/gcd }
        }
    }

    fn quadrant(&self) -> i64 {
        if self.dx == 0 && self.dy == 0 {
            0
        } else if self.dx >= 0 && self.dy < 0 {
            1
        } else if self.dx > 0 && self.dy >= 0 {
            2
        } else if self.dx <= 0 && self.dy > 0 {
            3
        } else if self.dx < 0 && self.dy <= 0 {
            4
        } else {
            unreachable!();
        }
    }
}

impl Coordinate {
    fn angle_to(&self, coordinate: &Coordinate) -> Angle {
        Angle::of(coordinate.y - self.y, coordinate.x - self.x)
    }

    fn distance_to(&self, coordinate: &Coordinate) -> i64 {
        (coordinate.y - self.y).abs() + (coordinate.x - self.x).abs()
    }
}

fn angle(a: &Coordinate, b: &Coordinate) -> Angle {
    Angle::of(b.y - a.y, b.x - a.x)
}
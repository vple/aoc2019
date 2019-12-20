use multimap::MultiMap;
use std::collections::HashMap;
use std::cmp;

#[aoc_generator(day6)]
fn parse(input: &str) -> (MultiMap<String, String>, HashMap<String, String>) {
    let mut orbiters = MultiMap::new();
    let mut orbitees = HashMap::new();

    for l in input.lines() {
        let objects: Vec<&str> = l.split(')').collect();
        orbiters.insert(objects[0].to_string(), objects[1].to_string());
        orbitees.insert(objects[1].to_string(), objects[0].to_string());
    }

    (orbiters, orbitees)
}

#[aoc(day6, part1)]
fn part1((orbiters, _): &(MultiMap<String, String>, HashMap<String, String>)) -> usize {
    let mut sum = 0;
    let mut depth = 0;
    let mut open = vec![String::from("COM")];
    while open.len() != 0 {
        depth += 1;
        open = open.iter().filter_map(|o| orbiters.get_vec(o)).flat_map(|os| os.to_vec()).collect();
        sum += open.len() * depth;
    }

    sum
}

#[aoc(day6, part2)]
fn part2((_, orbitees): &(MultiMap<String, String>, HashMap<String, String>)) -> usize {
    let mut you_parent = &orbitees["YOU"];
    let mut san_parent = &orbitees["SAN"];

    let you_depth = calculate_depth(&orbitees, you_parent);
    let san_depth = calculate_depth(&orbitees, san_parent);
    let diff = cmp::max(you_depth, san_depth) - cmp::min(you_depth, san_depth);

    if you_depth > san_depth {
        for _ in 0..diff {
            you_parent = &orbitees[you_parent];
        }
    } else {
        for _ in 0..diff {
            san_parent = &orbitees[san_parent];
        }
    }

    let mut additional = 0;
    while you_parent != san_parent {
        additional += 1;
        you_parent = &orbitees[you_parent];
        san_parent = &orbitees[san_parent];
    }

    diff + 2*additional
}

#[derive(Debug)]
struct Orbit {
    orbitee: String,
    orbiter: String,
}

fn calculate_depth(orbitees: &HashMap<String, String>, object: &str) -> usize {
    let mut depth = 0;
    let mut current = object;
    while current != "COM" {
        depth += 1;
        current = &orbitees[current];
    }
    depth
}
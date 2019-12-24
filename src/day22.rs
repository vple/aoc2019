use regex::Regex;

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Technique> {
    let deal_new_stack_regex = Regex::new(r"deal into new stack").unwrap();
    let cut_regex = Regex::new(r"cut (-?\d+)").unwrap();
    let deal_increment_regex = Regex::new(r"deal with increment (\d+)").unwrap();

    input.lines().map(|l| {
        if deal_new_stack_regex.is_match(l) {
            Technique::DealNewStack
        } else if cut_regex.is_match(l) {
            let n = cut_regex.captures(l).unwrap()[1].parse().unwrap();
            Technique::Cut(n)
        } else if deal_increment_regex.is_match(l) {
            let n = deal_increment_regex.captures(l).unwrap()[1].parse().unwrap();
            Technique::DealIncrement(n)
        } else {
            panic!("Invalid technique!");
        }
    })
    .collect()
}

enum Technique {
    DealNewStack,
    Cut(i64),
    DealIncrement(i64),
}

impl Technique {
    fn apply(&self, deck: &mut Vec<usize>) {
        match self {
            Technique::DealNewStack => deck.reverse(),
            Technique::Cut(n) => {
                let n =
                    if *n >= 0 {
                        *n as usize
                    } else {
                        (deck.len() as i64 + *n) as usize
                    };
                *deck = [&deck[n..deck.len()], &deck[0..n]].concat();
            },
            Technique::DealIncrement(n) => {
                let mut target_index = 0;
                let len = deck.len();
                let mut new_deck = vec![0; len];
                let n = *n as usize;
                for i in 0..len {
                    new_deck[target_index] = deck[i];
                    target_index = (target_index + n) % len;
                }
                *deck = new_deck;
            },
        }
    }
}

#[aoc(day22, part1)]
fn part1(techniques: &[Technique]) -> usize {
    let mut deck = (0..10007).collect();
    techniques.iter().for_each(|t| t.apply(&mut deck));
    deck.iter().position(|c| *c == 2019).unwrap()
}

#[aoc(day22, part2)]
fn part2(techniques: &[Technique]) -> usize {
    let size: usize = 119315717514047;
    // let size: usize = 10007;
    let mut one_shuffle = vec![];
    for i in 0..size {
        one_shuffle.push(i);
    }
    // let mut one_shuffle = (0..size).collect();
    println!("asf");
    techniques.iter().for_each(|t| t.apply(&mut one_shuffle));

    let mut deck = (0..size).collect();
    let shuffles: usize = 101741582076661;
    for i in 0..shuffles {
        if i % 1_000 == 0 {
            println!("{}", i);
        }
        quick_shuffle(&mut deck, &one_shuffle);
    }
    deck[2020]
}

fn quick_shuffle(deck: &mut Vec<usize>, shuffle_result: &[usize]) {
    let mut new_deck = vec![0; deck.len()];
    for i in 0..deck.len() {
        new_deck[i] = deck[shuffle_result[i]];
    }
    *deck = new_deck;
}
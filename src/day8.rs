const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[aoc_generator(day8)]
fn parse(input: &str) -> Vec<Layer> {
    let pixels: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    pixels.chunks(WIDTH * HEIGHT).map(|pixels| Layer { pixels: pixels.to_vec() }).collect()
}

struct Layer {
    pixels: Vec<u32>,
}

impl Layer {
    fn count_values(&self, value: u32) -> usize {
        self.pixels.iter().filter(|v| **v == value).count()
    }

    fn merge(mut self, layer: &Layer) -> Layer {
        self.pixels =
            self.pixels.iter().zip(layer.pixels.iter())
                .map(|(a, b)| if *a == 2 { *b } else { *a })
                .collect();
        self
    }
}

#[aoc(day8, part1)]
fn part1(layers: &Vec<Layer>) -> usize {
    let fewest_zeros =
        layers.iter()
            .min_by_key(|l| l.count_values(0))
            .unwrap();

    let ones = fewest_zeros.count_values(1);
    let twos = fewest_zeros.count_values(2);

    ones * twos
}

#[aoc(day8, part2)]
fn part2(layers: &Vec<Layer>) -> String {
    let initial = Layer { pixels: vec![2; WIDTH * HEIGHT] };
    let full = layers.iter().fold(initial, |acc, layer| acc.merge(layer));


    println!("{:?}", full.pixels);

    let mut result = String::new();

    result.push_str("\n");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if full.pixels[WIDTH * y + x] == 0 {
                result.push_str(" ");
            } else {
                result.push_str("▓");
            }
            // result.push_str(&full.pixels[HEIGHT * x + y].to_string());
        }
        result.push_str("\n");
    }
    result
}
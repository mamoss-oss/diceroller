use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::io::stdin;
use std::process::exit;

struct Dice {
    sides: u32,
    result: u32,
}

impl Dice {
    fn new(sides: u32) -> Self {
        Self { sides, result: 0 }
    }
    fn roll(&mut self, rng: &mut ThreadRng) {
        self.result = rng.gen_range(1..=self.sides)
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut input_dices = String::new();
    let mut input_sides = String::new();
    println!("### Dice roller ###");
    loop {
        println!("How many dice do you want to roll ?");
        stdin()
            .read_line(&mut input_dices)
            .expect("Failed to read line");
        let number_dices: u32 = match input_dices.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("How many sides should each dice have ?");
        stdin()
            .read_line(&mut input_sides)
            .expect("Failed to read line");
        let number_sides: u32 = match input_dices.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut my_dice = Dice::new(number_sides);
        for i in 1..=number_dices {
            my_dice.roll(&mut rng);
            println!("Roll NR {i}:: {}", my_dice.result)
        }
        exit(0)
    }
}

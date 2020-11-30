use rand::distributions::{Distribution, Uniform };
use std::io;

#[derive(Debug)]
pub struct Mastermind {
    solution: String
}

impl Mastermind {
    pub fn guess(&self, input: &str) -> (usize, usize) {
        // TODO: Add guessing method
        let mut correct_pos = 0;
        let mut correct_char = 0;
        let mut input_chs = input.chars();
        for ch in self.solution.chars() {
            let guess_ch = input_chs.nth(0).unwrap();
            if guess_ch == ch {
                correct_pos += 1;
            } else if input.contains(ch) {
                correct_char += 1
            }

        }
        (correct_pos, correct_char)
    }

    pub fn new(input: Option<String>)  -> Mastermind {
        // TODO: Add random string when option is None
        let mut rng = rand::thread_rng();
        let pool  = Uniform::from(1..7);
        match input {
            None => {
                let mut result = String::new();
                for _ in 1..5 {
                    result.push_str(&pool.sample(&mut rng).to_string());
                }
                Mastermind { solution: result}
            },
            Some(sol) => Mastermind { solution: sol }
        }
    }
}

pub fn play() {
    // This is for playing Mastermind
    let game = Mastermind::new(None);
    let stdin = io::stdin();
    let mut guess = String::new();
    for _ in 1..6 {
        match stdin.read_line(&mut guess) {
            Ok(_) => println!("Guess is {:?}", game.guess(&guess)),
            Err(err) => println!("Error is {}", &err)
        }
    }
}

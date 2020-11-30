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
        let input_chs: Vec<char> = input.chars().collect();
        for (index, ch) in self.solution.char_indices() {
            let guess_ch = input_chs[index];
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
    // println!("{}", game.solution);
    let stdin = io::stdin();
    let mut guess = String::new();
    for _ in 1..6 {
        if let Ok(_) = stdin.read_line(&mut guess) {
            let result = game.guess(&guess);
            println!("Your guess is {} and answer is {:?}", &guess.trim(), &result);
            if result == (4, 0) {
                println!("Congratulations! You exist.");
                break;
            }
        } else {
            panic!("Input error")
        }
        
        guess.clear();
    }
}

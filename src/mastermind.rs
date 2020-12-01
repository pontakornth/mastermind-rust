use rand::distributions::{Distribution, Uniform };
use std::io;

type Hint = (usize, usize);

#[derive(Debug)]
enum MastermindError {
    OutOfRange
}

#[derive(Debug)]
pub struct Mastermind {
    solution: String
}

impl Mastermind {
    pub fn guess(&self, input: &str) -> Result<Hint,MastermindError> {
        // TODO: Make this use result
        let input_chs: Vec<char> = input.chars().collect();
        let solution_chs: Vec<char> = self.solution.chars().collect();
        if input_chs.len() != solution_chs.len() {
            return Err(MastermindError::OutOfRange)
        }
        let mut correct_pos = 0;
        let mut correct_char = 0;
        for (index, ch) in self.solution.char_indices() {
            let guess_ch = input_chs[index];
            if guess_ch == ch {
                correct_pos += 1;
            } else if input.contains(ch) {
                correct_char += 1
            }

        }
        Ok((correct_pos, correct_char))
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

fn str_hint(hint: Hint) -> String {
    let mut output = String::new();
    for _ in 0..hint.0 {
        output.push('*');
    }
    for _ in 0..hint.1 {
        output.push('o');
    }
    while output.chars().count() < 4 {
        output.push('-');
    }
    output
}

pub fn play() {
    // This is for playing Mastermind
    let game = Mastermind::new(None);
    // println!("{}", game.solution);
    let stdin = io::stdin();
    let mut guess = String::new();
    for _ in 1..6 {
        if let Ok(_) = stdin.read_line(&mut guess) {
            guess = guess.trim().to_string();
            let result = game.guess(&guess);
            match result {
                Err(x) => println!("{:?}", x),
                Ok(res) => {
                println!("Your guess is {} and answer is {}", &guess, str_hint(res));
                if res == (4, 0) {
                    println!("Congratulations! You exist.");
                    break;
                }
            }
                
        }
    } else {
            panic!("Input error")
    }
        
        guess.clear();
    }
}

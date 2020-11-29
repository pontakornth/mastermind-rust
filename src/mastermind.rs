use rand::distributions::{Distribution, Uniform };

#[derive(Debug)]
pub struct Mastermind {
    solution: String
}

impl Mastermind {
    pub fn guess(&self, input: String) {
        // TODO: Add guessing method
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
            }
                ,
            Some(sol) => Mastermind { solution: sol }
        }
    }
}

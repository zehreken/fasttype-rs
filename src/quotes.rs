use rand::prelude::ThreadRng;
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

pub struct QuoteManager {
    quotes: Vec<String>,
    previous_index: usize,
    rnd: ThreadRng,
}

impl QuoteManager {
    pub fn new() -> QuoteManager {
        let mut file = File::open("quotes.txt").expect("Error while opening quotes.txt");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let quotes: Vec<String> = contents.lines().map(|q| q.to_owned()).collect();

        let mut rnd = rand::thread_rng();
        Self {
            quotes,
            previous_index: rnd.gen_range(0..10),
            rnd,
        }
    }

    pub fn get_random_quote(&mut self) -> String {
        let mut new_index = self.rnd.gen_range(0..self.quotes.len());
        if new_index == self.previous_index {
            new_index += 1;
            new_index %= self.quotes.len();
        }
        self.previous_index = new_index;
        self.quotes[new_index].clone()
    }
}

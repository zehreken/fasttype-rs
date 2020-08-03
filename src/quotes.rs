use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

pub struct QuoteManager {
    quotes: Vec<String>,
}

impl QuoteManager {
    pub fn new() -> QuoteManager {
        let mut file = File::open("quotes.txt").expect("Error while opening quotes.txt");
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let quotes: Vec<String> = contents.lines().map(|q| q.to_owned()).collect();
        Self { quotes }
    }

    pub fn get_random_quote(&self) -> String {
        String::from(self.quotes[rand::thread_rng().gen_range(0, self.quotes.len())].clone())
    }
}

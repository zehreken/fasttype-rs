use super::quotes::*;
use console::style;
use fmt::Display;
use std::fmt;

pub struct Round {
    pub quote: String,
    pub chars: Vec<char>,
    pub input_chars: Vec<char>,
    pub match_chars: Vec<bool>,
    pub total_keys: u32,
    pub correct_keys: u32,
    pub char_index: i32,
}

impl Round {
    pub fn new(quote_manager: &mut QuoteManager) -> Self {
        Self {
            quote: quote_manager.get_random_quote(),
            chars: Vec::new(),
            input_chars: Vec::new(),
            match_chars: Vec::new(),
            total_keys: 0,
            correct_keys: 0,
            char_index: 0,
        }
    }

    pub fn end(&self, duration: u128) -> RoundResult {
        RoundResult {
            quote: self.quote.clone(),
            total_keys: self.total_keys,
            correct_keys: self.correct_keys,
            duration,
        }
    }
}

pub struct RoundResult {
    pub quote: String,
    pub total_keys: u32,
    pub correct_keys: u32,
    pub duration: u128,
}

impl Display for RoundResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wpm: f32 = (self.quote.len() as f32 / 5 as f32) / (self.duration as f32 / 60000 as f32);
        let accuracy: f32 = self.correct_keys as f32 / self.total_keys as f32 * 100.0;
        let time_in_seconds: f32 = self.duration as f32 / 1000.0;
        write!(
            f,
            "WPM: {:.2} || Accuracy: {:.2}% || Time(s): {:.2}\n",
            style(wpm).yellow(),
            style(accuracy).yellow(),
            style(time_in_seconds).yellow(),
        )
    }
}

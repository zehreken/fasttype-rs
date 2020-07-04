use console::style;
use console::Key;
use rand::Rng;
use std::fmt;
use std::time::Instant;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[0;41m";
const GREEN: &str = "\x1b[0;32m";
const YELLOW: &str = "\x1b[0;33m";

fn main() {
    start();
}

struct Round {
    quote: String,
    input: String,
    chars: Vec<char>,
    input_chars: Vec<char>,
    match_chars: Vec<bool>,
    total_keys: u32,
    correct_keys: u32,
    char_index: i32,
}

impl Round {
    fn new() -> Round {
        Round {
            quote: get_random_quote(),
            input: String::from("_"),
            chars: Vec::new(),
            input_chars: Vec::new(),
            match_chars: Vec::new(),
            total_keys: 0,
            correct_keys: 0,
            char_index: 0,
        }
    }

    fn end(&self, duration: u128) -> Result {
        Result {
            quote: self.quote.clone(),
            total_keys: self.total_keys,
            correct_keys: self.correct_keys,
            duration: duration,
        }
    }
}

struct Result {
    quote: String,
    total_keys: u32,
    correct_keys: u32,
    duration: u128,
}

impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

fn get_random_quote() -> String {
    let quotes = ["Nothing is so difficult as not deceiving oneself.",
    "Talent is cheaper than table salt. What separates the talented individual from the successful one is a lot of hard work.",
    "The harder you work, the luckier you get.",
    "Don't ignore your dreams; don't work too much; say what you think; cultivate friendships; be happy.",
    "I was an ordinary person who studied hard. There are no miracle people. It happens they get interested in this thing and they learn all this stuff, but they're just people.",
    "There are more things, Lucilius, that frighten us than injure us, and we suffer more in imagination than in reality.",
    "Every new beginning comes from some other beginning's end.",
    "Luck is what happens when preparation meets opportunity.",
    "Fate leads the willing, and drags along the reluctant.",
    "Life, if well lived, is long enough.",
    "As is a tale, so is life: not how long it is, but how good it is, is what matters.",
    "Sometimes even to live is an act of courage.",
    "All cruelty springs from weakness.",
    "Wherever there is a human being, there is an opportunity for a kindness.",
    "One of the most beautiful qualities of true friendship is to understand and to be understood.",
    "Practice does not make perfect. Only perfect practice makes perfect."];

    String::from(quotes[rand::thread_rng().gen_range(0, quotes.len())])
}

fn new_round(term: &console::Term, round: &mut Round) {
    let mut colored_quote = String::new();
    colored_quote.push_str(YELLOW);
    colored_quote.push_str(round.quote.as_str());
    colored_quote.push_str(RESET);

    for ch in round.quote.chars() {
        round.chars.push(ch);
    }

    term.write_line(colored_quote.as_str())
        .expect("Error while writing line");
    term.write_line(&round.input[..])
        .expect("Error while writing line");
    term.hide_cursor().expect("Error while hiding cursor");
}

fn start() {
    let mut now = Instant::now();
    let term = console::Term::stdout();
    let mut results: Vec<Result> = vec![];

    let mut round = Round::new();
    new_round(&term, &mut round);

    let mut res_key;
    'running: loop {
        res_key = term.read_key();
        match res_key.unwrap() {
            Key::Char(c) => {
                if round.char_index < round.chars.len() as i32 {
                    round.input.pop();
                    round.input.push(c);
                    round.input.push_str("_");
                    round.input_chars.push(c);
                    round.total_keys += 1;
                    if round.chars[round.char_index as usize]
                        == round.input_chars[round.char_index as usize]
                    {
                        round.match_chars.push(true);
                        round.correct_keys += 1;
                    } else {
                        round.match_chars.push(false);
                    }

                    round.char_index += 1;
                }
            }
            Key::Escape => break 'running,
            Key::Enter => {
                if round.char_index == round.chars.len() as i32 {
                    // break 'running;
                    // Next sentence
                    let duration = (Instant::now() - now).as_millis();
                    now = Instant::now();
                    let result = round.end(duration);
                    println!("{}", result);
                    results.push(result);
                    round = Round::new();
                    new_round(&term, &mut round);
                }
            }
            Key::Backspace => {
                round.input.pop();
                round.input.pop();
                round.input.push_str("_");
                round.input_chars.pop();
                round.match_chars.pop();
                if round.char_index > 0 {
                    round.char_index -= 1;
                }
            }
            _ => {}
        }

        let mut input_temp = String::new();
        let asd: Vec<(&char, &bool)> = round
            .input_chars
            .iter()
            .zip(round.match_chars.iter())
            .collect();
        for (a, b) in asd {
            if *b {
                input_temp.push_str(GREEN);
            } else {
                input_temp.push_str(RED);
            }
            input_temp.push(*a);
        }
        input_temp.push_str(RESET);
        term.move_cursor_up(1)
            .expect("Error while moving cursor up");
        term.clear_line().expect("Error while clearing line");
        term.write_line(&input_temp[..])
            .expect("Error while writing line");
    } // 'running

    // Print session result here.
    let mut sum_chars = 0;
    let mut sum_duration = 0;
    let mut sum_total_keys = 0;
    let mut sum_correct_keys = 0;
    for r in results {
        sum_chars += r.quote.len();
        sum_duration += r.duration;
        sum_total_keys += r.total_keys;
        sum_correct_keys += r.correct_keys;
    }

    let session_wpm = (sum_chars as f32 / 5 as f32) / (sum_duration as f32 / 60000 as f32);
    let session_accuracy = sum_correct_keys as f32 / sum_total_keys as f32 * 100.0;
    println!(
        "{}Session Summary{}\nWPM: {:.2}\nAccuracy: {:.2}%",
        RED,
        RESET,
        style(session_wpm).yellow(),
        style(session_accuracy).yellow()
    );
    term.show_cursor().expect("Error while showing cursor");
}

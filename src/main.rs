use console::style;
use console::Key;
use console::Style;
use rand::Rng;
use std::time::{Duration, Instant};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[0;41m";
const GREEN: &str = "\x1b[0;32m";
const YELLOW: &str = "\x1b[0;33m";

fn main() {
    println!("Hello, world!");
    start();
}

struct Round {
    quote: String,
    input: String,
    chars: Vec<char>,
    input_chars: Vec<char>,
    match_chars: Vec<bool>,
    total_keys: u32,
    wrong_keys: u32,
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
            wrong_keys: 0,
            char_index: -1,
        }
    }

    fn end(&self, duration: u128) -> Result {
        let mut true_count = 0;
        let mut false_count = 0;
        for b in &self.match_chars {
            if *b {
                true_count += 1;
            } else {
                false_count += 1;
            }
        }

        Result {
            quote: self.quote.clone(),
            total_keys: true_count + false_count,
            wrong_keys: false_count,
            duration: duration,
        }
    }
}

struct Result {
    quote: String,
    total_keys: u32,
    wrong_keys: u32,
    duration: u128,
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
    "One of the most beautiful qualities of true friendship is to understand and to be understood."];

    String::from(quotes[rand::thread_rng().gen_range(0, quotes.len())])
}

fn print_result(result: &Result) {
    let wpm: f32 = (result.quote.len() as f32 / 5 as f32) / (result.duration as f32 / 60000 as f32);
    println!(
        "WPM: {} || {} / {} || Time: {} || {}",
        wpm,
        style(result.wrong_keys).yellow(),
        style(result.total_keys).red(),
        result.duration,
        result.quote,
    );
}

fn _print_result(now: &Instant, round: &Round) {
    let mut true_count = 0;
    let mut false_count = 0;
    for b in round.match_chars.clone() {
        // TODO: Remove this clone
        if b {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }
    let duration = Instant::now() - *now;
    println!(
        "Exit: {} / {} Time: {}",
        style(true_count).yellow(),
        style(false_count).red(),
        duration.as_secs()
    );

    println!("\x1b[33mThis is colored text.");
    println!("This is colored text.\x1b[0m");
    let cyan = Style::new().cyan();
    println!("This is {} neat", cyan.reverse().apply_to("quite"));
}

fn new_round(term: &console::Term, round: &mut Round) {
    let mut temp_colored_string = String::new();
    temp_colored_string.push_str(YELLOW);
    temp_colored_string.push_str(round.quote.as_str());
    temp_colored_string.push_str(RESET);

    for ch in round.quote.chars() {
        round.chars.push(ch);
        //     if index & 4 == 0 {
        //         temp_colored_string.push_str(RED);
        //     } else {
        //         temp_colored_string.push_str(GREEN);
        //     }
        //     index += 1;
    }

    term.write_line(temp_colored_string.as_str())
        .expect("Error while writing line");
    term.write_line(&round.input[..])
        .expect("Error while writing line");
    term.hide_cursor().expect("Error while hiding cursor");
}

fn start() {
    let mut now = Instant::now();

    let term = console::Term::stdout();
    let mut round = Round::new();
    new_round(&term, &mut round);

    let mut results: Vec<Result> = vec![];

    let mut res;
    'running: loop {
        res = term.read_key();
        match res.unwrap() {
            Key::Char(c) => {
                round.input.pop();
                round.input.push(c);
                round.input.push_str("_");
                round.input_chars.push(c);
                round.char_index += 1;

                if round.char_index == round.chars.len() as i32 {
                    break 'running;
                    // Next sentence
                }
                if round.chars[round.char_index as usize]
                    == round.input_chars[round.char_index as usize]
                {
                    round.match_chars.push(true);
                } else {
                    round.match_chars.push(false);
                }
            }
            Key::Escape => break 'running,
            Key::Enter => {
                if round.char_index + 1 == round.chars.len() as i32 {
                    // break 'running;
                    // Next sentence
                    let duration = (Instant::now() - now).as_millis();
                    now = Instant::now();
                    let result = round.end(duration);
                    print_result(&result);
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
                if round.char_index >= 0 {
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
}

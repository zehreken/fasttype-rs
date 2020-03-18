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
    chars: Vec<char>,
    input_chars: Vec<char>,
    match_chars: Vec<bool>,
    char_index: i32,
    duration: u32,
}

impl Round {
    fn new() -> Round {
        Round {
            quote: get_random_quote(),
            chars: Vec::new(),
            input_chars: Vec::new(),
            match_chars: Vec::new(),
            char_index: -1,
            duration: 0,
        }
    }
}

struct Result {
    quote: String,
    total_keys: u32,
    wrong_keys: u32,
    duration: u32,
}

fn get_random_quote() -> String {
    let quotes = ["Nothing is so difficult as not deceiving oneself.",
    "Talent is cheaper than table salt. What separates the talented individual from the successful one is a lot of hard work.",
    "The harder you work, the luckier you get.",
    "Don't ignore your dreams; don't work too much; say what you think; cultivate friendships; be happy.",
    "I was an ordinary person who studied hard. There are no miracle people. It happens they get interested in this thing and they learn all this stuff, but they're just people."];

    String::from(quotes[rand::thread_rng().gen_range(0, quotes.len())])
}

fn print_result(now: &Instant, round: &Round) {
    // Move Round, it is not needed after this anyways
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

fn start() {
    let mut now = Instant::now();

    let mut round = Round::new();

    let mut temp_colored_string = String::new();
    temp_colored_string.push_str(YELLOW);
    temp_colored_string.push_str(round.quote.as_str());
    temp_colored_string.push_str(RESET);

    // let mut index = 0;
    for ch in round.quote.chars() {
        round.chars.push(ch);
        //     if index & 4 == 0 {
        //         temp_colored_string.push_str(RED);
        //     } else {
        //         temp_colored_string.push_str(GREEN);
        //     }
        //     index += 1;
    }

    let term = console::Term::stdout();
    let mut input = String::from("_");
    term.write_line(temp_colored_string.as_str())
        .expect("Error while writing line");
    term.write_line(&input[..])
        .expect("Error while writing line");
    term.hide_cursor().expect("Error while hiding cursor");
    let mut res;
    'running: loop {
        res = term.read_key();
        match res.unwrap() {
            Key::Char(c) => {
                input.pop();
                input.push(c);
                input.push_str("_");
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
                    print_result(&now, &round);
                    round = Round::new();
                }
            }
            Key::Backspace => {
                input.pop();
                input.pop();
                input.push_str("_");
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

    print_result(&now, &round);
}

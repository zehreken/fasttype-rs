use console::style;
use console::Key;
use round::*;
use std::time::Instant;

mod quotes;
mod round;

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[0;41m";
const GREEN: &str = "\x1b[0;32m";
const YELLOW: &str = "\x1b[0;33m";
const CURSOR: char = '_';

fn main() {
    start();
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
    term.write_line(&CURSOR.to_string()[..]).unwrap();
    term.hide_cursor().expect("Error while hiding cursor");
}

fn start() {
    let mut now = Instant::now();
    let term = console::Term::stdout();
    let mut quote_manager = quotes::QuoteManager::new();
    let mut results: Vec<RoundResult> = vec![];

    term.clear_screen()
        .expect("Error while clearing the screen");

    let mut round = Round::new(&mut quote_manager);
    new_round(&term, &mut round);

    let mut res_key;
    'running: loop {
        res_key = term.read_key();
        match res_key.unwrap() {
            Key::Char(c) => {
                if round.char_index < round.chars.len() as i32 {
                    if round.input_chars.len() == 0 {
                        now = Instant::now(); // Start time after the first key stroke
                    }
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
                    // Next sentence
                    let duration = (Instant::now() - now).as_millis();
                    now = Instant::now(); // Reset now
                    let result = round.end(duration);
                    println!("{}", result);
                    results.push(result);
                    round = Round::new(&mut quote_manager);
                    new_round(&term, &mut round);
                }
            }
            Key::Backspace => {
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
        input_temp.push(CURSOR);
        term.move_cursor_up(1)
            .expect("Error while moving cursor up");
        term.clear_line().expect("Error while clearing line");
        term.write_line(&input_temp[..])
            .expect("Error while writing line");
    } // 'running

    print_session_result(&term, results);
}

fn print_session_result(term: &console::Term, results: Vec<RoundResult>) {
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
        "{}SESSION SUMMARY{}\nWPM: {:.2}\nAccuracy: {:.2}%",
        RED,
        RESET,
        style(session_wpm).yellow(),
        style(session_accuracy).yellow()
    );
    term.show_cursor().expect("Error while showing cursor");
}

use console::style;
use console::Key;
use console::Style;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    start();
}

pub fn start() {
    let quotes = ["Nothing is so difficult as not deceiving oneself.",
    "Talent is cheaper than table salt. What separates the talented individual from the successful one is a lot of hard work.",
    "The harder you work, the luckier you get.",
    "Don't ignore your dreams; don't work too much; say what you think; cultivate friendships; be happy.",
    "I was an ordinary person who studied hard. There are no miracle people. It happens they get interested in this thing and they learn all this stuff, but they're just people."];
    let sample_text = quotes[rand::thread_rng().gen_range(0, quotes.len())];
    let mut chars: Vec<char> = Vec::new();
    let mut input_chars: Vec<char> = Vec::new();
    let mut match_chars: Vec<bool> = Vec::new();
    let mut char_index: i32 = -1;

    let reset = "\x1b[0m";
    let red = "\x1b[0;31m";
    let green = "\x1b[0;32m";
    let yellow = "\x1b[0;33m";
    let mut temp_colored_string = String::new();
    let mut index = 0;

    for ch in sample_text.chars() {
        chars.push(ch);
        temp_colored_string.push(ch);
        if index & 4 == 0 {
            temp_colored_string.push_str(red);
        } else {
            temp_colored_string.push_str(green);
        }
        index += 1;
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
                input_chars.push(c);
                char_index += 1;
                if chars[char_index as usize] == input_chars[char_index as usize] {
                    match_chars.push(true);
                } else {
                    match_chars.push(false);
                }
            }
            Key::Escape => break 'running,
            Key::Backspace => {
                input.pop();
                input.pop();
                input.push_str("_");
                input_chars.pop();
                match_chars.pop();
                if char_index >= 0 {
                    char_index -= 1;
                }
            }
            _ => {}
        }

        let mut input_temp = String::new();
        let asd: Vec<(&char, &bool)> = input_chars.iter().zip(match_chars.iter()).collect();
        for (a, b) in asd {
            if *b {
                input_temp.push_str(green);
            } else {
                input_temp.push_str(red);
            }
            input_temp.push(*a);
        }
        term.move_cursor_up(1)
            .expect("Error while moving cursor up");
        term.clear_line().expect("Error while clearing line");
        term.write_line(&input_temp[..])
            .expect("Error while writing line");
    }
    let mut true_count = 0;
    let mut false_count = 0;
    for b in match_chars {
        if b {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }
    println!(
        "Exit: {} / {}",
        style(true_count).yellow(),
        style(false_count).red()
    );

    println!("\x1b[33mThis is colored text.");
    println!("This is colored text.\x1b[0m");
    let cyan = Style::new().cyan();
    println!("This is {} neat", cyan.apply_to("quite"));
}

use rand::Rng;

pub fn get_random_quote() -> String {
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

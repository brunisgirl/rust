fn is_yelled_alphabetic(s: &str) -> bool {
    s.chars().any(char::is_alphabetic) && !(s.chars().any(char::is_lowercase))
}

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message == "" {
        "Fine. Be that way!"
    } else {
        let is_question = message.ends_with('?');
        let is_yelled = is_yelled_alphabetic(message);
        match (is_question, is_yelled) {
            (true, true) => "Calm down, I know what I'm doing!",
            (true, false) => "Sure.",
            (false, true) => "Whoa, chill out!",
            (false, false) => "Whatever.",
        }
    }
}

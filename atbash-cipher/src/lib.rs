static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
static REVERSE_ALPHABET: &str = "zyxwvutsrqponmlkjihgfedcba";

fn remove_non_alphanumeric(input: &str) -> String {
    let mut string = String::new();
    for c in input.to_lowercase().chars() {
        if (c.is_alphabetic() && c.is_ascii()) || c.is_numeric() {
            string.push(c);
        }
    }
    string
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let formatted_string = remove_non_alphanumeric(plain);
    let mut encoded_string = String::with_capacity(plain.len() + (plain.len() / 5)); // Add 20% because of the spaces that get inserted
    let mut counter = 0;
    let mut length = 0;
    for letter in formatted_string.chars() {
        counter += 1;
        length += 1;
        if letter.is_numeric() {
            encoded_string.push(letter);
        } else {
            let char_index = ALPHABET.chars().position(|c| c == letter).unwrap();
            encoded_string.push(REVERSE_ALPHABET.chars().nth(char_index).unwrap());
        }
        if counter > 4 && length < formatted_string.chars().count() {
            counter = 0;
            encoded_string.push(' ');
        }
    }
    encoded_string
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let mut decoded_string = String::new();
    for letter in cipher.to_lowercase().chars().filter(|c| (c.is_alphabetic() && c.is_ascii()) || c.is_numeric()) {

        if letter.is_numeric() {
            decoded_string.push(letter);
        } else {
            let char_index = ALPHABET.chars().position(|c| c == letter).unwrap();
            decoded_string.push(REVERSE_ALPHABET.chars().nth(char_index).unwrap());
        }
    }
    decoded_string
}



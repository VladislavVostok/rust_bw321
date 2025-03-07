use std::collections::HashMap;

fn decode_morze(code_str: &str) -> String {
    let mut morse_code = HashMap::new();
    morse_code.insert(".-", "A");
    morse_code.insert("-...", "B");
    morse_code.insert(".--", "W");
    morse_code.insert("--.", "G");
    morse_code.insert("-..", "D");
    morse_code.insert(".", "E");
    morse_code.insert("...-", "V");
    morse_code.insert("--..", "Z");
    morse_code.insert("..", "I");
    morse_code.insert(".---", "J");
    morse_code.insert("-.-", "K");
    morse_code.insert(".-..", "L");
    morse_code.insert("--", "M");
    morse_code.insert("-.", "N");
    morse_code.insert("---", "O");
    morse_code.insert(".-.", "R");
    morse_code.insert("...", "S");
    morse_code.insert("-", "T");
    morse_code.insert("..-", "U");
    morse_code.insert("..-.", "F");
    morse_code.insert("....", "H");
    morse_code.insert("-.-.", "C");
    morse_code.insert("--.-", "Q");
    morse_code.insert("-.--", "Y");
    morse_code.insert("-..-", "X");


    let word_chars: Vec<&str> = code_str.split("   ").collect();

    let mut decoded_text = String::new();

    for word in word_chars {
        let morse_chars: Vec<&str> = word.split(' ').collect();

        for code in morse_chars {
            if let Some(&char) = morse_code.get(code)
            {
                decoded_text.push_str(char);
            }

        }
        decoded_text.push(' ');
    }
    decoded_text.trim().to_string()
}

fn main() {

    let decode = decode_morze(".... . -.--   .--- ..- -.. .");

    println!("{decode}");
}


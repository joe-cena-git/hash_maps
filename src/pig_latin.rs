// Convert strings to Pig Latin.
// The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!
pub fn to_pig_latin(word: &str) -> Option<String> {
    
    // get length of the input word
    let word_length: usize = word.len();

    if word_length == 0 {
        // word is empty, return None
        return None;
    } else {
        // word has at least 1 character
        let mut chars = word.chars();

        let first_char: Option<char> = chars.next();

        match first_char {
            Some(first_char_value) => {
                if is_vowel(first_char_value) {
                    // first letter is a vowel
                    let pig_latin_word: String = format!("{word}-hay");
                    return Some(pig_latin_word);
                } else {
                    // first letter is NOT a vowel
                    let mut pig_latin_word: String = String::new();
                    let mut on_first_letter = true;
                    loop {
                        let c: Option<char> = chars.next();
                        match c {
                            // we are still iterating through the chars of the original word
                            Some(value) => {
                                // if the original input word had a capitalized first letter,
                                // and we are at char_iter 1, we capitalize the first letter of the returned word
                                if char::is_ascii_uppercase(&first_char_value) && on_first_letter {
                                    pig_latin_word.push(value.to_ascii_uppercase());
                                    on_first_letter = false;
                                } else {
                                    // for all other letters, make them lowercase
                                    pig_latin_word.push(value.to_ascii_lowercase());
                                }
                            },
                            None => {
                                // we have reached the end of the chars in the original word
                                // add the ending to the word and return the result
                                pig_latin_word.push(first_char_value.to_ascii_lowercase());
                                pig_latin_word.push_str("ay");
                                return Some(pig_latin_word)
                            }
                        }
                    }
                }
            },
            // if first character is None, return None
            // should never reach here with existing word length check above
            None => return None
        }

        
    }
}

fn is_vowel(c: char) -> bool {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let lowercase_char: char = c.to_ascii_lowercase();
    return vowels.contains(&lowercase_char);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn capital_vowel_word_returns_capital_vowel_word() {
        let input_word = String::from("Zebra");
        let expected_word = String::from("Ebrazay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn lowercase_consonant_word_returns_lowercase_consonant_word() {
        let input_word = String::from("modern");
        let expected_word = String::from("odernmay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn capital_consonant_word_returns_capital_consonant_word() {
        let input_word = String::from("Modern");
        let expected_word = String::from("Odernmay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn empty_string_returns_none() {
        let input_word = String::from("");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(None, output_word);
    }

    #[test]
    fn lowercase_vowel_word_returns_hay_suffix() {
        let input_word = String::from("apple");
        let expected_word = String::from("apple-hay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn uppercase_vowel_word_preserves_capitalization_with_hay_suffix() {
        let input_word = String::from("Apple");
        let expected_word = String::from("Apple-hay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn single_consonant_letter_returns_consonant_word() {
        let input_word = String::from("b");
        let expected_word = String::from("bay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn single_vowel_letter_returns_hay_suffix() {
        let input_word = String::from("a");
        let expected_word = String::from("a-hay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn all_caps_consonant_word_capitalizes_first_letter_of_result() {
        let input_word = String::from("FIRST");
        let expected_word = String::from("Irstfay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }

    #[test]
    fn two_letter_consonant_word_returns_consonant_word() {
        let input_word = String::from("by");
        let expected_word = String::from("ybay");
        let output_word = to_pig_latin(&input_word);
        assert_eq!(Some(expected_word), output_word);
    }
}
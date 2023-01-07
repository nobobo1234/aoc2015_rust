use advent_of_code::char_windows::CharWindows;

// Part 1: Function that counts the number of characters by folding it using
// an accumulator that checks if each element contains a vowel.
fn count_vowels(word: &str) -> u32 {
    word.chars().fold(0, |acc, e| acc + "aeiou".contains(e) as u32)
}

// Part 1: Function that checks if a string has a repeated letter by checking the
// previous value of the iterator and comparing it with the current value.
fn contains_repeated_letter(word: &str) -> bool {
    let mut prev_letter = '\0';
    for letter in word.chars() {
        if prev_letter == letter {
            return true;
        }

        prev_letter = letter
    }

    false
}

// Part 1: Function that checks if a word contains none of the substrings in
// bad_substrings.
fn contains_no_bad_substrings(word: &str, bad_substrings: Vec<&str>) -> bool {
    bad_substrings.iter().all(|bad_substring| !word.contains(bad_substring))
}

// Part 2: Function that checks if a two-letter pattern appears at least twice
// in a string without overlap.
fn pattern_appears_twice(word: &str) -> bool {
    // Generate an iterator over each two-letter pattern as Strings.
    let patterns = word.char_windows(2);
    for pattern in patterns {
        // If the pattern appears at least twice in the word return true.
        if word.matches(&pattern).count() >= 2 {
            return true
        }
    }
    false
}

// Part 2: Function that checks if a word contains at least two the same letters
// with exactly one letter inbetween.
fn has_letter_inbetween(word: &str) -> bool {
    // Generate an iterator over each pattern as a tuple but skip 2 so we get
    // the letter two after the current one.
    let patterns = word.chars()
        .zip(word.chars().skip(2));
    for pattern in patterns {
        // Check if both elements of the tuple are the same, then we know we got
        // the pattern we wanted.
        if pattern.0 == pattern.1 {
            return true
        }
    }
    false
}

// Function that checks if a word is nice with the rules from part1.
fn is_nice(word: &str) -> bool {
    // 1. Check if it contains at least three vowels.
    let contains_three_vowels = count_vowels(word) >= 3;

    // 2. Check if it contains one letter twice in a row
    let letter_twice_in_a_row = contains_repeated_letter(word);

    // 3. It does not contain any prohibited substrings
    let contains_bad_substrings =
        contains_no_bad_substrings(word, vec!["ab", "cd", "pq", "xy"]);

    contains_three_vowels && letter_twice_in_a_row && contains_bad_substrings
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut amount_nice_words = 0;
    for word in input.split("\n") {
        if is_nice(word) {
            amount_nice_words += 1;
        }
    }

    Some(amount_nice_words)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut amount_nice_words = 0;
    for word in input.split("\n") {
        // Since there are only two conditions we don't make a seperate
        // function to check for niceness.
        if pattern_appears_twice(word) && has_letter_inbetween(word) {
            amount_nice_words += 1;
        }
    }

    Some(amount_nice_words)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_letters() {
        assert_eq!(contains_repeated_letter("xx"), true);
        assert_eq!(contains_repeated_letter("abcdde"), true);
        assert_eq!(contains_repeated_letter("aabbccdd"), true);
        assert_eq!(contains_repeated_letter("jchzalrnumimnmhp"), false);
    }

    #[test]
    fn test_niceness() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_pattern_appears_twice() {
        assert_eq!(pattern_appears_twice("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(pattern_appears_twice("xxyxx"), true);
        assert_eq!(pattern_appears_twice("uurcxstgmygtbstg"), true);
    }
}

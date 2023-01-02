pub fn part_one(input: &str) -> Option<i32> {
    let mut count: i32 = 0;
    for character in input.split("") {
        count += match character {
            "(" => 1,
            ")" => -1,
            &_ => 0
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut count: i32 = 0;
    for (index, character) in input.split("").enumerate() {
        count += match character {
            "(" => 1,
            ")" => -1,
            &_ => 0
        };

        // If the count is -1, we're on the basement and we need to quit.
        if count == -1 {
            return Some(index);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), Some(0));
        assert_eq!(part_one("()()"), Some(0));
        assert_eq!(part_one("((("), Some(3));
        assert_eq!(part_one("(()(()("), Some(3));
        assert_eq!(part_one("())"), Some(-1));
        assert_eq!(part_one("))("), Some(-1));
        assert_eq!(part_one(")))"), Some(-3));
        assert_eq!(part_one(")())())"), Some(-3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), Some(1));
        assert_eq!(part_two("()())"), Some(5));
        assert_eq!(part_two("(((((("), None);
    }
}

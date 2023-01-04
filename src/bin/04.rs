use md5::compute;

pub fn part_one(input: &str) -> Option<u32> {
    let mut number = 0;
    loop {
        let digest = compute(format!("{}{}", input, number));

        if format!("{:x}", digest).starts_with("00000") {
            return Some(number)
        }

        number += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut number = 0;
    loop {
        let digest = compute(format!("{}{}", input, number));

        if format!("{:x}", digest).starts_with("000000") {
            return Some(number)
        }

        number += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}

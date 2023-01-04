pub fn match_pos(dir: &str, pos: (i32, i32)) -> (i32, i32) {
    match dir {
        ">" => (pos.0 + 1, pos.1),
        "<" => (pos.0 - 1, pos.1),
        "^" => (pos.0, pos.1 - 1),
        "v" => (pos.0, pos.1 + 1),
        _ => pos
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    // We start with a vector that already has the starting position visited.
    let mut visited: Vec<(i32, i32)> = vec![(0, 0)];

    let mut pos = (0, 0);
    for direction in input.trim().split("") {
        pos = match_pos(direction, pos);

        if !visited.contains(&pos) {
            visited.push(pos)
        }
    }

    Some(visited.len())
}


pub fn part_two(input: &str) -> Option<usize> {
    // We start with a vector that already has the starting position visited.
    let mut visited: Vec<(i32, i32)> = vec![(0, 0)];

    let mut santa_pos = (0, 0);
    let mut robosanta_pos = (0, 0);
    let binding = input.split("").collect::<Vec<&str>>();
    let chunks = binding.chunks(2);

    for chunk in chunks {
        // The input should always be divisble by 2 since there's always a
        // command for santa -> robosanta.
        if let [santa_dir, robosanta_dir] = chunk {
            // Handle santas position.
            santa_pos = match_pos(santa_dir, santa_pos);
            if !visited.contains(&santa_pos) {
                visited.push(santa_pos)
            }

            // Handle robosantas position.
            robosanta_pos = match_pos(robosanta_dir, robosanta_pos);
            if !visited.contains(&robosanta_pos) {
                visited.push(robosanta_pos)
            }
        }
    }

    Some(visited.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(">"), Some(2));
        assert_eq!(part_one("^>v<"), Some(4));
        assert_eq!(part_one("^v^v^v^v^v"), Some(2));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), Some(3));
        assert_eq!(part_two("^>v<"), Some(3));
        assert_eq!(part_two("^v^v^v^v^v"), Some(11));
    }
}

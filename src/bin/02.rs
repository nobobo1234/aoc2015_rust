pub fn part_one(input: &str) -> Option<i32> {
    let mut total_cost: i32 = 0;
    for present in input.split("\n") {
        // If we encounter an empty line, continue
        if present == "" {
            continue;
        }

        // Otherwise, parse all the values
        let values: Vec<i32> = present.split("x")
            .map(|val| val.parse().unwrap())
            .collect();

        // If we got our expected 3 values, we use them
        if values.len() == 3 {
            let (l, w, h) = (values[0], values[1], values[2]);

            // Calculate all the areas and the total cost of wrapping
            let areas = [l * w, w * h, h * l];
            total_cost += areas.iter().min().unwrap() +
                areas.iter().map(|e| 2 * e).sum::<i32>()
        }
    }
    Some(total_cost)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total_cost: i32 = 0;

    for present in input.split("\n") {
        // If we encounter an empty line, continue
        if present == "" {
            continue;
        }

        // Otherwise, parse all the values
        let mut values: Vec<i32> = present.split("x")
            .map(|val| val.parse().unwrap())
            .collect();

        values.sort();

        if values.len() == 3 {
            let ribbon_length = 2 * values[0] + 2 * values[1];

            total_cost += ribbon_length + values.iter().product::<i32>()
        }
    }

    Some(total_cost)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}

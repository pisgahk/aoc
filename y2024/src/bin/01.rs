const YEAR: u32 = 2024;
const DAY: u8 = 1;

fn part_one(input: &str) -> Option<String> {
    None
}

fn part_two(input: &str) -> Option<String> {
    None
}

fn main() {
    let input = aoc_lib::input::read(YEAR, DAY);

    let t = std::time::Instant::now();
    match part_one(&input) {
        Some(ans) => println!("Part 1: {ans}  ({:.2?})", t.elapsed()),
        None      => println!("Part 1: not yet solved"),
    }

    let t = std::time::Instant::now();
    match part_two(&input) {
        Some(ans) => println!("Part 2: {ans}  ({:.2?})", t.elapsed()),
        None      => println!("Part 2: not yet solved"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example = aoc_lib::input::read_example(YEAR, DAY);
        assert_eq!(part_one(&example), None); // replace with expected answer
    }

    #[test]
    fn test_part_two() {
        let example = aoc_lib::input::read_example(YEAR, DAY);
        assert_eq!(part_two(&example), None);
    }
}

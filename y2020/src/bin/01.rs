use aoc_lib::parsing::lines;

const YEAR: u32 = 2020;
const DAY: u8 = 1;

fn part_one(input: &str) -> Option<i64> {
    None // your solution here
}

fn part_two(input: &str) -> Option<i64> {
    None
}

fn main() {
    let input = aoc_lib::input::read(YEAR, DAY);
    
    let t = std::time::Instant::now();
    println!("Part 1: {:?}  ({:.2?})", part_one(&input), t.elapsed());
    
    let t = std::time::Instant::now();
    println!("Part 2: {:?}  ({:.2?})", part_two(&input), t.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc_lib::input::read_example(YEAR, DAY);
        assert_eq!(part_one(&input), None); // replace None with expected
    }

    #[test]
    fn test_part_two() {
        let input = aoc_lib::input::read_example(YEAR, DAY);
        assert_eq!(part_two(&input), None);
    }
}

const YEAR: u32 = 2024;
const DAY: u8 = 1;

fn part_one(input: &str) -> Option<String> {
    let _test_input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";

    let (mut lhs, mut rhs) = (Vec::new(), Vec::new());

    for line in input.lines(){
        let num = line
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect::<Vec<u32>>();

        lhs.push(num[0]);
        rhs.push(num[1]);
    }

    lhs.sort();
    rhs.sort();

    let summation:u32 = lhs.iter().zip(rhs.iter())
        .map(|(a,b)| a.abs_diff(*b))
        .sum();

    Some(summation.to_string())


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

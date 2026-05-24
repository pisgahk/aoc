pub mod input {
    /// Reads a puzzle input file given year, day.
    /// Expects files at: data/{year}/inputs/{day}.txt
    pub fn read(year: u32, day: u8) -> String {
        let path = format!("data/{}/inputs/{:02}.txt", year, day);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Input file not found: {path}"))
    }

    pub fn read_example(year: u32, day: u8) -> String {
        let path = format!("data/{}/examples/{:02}.txt", year, day);
        std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("Example file not found: {path}"))
    }
}

pub mod grid {
    // you'll fill this in as you need it
    // e.g. Grid<T>, directions, neighbours, BFS helpers
}

pub mod parsing {
    // common parsing helpers
    pub fn lines(input: &str) -> Vec<&str> {
        input.lines().collect()
    }

    pub fn numbers(input: &str) -> Vec<i64> {
        input
            .split_whitespace()
            .filter_map(|w| w.parse().ok())
            .collect()
    }
}

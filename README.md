# How I built this:

Basically it was built by Claude, though here are the steps:

Let's build it out step by step. I'll walk you through the full setup — workspace, shared lib, first year crate, and the tooling to tie it all together.

Download `aoc-cli` from `cargo install aoc-cli`

---

## Step 1 — Create the workspace root

```bash
mkdir -p ~/dev/aoc && cd ~/dev/aoc
git init
```

**`Cargo.toml`** (workspace root — no `[package]` here):

```toml
[workspace]
members = ["aoc-lib"]   # we'll add y20XX as we create them
resolver = "2"

[workspace.dependencies]
# pin shared deps here so all year crates use the same versions
itertools = "0.13"
regex = "1"
```

---

## Step 2 — Create `aoc-lib`

This is your shared utilities crate — grid helpers, parsing, math, etc. Grows over time.

```bash
cargo new --lib aoc-lib
```

**`aoc-lib/Cargo.toml`:**

```toml
[package]
name = "aoc-lib"
version = "0.1.0"
edition = "2021"

[dependencies]
itertools = { workspace = true }
```

**`aoc-lib/src/lib.rs`** — start with just the essentials:

```rust
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
```

---

## Step 3 — Shared data directory

All inputs live here, outside any individual year crate:

```bash
mkdir -p data/{2020,2024}/{inputs,examples,puzzles}
```

Final layout:

```
~/dev/aoc/
├── data/
│   ├── 2020/
│   │   ├── inputs/     ← 01.txt, 02.txt ...
│   │   ├── examples/   ← 01.txt ...
│   │   └── puzzles/    ← 01.md ...
│   └── 2024/
│       └── ...
```

> Data lives at workspace root so both `aoc-lib` and year crates can reference it with a consistent relative path when run via `cargo run` (which sets cwd to workspace root).

---

## Step 4 — Create your first year crate

```bash
cargo new --lib y2020
mkdir -p y2020/src/bin
```

**`y2020/Cargo.toml`:**

```toml
[package]
name = "aoc-y2020"
version = "0.1.0"
edition = "2021"

[dependencies]
aoc-lib = { path = "../aoc-lib" }
itertools = { workspace = true }
regex = { workspace = true }
```

Then add it to the workspace root `Cargo.toml`:

```toml
[workspace]
members = ["aoc-lib", "y2020"]
```

---

## Step 5 — Day file template

Save this as tools/scaffold/template.rs (the file it copies for each new day):

```rust
const YEAR: u32 = {YEAR};
const DAY: u8 = {DAY};

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
```

<!-- **`y2020/src/bin/01.rs`:** -->
<!---->
<!-- ```rust -->
<!-- use aoc_lib::parsing::lines; -->
<!---->
<!-- const YEAR: u32 = 2020; -->
<!-- const DAY: u8 = 1; -->
<!---->
<!-- fn part_one(input: &str) -> Option<i64> { -->
<!--     None // your solution here -->
<!-- } -->
<!---->
<!-- fn part_two(input: &str) -> Option<i64> { -->
<!--     None -->
<!-- } -->
<!---->
<!-- fn main() { -->
<!--     let input = aoc_lib::input::read(YEAR, DAY); -->
<!---->
<!--     let t = std::time::Instant::now(); -->
<!--     println!("Part 1: {:?}  ({:.2?})", part_one(&input), t.elapsed()); -->
<!---->
<!--     let t = std::time::Instant::now(); -->
<!--     println!("Part 2: {:?}  ({:.2?})", part_two(&input), t.elapsed()); -->
<!-- } -->
<!---->
<!-- #[cfg(test)] -->
<!-- mod tests { -->
<!--     use super::*; -->
<!---->
<!--     #[test] -->
<!--     fn test_part_one() { -->
<!--         let input = aoc_lib::input::read_example(YEAR, DAY); -->
<!--         assert_eq!(part_one(&input), None); // replace None with expected -->
<!--     } -->
<!---->
<!--     #[test] -->
<!--     fn test_part_two() { -->
<!--         let input = aoc_lib::input::read_example(YEAR, DAY); -->
<!--         assert_eq!(part_two(&input), None); -->
<!--     } -->
<!-- } -->
<!-- ``` -->

## <!---->

## Step 6 — `.cargo/config.toml` aliases

```bash
mkdir -p .cargo
```

**`.cargo/config.toml`:**

```toml
[alias]
# cargo scaffold 2024 1
scaffold = "run --manifest-path tools/scaffold/Cargo.toml --"

# cargo solve 2024 01
solve = "run --release --bin"

# cargo aoc-test 2024
aoc-test = "test -p"
```

The `solve` alias is a bit limited since it needs `-p` too — the shell function below is cleaner.

---

## Step 7 — Shell functions

Add to your shell config (`~/.bashrc`, `~/.zshrc`, or your functions file):

```bash
# Usage: aoc-solve 2024 1
aoc-solve() {
  local year=$1
  local day=$(printf "%02d" $2)
  cargo run -p "aoc-y$year" --bin "$day" --release
}

# Usage: aoc-download 2024 1
aoc-download() {
  local year=$1
  local day=$2
  local padded=$(printf "%02d" $day)
  local base=~/dev/aoc/data/$year
  mkdir -p "$base"/{inputs,examples,puzzles}

  # remove existing files so aoc-cli can overwrite
  rm -f "$base/inputs/$padded.txt"
  rm -f "$base/puzzles/$padded.md"

  aoc download -y "$year" -d "$day" \
    --input-file  "$base/inputs/$padded.txt" \
    --puzzle-file "$base/puzzles/$padded.md"
  echo "✓ Downloaded $year day $day"
}

# Usage: aoc-test 2024 01
aoc-test() {
  cargo test -p "aoc-y$1" --bin "$2"
}

# Usage: aoc-new-year 2021
aoc-new-year() {
  local year=$1
  local root=~/dev/aoc
  cargo new --lib "$root/y$year"
  mkdir -p "$root/y$year/src/bin"
  mkdir -p "$root/data/$year"/{inputs,examples,puzzles}
  # append to workspace members
  sed -i "s/members = \[/members = [\"y$year\", /" "$root/Cargo.toml"
  echo "✓ Year $year crate created"
}

# Your aoc-scaffold shell function just copies this file and replaces {YEAR} / {DAY} and provides a template where you can inject your approch to solving the problem:

aoc-scaffold() {
  local year=$1
  local day=$2
  local padded=$(printf "%02d" $day)
  local dest=~/dev/aoc/y$year/src/bin/$padded.rs
  local template=~/dev/aoc/tools/scaffold/template.rs

  if [ -f "$dest" ]; then
    echo "✗ $dest already exists"
    return 1
  fi

  sed "s/{YEAR}/$year/g; s/{DAY}/$day/g" "$template" > "$dest"
  echo "✓ Created $dest"
}

aoc-submit() {
	local year=$1
	local day=$2
	local part=$3
	local padded=$(printf "%02d" $day)

	echo "Running solution..."
	local answer=$(cargo run -p "aoc-y$year" --bin "$padded" --release -q 2>/dev/null |
		grep "Part $part:" |
		awk '{print $3}')

	if [ -z "$answer" ]; then
		echo "✗ Could not extract Part $part answer"
		return 1
	fi

	echo "Submitting Part $part: $answer"
	aoc submit --year "$year" --day "$day" "$part" "$answer"
}

```

---

## Final structure

```
~/dev/aoc/
├── Cargo.toml              ← workspace
├── Cargo.lock
├── .cargo/
│   └── config.toml         ← aliases
├── aoc-lib/
│   ├── Cargo.toml
│   └── src/lib.rs          ← shared utilities
├── data/
│   ├── 2020/
│   │   ├── inputs/01.txt
│   │   ├── examples/01.txt
│   │   └── puzzles/01.md
│   └── 2024/
├── y2020/
│   ├── Cargo.toml
│   └── src/bin/
│       ├── 01.rs
│       └── 02.rs
└── y2024/
    └── src/bin/
```

---

## Daily workflow from here

```bash
aoc-download 2024 1     # fetch input + puzzle md
aoc-scaffold 2024 1     # generate y2024/src/bin/01.rs
# ... write logic in part_one / part_two ...
aoc-solve 2024 1        # run it

# Submit part 1
aoc-submit 2024 1 1   # solve and submit part 1

# Then download the updated ../puzzles/01.md
aoc-download 2024 1

# Submit part 2
aoc-submit 2024 1 2   # solve and submit part 2
```

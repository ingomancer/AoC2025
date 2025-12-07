use std::collections::{HashMap, HashSet};

pub fn run(input: String) -> (String, String) {
    let lines = input.lines();

    let mut beam_cols = HashMap::new();
    let mut splitters = vec![];
    let mut part1 = 0;
    let mut part2: u64 = 1;
    for line in lines {
        let mut row_splitters = HashSet::new();
        for (i, char) in line.chars().enumerate() {
            if char == 'S' {
                beam_cols.insert(i, 1);
            } else if char == '^' {
                row_splitters.insert(i);
            }
        }
        if !row_splitters.is_empty() {
            splitters.push(row_splitters);
        }
    }

    for row in splitters {
        let mut next_cols = HashMap::new();
        for (beam, count) in beam_cols {
            if row.contains(&beam) {
                part1 += 1;
                part2 += count;
                next_cols
                    .entry(beam - 1)
                    .and_modify(|b| *b += count)
                    .or_insert(count);
                next_cols
                    .entry(beam + 1)
                    .and_modify(|b| *b += count)
                    .or_insert(count);
            } else {
                next_cols
                    .entry(beam)
                    .and_modify(|b| *b += count)
                    .or_insert(count);
            }
        }
        beam_cols = next_cols;
    }

    (format!("{part1}"), format!("{part2}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("21".to_owned(), "40".to_owned()));
    }
}

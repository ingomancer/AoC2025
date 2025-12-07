use std::collections::HashSet;

pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut grid = HashSet::new();
    for (i, row) in lines.enumerate() {
        for (j, val) in row.chars().enumerate() {
            if val == '@' {
                grid.insert((i as i32, j as i32));
            }
        }
    }

    let mut part1 = vec![];
    for pos in grid.iter() {
        if count_neighbours(pos, &grid) < 4 {
            part1.push(*pos);
        }
    }
    let mut part2 = part1.len();
    let mut to_remove = part1.clone();
    while !to_remove.is_empty() {
        for pos in &to_remove {
            grid.remove(pos);
        }
        to_remove.clear();

        for pos in grid.iter() {
            if count_neighbours(pos, &grid) < 4 {
                to_remove.push(*pos);
            }
        }
        part2 += to_remove.len();
    }

    (format!("{}", part1.len()), format!("{part2}"))
}

fn count_neighbours(pos: &(i32, i32), grid: &HashSet<(i32, i32)>) -> i32 {
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (-1, 0),
    ];
    let mut count = 0;
    for neighbour in directions {
        if grid.contains(&(pos.0 + neighbour.0, pos.1 + neighbour.1)) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("13".to_owned(), "43".to_owned()));
    }
}

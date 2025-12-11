use std::collections::{HashSet, VecDeque};

use microlp::{LinearExpr, Problem};
use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn run(input: String) -> (String, String) {
    let (part1, part2) = input
        .par_lines()
        .map(|line| {
            if line.trim().is_empty() {
                return (0, 0);
            }
            let mut p1 = None;
            let mut splits = line.split_ascii_whitespace().collect::<Vec<&str>>();
            let target = splits
                .remove(0)
                .chars()
                .filter(|x| *x == '.' || *x == '#')
                .map(|x| x == '#')
                .collect::<Vec<bool>>();
            let jolts = splits.pop().unwrap();
            let jolts = jolts[1..jolts.len() - 1]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            let cur_state = vec![false; target.len()];
            let buttons = splits
                .iter()
                .map(|x| {
                    x.chars()
                        .filter(|x| x.is_numeric())
                        .map(|x| x.to_digit(10).unwrap() as usize)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            let mut tried = HashSet::new();
            let mut next = VecDeque::new();

            for button in &buttons {
                let next_state = press_button(&cur_state, button);

                if next_state == target {
                    p1 = Some(1);
                    break;
                }
                tried.insert(next_state.clone());
                next.push_back((next_state, 1));
            }
            while let Some((state, depth)) = next.pop_front() {
                if p1.is_some() {
                    break;
                }
                for button in &buttons {
                    let next_state = press_button(&state, button);

                    if tried.contains(&next_state) {
                        continue;
                    }

                    tried.insert(next_state.clone());

                    if next_state == target {
                        p1 = Some(depth + 1);
                        break;
                    }
                    next.push_back((next_state, depth + 1));
                }
            }

            let mut problem = Problem::new(microlp::OptimizationDirection::Minimize);

            let mut vars = vec![];

            buttons
                .iter()
                .for_each(|_| vars.push(problem.add_integer_var(1.0, (0, i32::MAX))));

            for (index, target) in jolts.iter().enumerate() {
                let mut expr = LinearExpr::empty();
                for (button, val) in buttons.iter().enumerate() {
                    if val.contains(&index) {
                        expr.add(vars[button], 1.0);
                    }
                }
                problem.add_constraint(expr, microlp::ComparisonOp::Eq, *target as f64);
            }

            (
                p1.unwrap(),
                problem.solve().unwrap().objective().round() as u32,
            )
        })
        .reduce(
            || (0, 0),
            |(part1, part2), (p1, p2)| (part1 + p1, part2 + p2),
        );

    (format!("{part1}"), format!("{part2}"))
}

fn press_button(state: &[bool], button: &[usize]) -> Vec<bool> {
    state
        .iter()
        .enumerate()
        .map(|(index, x)| if button.contains(&index) { !x } else { *x })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("7".to_owned(), "33".to_owned()));
    }
}

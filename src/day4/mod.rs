use aoc2025::in_range_inclusive;

pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut lines = lines.peekable();
    let mut ranges = vec![];
    let mut part1 = 0;
    while !lines.peek().unwrap().is_empty() {
        let (start, stop) = lines.next().unwrap().split_once('-').unwrap();
        ranges.push((start.parse::<u64>().unwrap(), stop.parse::<u64>().unwrap()));
    }
    for ingredient in lines {
        if let Ok(ingredient) = ingredient.parse::<u64>() {
            for (start, stop) in &ranges {
                if in_range_inclusive(&ingredient, start, stop) {
                    part1 += 1;
                    break;
                }
            }
        }
    }
    let mut part2 = 0;
    let mut used_ranges = vec![];
    ranges.sort();
    for (start, stop) in ranges {
        let mut new_start = start;
        let mut new_stop = stop;

        for (used_start, used_stop) in &used_ranges {
            if in_range_inclusive(&new_start, used_start, used_stop) {
                new_start = *used_stop + 1;
            }
            if in_range_inclusive(&new_stop, used_start, used_stop) {
                new_stop = *used_start - 1;
            }
        }
        used_ranges.push((start, stop));

        if new_stop >= new_start {
            part2 += new_stop - new_start + 1;
        }
    }
    (format!("{part1}"), format!("{part2}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("3".to_owned(), "14".to_owned()));
    }
}

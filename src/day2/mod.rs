use itertools::Itertools;

pub fn run(input: String) -> (String, String) {
    let mut part1 = 0;
    let mut part2 = 0;
    for (start, stop) in input
        .trim()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
    {
        for num in start.parse::<usize>().unwrap()..=stop.parse().unwrap() {
            let num_string = num.to_string();
            if num_string.len() % 2 == 0 {
                let (left, right) = num_string.split_at(num_string.len() / 2);
                if left == right {
                    part1 += num;
                }
            }
            for n in (1..=num_string.len() / 2).rev() {
                if num_string.as_bytes().chunks(n).all_equal() {
                    part2 += num;
                    break;
                }
            }
        }
    }
    (format!("{part1}"), format!("{part2}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("1227775554".to_owned(), "4174379265".to_owned())
        );
    }
}

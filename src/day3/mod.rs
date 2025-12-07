use itertools::Itertools;

pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let bank = &line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        part1 += bank_joltage(bank, 2);
        part2 += bank_joltage(bank, 12)
    }
    (format!("{part1}"), format!("{part2}"))
}

fn bank_joltage(bank: &[u32], size: usize) -> u64 {
    let mut batts = vec![0; size];
    for nums in bank.windows(size) {
        for n in 0..size {
            if nums[n] > batts[n] {
                batts[n] = nums[n];
                for k in batts.iter_mut().skip(n + 1) {
                    *k = 0;
                }
            }
        }
    }
    batts
        .iter()
        .map(|num| num.to_string())
        .join("")
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
987654321111111
811111111111119
234234234234278
818181911112111
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("357".to_owned(), "3121910778619".to_owned())
        );
    }
}

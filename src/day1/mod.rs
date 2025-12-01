pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut dial = 50;
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let (direction, amount) = line.split_at(1);
        let amount = amount.parse::<isize>().unwrap();

        part2 += amount / 100;
        let amount = amount.rem_euclid(100);
        if direction == "L" {
            dial -= amount;
            if dial < 0 && dial.abs() != amount {
                part2 += 1;
            }
        } else {
            dial += amount;
            if dial > 100 {
                part2 += 1;
            }
        }

        dial = dial.rem_euclid(100);

        if dial == 0 {
            part1 += 1;
        }
    }
    (format!("{part1}"), format!("{}", part1 + part2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("3".to_owned(), "6".to_owned()));
    }
}

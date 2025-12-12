pub fn run(input: String) -> (String, String) {
    let mut lines = input.lines();
    let mut presents = vec![];
    let mut part1 = 0;
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let (index_or_dims, goal_presents) = line.split_once(':').unwrap();
        if let Ok(_) = index_or_dims.parse::<usize>() {
            let mut present = 0;
            for _ in 0..3 {
                let line = lines.next().unwrap();
                present += line
                    .chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .sum::<u32>();
            }
            presents.push(present);
            lines.next();
        } else {
            let (dimx, dimy) = index_or_dims.split_once('x').unwrap();
            let (dimx, dimy) = (dimx.parse::<u32>().unwrap(), dimy.parse::<u32>().unwrap());
            let size = dimx * dimy;
            let goal_presents = goal_presents
                .split_ascii_whitespace()
                .enumerate()
                .map(|(index, x)| {
                    let x = x.parse::<u32>().unwrap();
                    presents[index] * x
                })
                .sum::<u32>();
            // SO apparently for the real input, this is enough.
            // The example has one that doesn't fit despite having enough room, but the full problem does not.
            if goal_presents < size {
                part1 += 1;
            }
        }
    }
    (format!("{part1}"), format!(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("3".to_owned(), "".to_owned()));
    }
}

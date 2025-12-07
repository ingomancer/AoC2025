pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut part1 = 0;
    let mut part2 = 0;
    let mut problems = vec![];
    let mut problems2 = vec![];
    for line in lines {
        for (i, problem) in line.split_whitespace().enumerate() {
            if i == problems.len() {
                let problemlist = vec![problem];
                problems.push(problemlist);
            } else {
                problems[i].push(problem);
            }
        }
        for (i, problem) in line.chars().enumerate() {
            if i == problems2.len() {
                let problemlist = vec![problem];
                problems2.push(problemlist);
            } else {
                problems2[i].push(problem);
            }
        }
    }

    let mut cur_problem = vec![];
    for problem in problems2.iter().rev() {
        let mut num = problem.iter().collect::<String>().trim().to_string();
        match num.parse::<u64>() {
            Ok(num) => cur_problem.push(num),
            Err(_) => {
                if num.trim().is_empty() {
                    continue;
                } else {
                    let op = num.pop().unwrap();
                    cur_problem.push(num.trim().parse().unwrap());
                    part2 += cur_problem
                        .iter()
                        .fold(if op == '+' { 0 } else { 1 }, |acc, x| {
                            if op == '+' { acc + x } else { acc * x }
                        });
                    cur_problem = vec![];
                }
            }
        }
    }

    for mut problem in problems {
        let op = problem.pop().unwrap();
        let mut res = 0;
        for num in problem.iter().map(|x| x.parse::<u64>().unwrap()) {
            if op == "+" {
                res += num;
            } else if res == 0 {
                res = num;
            } else {
                res *= num
            }
        }
        part1 += res;
    }

    (format!("{part1}"), format!("{part2}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
        "};

    #[test]
    fn works() {
        assert_eq!(
            run(INPUT.to_owned()),
            ("4277556".to_owned(), "3263827".to_owned())
        );
    }
}

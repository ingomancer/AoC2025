use std::collections::HashMap;

use memoize::memoize;

pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut connections = HashMap::new();

    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let (source, dests) = line.split_once(": ").unwrap();
        connections.insert(source, vec![]);
        for dest in dests.split_ascii_whitespace() {
            connections.entry(source).and_modify(|val| val.push(dest));
        }
    }

    let part1 = count_paths("you".to_string(), &connections, true, true);

    let part2 = count_paths("svr".to_string(), &connections, false, false);

    (format!("{part1}"), format!("{part2}"))
}

#[memoize(Ignore: connections)]
fn count_paths(cur: String, connections: &HashMap<&str, Vec<&str>>, fft: bool, dac: bool) -> u64 {
    if cur == "out" {
        if fft && dac {
            return 1;
        } else {
            return 0;
        }
    }
    let fft = if cur == "fft" { true } else { fft };
    let dac = if cur == "dac" { true } else { dac };
    connections
        .get(&cur as &str)
        .unwrap()
        .iter()
        .map(|x| count_paths(x.to_string(), connections, fft, dac))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
you: svr
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("8".to_owned(), "2".to_owned()));
    }
}

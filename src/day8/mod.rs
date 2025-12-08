use std::collections::HashMap;

use itertools::Itertools;
use num::integer::sqrt;

#[cfg(test)]
const PAIRS: usize = 10;
#[cfg(not(test))]
const PAIRS: usize = 1000;

pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut distance_pairs = vec![];
    let mut junctions: Vec<Vec<i64>> = vec![];
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let xyz: Vec<i64> = line.splitn(3, ',').map(|x| x.parse().unwrap()).collect();
        for junction in junctions.iter() {
            let distance = distance_euc(&xyz, junction);
            distance_pairs.push((distance, (xyz.clone(), junction.clone())));
        }
        junctions.push(xyz);
    }
    distance_pairs.sort();
    let mut circuits: HashMap<&Vec<i64>, i64> = HashMap::new();
    let mut circuit_id = 0;
    let mut part1 = HashMap::new();
    let mut p2 = 0;
    for (num, (_, (left, right))) in distance_pairs.iter().enumerate() {
        if num == PAIRS {
            for (_, val) in circuits.clone() {
                part1.entry(val).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        if circuits.contains_key(left) {
            let new_id = *circuits.get(left).unwrap();

            if circuits.contains_key(right) {
                let old_id = *circuits.get(right).unwrap();
                for point in circuits.iter_mut() {
                    if *point.1 == old_id {
                        *point.1 = new_id;
                    }
                }
            } else {
                circuits.insert(right, new_id);
            }
        } else if circuits.contains_key(right) {
            let new_id = *circuits.get(right).unwrap();

            circuits.insert(left, new_id);
        } else {
            circuits.insert(left, circuit_id);
            circuits.insert(right, circuit_id);
            circuit_id += 1;
        }
        if num > PAIRS
            && circuits.values().all_equal()
            && junctions
                .iter()
                .map(|junction| circuits.contains_key(junction))
                .all_equal()
        {
            p2 = left[0] * right[0];
            break;
        }
    }

    let mut p1 = 0;
    for _ in 0..3 {
        let val = part1.iter().max_by_key(|x| x.1).unwrap();
        if p1 == 0 {
            p1 = *val.1;
        } else {
            p1 *= val.1;
        }
        let to_remove = *val.0;
        part1.remove(&to_remove);
    }

    (format!("{p1}"), format!("{p2}"))
}

fn distance_euc(p1: &[i64], p2: &[i64]) -> i64 {
    sqrt((p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2) + (p1[2] - p2[2]).pow(2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("40".to_owned(), "25272".to_owned()));
    }
}

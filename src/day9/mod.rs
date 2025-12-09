use geo::{Covers, LineString, Polygon, Rect};

use rayon::iter::ParallelIterator;

use num::abs;
use rayon::iter::IntoParallelRefIterator;

pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    let mut part1 = 0;
    let mut corners: Vec<(i64, i64)> = vec![];
    let mut rects = vec![];
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap());
        for corner in &corners {
            let size = (abs(x - corner.0) + 1) * (abs(y - corner.1) + 1);
            let rect = Rect::new((x as f64, corner.1 as f64), (corner.0 as f64, y as f64));
            rects.push((rect, size));
            if size > part1 {
                part1 = size;
            }
        }
        corners.push((x, y));
    }

    let polygon = Polygon::new(
        LineString::from(
            corners
                .iter()
                .map(|x| (x.0 as f64, x.1 as f64))
                .collect::<Vec<(f64, f64)>>(),
        ),
        vec![],
    );
    let part2 = rects
        .par_iter()
        .map(|(rect, size)| if polygon.covers(rect) { *size } else { 0 })
        .max()
        .unwrap();

    (format!("{part1}"), format!("{part2}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("50".to_owned(), "24".to_owned()));
    }
}

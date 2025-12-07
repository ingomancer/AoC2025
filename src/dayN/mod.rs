pub fn run(input: String) -> (String, String) {
    let lines = input.lines();
    (format!(""), format!(""))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"

        "};

    #[test]
    fn works() {
        assert_eq!(run(INPUT.to_owned()), ("".to_owned(), "".to_owned()));
    }
}

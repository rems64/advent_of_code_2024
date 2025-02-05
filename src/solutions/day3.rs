#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use itertools::Itertools;
    use regex_split::RegexSplit;

    #[test]
    fn day3_part1() {
        let source = include_str!("../resources/day3.txt");

        let s = source
            .split("mul(")
            .filter_map(|s| {
                s.split_once(")").map(|v| {
                    v.0.split(",")
                        .map(|n| n.parse::<u32>())
                        .collect_tuple::<(Result<u32, ParseIntError>, Result<u32, ParseIntError>)>()
                        .map(|c| match c {
                            (Ok(a), Ok(b)) => (a, b),
                            (_, Err(_)) => (0, 0),
                            (Err(_), _) => (0, 0),
                        })
                        .unwrap_or((0, 0))
                })
            })
            .fold(0, |acc: u32, (a, b): (u32, u32)| acc + (a * b));

        dbg!(s);
    }

    #[test]
    fn day3_part2() {
        let source = include_str!("../resources/day3.txt");

        let re = regex::Regex::new(r"mul\(|do\(\)|don't\(\)").unwrap();

        let mut valid = true;

        let s = re.split_inclusive_left(source)
            .filter_map(|s| {
                if s.starts_with("do()") {
                    valid = true;
                    None
                } else if s.starts_with("don't()") {
                    valid = false;
                    None
                } else if s.starts_with("mul(") {
                    if valid {
                        s.strip_prefix("mul(").unwrap_or("").split_once(")").map(|v| {
                            v.0.split(",")
                                .map(|n| n.parse::<u32>())
                                .collect_tuple::<(Result<u32, ParseIntError>, Result<u32, ParseIntError>)>()
                                .map(|c| match &c {
                                    (Ok(a), Ok(b)) => (*a, *b),
                                    (_, Err(_)) => (0, 0),
                                    (Err(_), _) => (0, 0),
                                })
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).flatten()
            .fold(0, |acc: u32, (a, b)| acc + (a * b));

        dbg!(s);
    }
}

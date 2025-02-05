#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    use itertools::Itertools;

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
}

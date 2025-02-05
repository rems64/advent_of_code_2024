#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn day5_part1() {
        let source = include_str!("../resources/day5.txt");

        let rules = source
            .lines()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.split('|')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_tuple::<(u32, u32)>()
                    .unwrap()
            })
            .collect_vec();

        let updates = source
            .lines()
            .skip_while(|l| !l.is_empty())
            .skip(1)
            .map(|l| {
                l.split(',')
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_vec()
            })
            .collect_vec();

        let result = updates
            .iter()
            .map(|update| {
                if update.iter().enumerate().all(|(i, c)| {
                    update.iter().take(i).all(|b| {
                        rules.iter().all(|r| match r {
                            _ if r.0 == *c && r.1 == *b => false,
                            _ => true,
                        })
                    }) && update.iter().skip(i + 1).all(|b| {
                        rules.iter().all(|r| match r {
                            _ if r.0 == *b && r.1 == *c => false,
                            _ => true,
                        })
                    })
                }) {
                    update[(update.len() - 1) / 2]
                } else {
                    0
                }
            })
            .fold(0, |acc, x| acc + x);

        dbg!(result);
    }
}

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

    #[test]
    fn day5_part2() {
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

        let mut updates_to_correct = updates
            .iter()
            .filter_map(|update| {
                if update.iter().enumerate().any(|(i, c)| {
                    update.iter().take(i).any(|b| {
                        rules.iter().any(|r| match r {
                            _ if r.0 == *c && r.1 == *b => true,
                            _ => false,
                        })
                    }) || update.iter().skip(i + 1).any(|b| {
                        rules.iter().any(|r| match r {
                            _ if r.0 == *b && r.1 == *c => true,
                            _ => false,
                        })
                    })
                }) {
                    Some(update.clone())
                } else {
                    None
                }
            })
            .collect_vec();

        for update in updates_to_correct.iter_mut() {
            let n = update.len();
            let mut swapped = true;
            while swapped {
                swapped = false;
                for i in 0..n {
                    for j in i + 1..n {
                        if rules.iter().any(|r| match r {
                            _ if r.0 == update[j] && r.1 == update[i] => true,
                            _ => false,
                        }) {
                            update.swap(i, j);
                            swapped = true;
                        }
                    }
                }
            }
        }

        let result = updates_to_correct
            .iter()
            .map(|update| update[(update.len() - 1) / 2])
            .fold(0, |acc, x| acc + x);

        dbg!(&result);
    }
}

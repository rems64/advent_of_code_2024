#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn day1_part1() {
        let source = include_str!("../resources/day1.txt");
        let (mut first, mut second): (Vec<u32>, Vec<u32>) = source
            .lines()
            .map(|l| {
                l.split(" ")
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_tuple::<(u32, u32)>()
                    .unwrap()
            })
            .unzip();
        first.sort();
        second.sort();
        let s = first
            .iter()
            .zip(second.iter())
            .map(|(a, b)| b.abs_diff(*a))
            .fold(0, |acc, x| acc + x);
        dbg!(s);
    }

    #[test]
    fn day1_part2() {
        let source = include_str!("../resources/day1.txt");
        let (first, second): (Vec<u32>, Vec<u32>) = source
            .lines()
            .map(|l| {
                l.split(" ")
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_tuple::<(u32, u32)>()
                    .unwrap()
            })
            .unzip();
        let s = first
            .iter()
            .map(|a| *a * (second.iter().filter(|b| *a == **b).collect_vec().len() as u32))
            .fold(0, |acc, x| acc + x);
        dbg!(s);
    }
}

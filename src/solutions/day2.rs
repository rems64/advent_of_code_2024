#[cfg(test)]
mod tests {
    use itertools::Itertools;

    enum Growth {
        Up,
        Down,
    }

    #[test]
    fn day2_part1() {
        let source = include_str!("../resources/day2.txt");
        let reports: Vec<Vec<u32>> = source
            .lines()
            .map(|l| {
                l.split(" ")
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_vec()
            })
            .collect_vec();
        let res = reports
            .iter()
            .filter(|report| match report.len() {
                0 => false,
                1 => true,
                _ => {
                    let growth = if report[0] < report[1] {
                        Growth::Up
                    } else if report[0] > report[1] {
                        Growth::Down
                    } else {
                        return false;
                    };
                    report
                        .iter()
                        .zip(report.iter().skip(1))
                        .all(|p| match growth {
                            Growth::Up => (p.0 < p.1) && (p.1 - p.0) <= 3,
                            Growth::Down => (p.0 > p.1) && (p.0 - p.1) <= 3,
                        })
                }
            })
            .collect_vec()
            .len();
        dbg!(res);
    }

    #[test]
    fn day2_part2() {
        let source = include_str!("../resources/day2.txt");
        let reports: Vec<Vec<u32>> = source
            .lines()
            .map(|l| {
                l.split(" ")
                    .filter_map(|n| n.parse::<u32>().ok())
                    .collect_vec()
            })
            .collect_vec();
        let res = reports
            .iter()
            .filter(|report| {
                (0..report.len()).any(|i| {
                    let report_skiped = report
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, v)| *v)
                        .collect_vec();
                    match report_skiped.len() {
                        0 => false,
                        1 => true,
                        _ => {
                            let growth = if report_skiped[0] < report_skiped[1] {
                                Growth::Up
                            } else if report_skiped[0] > report_skiped[1] {
                                Growth::Down
                            } else {
                                return false;
                            };
                            report_skiped
                                .iter()
                                .zip(report_skiped.iter().skip(1))
                                .all(|p| match growth {
                                    Growth::Up => (p.0 < p.1) && (p.1 - p.0) <= 3,
                                    Growth::Down => (p.0 > p.1) && (p.0 - p.1) <= 3,
                                })
                        }
                    }
                })
            })
            .collect_vec()
            .len();
        dbg!(res);
    }
}

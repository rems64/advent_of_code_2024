#[cfg(test)]
mod tests {
    use itertools::Itertools;

    #[test]
    fn day4_part1() {
        let source = include_str!("../resources/day4.txt");

        let v = source
            .lines()
            .map(|c| c.chars().collect_vec())
            .collect_vec();

        let n = v.len();

        let result = (0..n)
            .cartesian_product(0..n)
            .map(|(i, j)| {
                (-1_i32..=1_i32)
                    .cartesian_product(-1_i32..=1_i32)
                    .map(|(di, dj)| {
                        if (di < 0 && i >= 3 || di > 0 && (i < n - 3) || di == 0)
                            && (dj < 0 && j >= 3 || dj > 0 && (j < n - 3) || dj == 0)
                        {
                            match (
                                v[i][j],
                                v[(i as i32 + 1 * di) as usize][(j as i32 + 1 * dj) as usize],
                                v[(i as i32 + 2 * di) as usize][(j as i32 + 2 * dj) as usize],
                                v[(i as i32 + 3 * di) as usize][(j as i32 + 3 * dj) as usize],
                            ) {
                                ('X', 'M', 'A', 'S') => 1,
                                _ => 0,
                            }
                        } else {
                            0
                        }
                    })
                    .fold(0, |acc, x| acc + x)
            })
            .fold(0, |acc, x| acc + x);
        dbg!(result);
    }
}

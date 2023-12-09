pub fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn calculate_derivatives(sequence: Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequences = vec![sequence];
    loop {
        let last = sequences.last().unwrap();

        let derivative: Vec<i64> = last.array_windows().map(|[a, b]| b - a).collect();

        let all_zeroes = derivative.iter().all(|&n| n == 0);
        sequences.push(derivative);

        if all_zeroes {
            break;
        }
    }

    sequences
}

fn solve(input: &str, next_slope: impl Fn(&[i64], i64) -> i64) -> i64 {
    parse(input)
        .into_iter()
        .map(|sequence| {
            let derivatives = calculate_derivatives(sequence);
            derivatives[..derivatives.len() - 1]
                .iter()
                .rfold(0, |slope, function| next_slope(function, slope))
        })
        .sum()
}

pub fn part1(input: &str) -> i64 {
    solve(input, |function, slope| {
        let &prev_value = function.last().unwrap();
        prev_value + slope // calculate next value
    })
}

pub fn part2(input: &str) -> i64 {
    solve(input, |function, slope| {
        let &next_value = function.first().unwrap();
        next_value - slope // calculate previous value
    })
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 114);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 2);
    }
}

pub fn parse(input: &str) -> (&str, &str) {
    let mut lines = input.lines();

    let times = { lines.next().unwrap() }
        .trim_start_matches("Time:")
        .trim_start_matches(' ');

    let records = { lines.next().unwrap() }
        .trim_start_matches("Distance:")
        .trim_start_matches(' ');

    (times, records)
}

fn solve_race(time: f64, record: f64) -> u64 {
    // race distance equation:
    // -x² + bx
    //
    // where x = time spent holding down button
    //   and b = total time of race
    //
    // subtract record time (c) and find roots to calculate possible win scenarios
    // -x² + bx - c

    let min_button_time = (time / 2.0) - ((-time / 2.0).powi(2) - record).sqrt();
    let max_button_time = (time / 2.0) + ((-time / 2.0).powi(2) - record).sqrt();

    let min_button_time = min_button_time.floor() as u64 + 1;
    let max_button_time = max_button_time.ceil() as u64 - 1;

    let possible_ways_to_win = 1 + max_button_time - min_button_time;
    possible_ways_to_win
}

pub fn part1(input: &str) -> u64 {
    let (times, records) = parse(input);

    let times = times.split_whitespace().map(|s| s.parse().unwrap());
    let records = records.split_whitespace().map(|s| s.parse().unwrap());

    times
        .zip(records)
        .map(|(time, record)| solve_race(time, record))
        .product()
}

pub fn part2(input: &str) -> u64 {
    let (time, record) = parse(input);

    let time = time.replace(' ', "").parse().unwrap();
    let record = record.replace(' ', "").parse().unwrap();

    solve_race(time, record)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 288);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 71503);
    }
}

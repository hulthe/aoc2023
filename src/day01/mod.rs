pub fn parse(input: &str) -> impl Iterator<Item = &str> {
    // the best parse function
    input.lines()
}

const DIGIT_NAMES: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Return an iterator of all possible prefixes of a string
///
/// i.e. "abc" will field ["a", "ab", "abc"]
fn prefixes(s: &str) -> impl Iterator<Item = &str> {
    (1..=s.len()).map(|n| &s[..n])
}

/// Return an iterator of all possible suffixes of a string
///
/// i.e. "abc" will field ["c", "bc", "abc"]
fn suffixes(s: &str) -> impl Iterator<Item = &str> {
    (1..=s.len()).map(|n| &s[s.len() - n..])
}

/// Get the first character of a str (as a str)
fn first_char(s: &str) -> &str {
    &s[..1]
}

/// Get the first character of a str (as a str)
fn last_char(s: &str) -> &str {
    &s[s.len() - 1..]
}

pub fn part1(input: &str) -> u64 {
    parse(input)
        .map(|line| {
            let first: u64 = prefixes(line)
                .find_map(|prefix| last_char(prefix).parse().ok())
                .unwrap_or_else(|| panic!("Failed to find first digit in {line:?}"));

            let last: u64 = suffixes(line)
                .find_map(|suffix| first_char(suffix).parse().ok())
                .unwrap_or_else(|| panic!("Failed to find last digit in {line:?}"));

            first * 10 + last
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse(input)
        .map(|line| {
            let first: u64 = prefixes(line)
                .find_map(|prefix| {
                    last_char(prefix).parse().ok().or_else(|| {
                        DIGIT_NAMES
                            .into_iter()
                            .enumerate()
                            .find_map(|(i, name)| prefix.ends_with(name).then_some(i as u64))
                    })
                })
                .unwrap_or_else(|| panic!("Failed to find first digit in {line:?}"));

            let last: u64 = suffixes(line)
                .find_map(|suffix| {
                    first_char(suffix).parse().ok().or_else(|| {
                        DIGIT_NAMES
                            .into_iter()
                            .enumerate()
                            .find_map(|(i, name)| suffix.starts_with(name).then_some(i as u64))
                    })
                })
                .unwrap_or_else(|| panic!("Failed to find last digit in {line:?}"));

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input1");
        assert_eq!(part1(input), 142);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input2");
        assert_eq!(part2(input), 281);
    }
}

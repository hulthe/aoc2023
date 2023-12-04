use crate::util::{parse_u64, HashMap, HashSet};

#[derive(Debug)]
pub struct Card {
    winning_numbers: HashSet<u64>,
    numbers: HashSet<u64>,
}

pub fn parse(input: &str) -> Vec<Card> {
    let input = input.as_bytes();

    // Length of an entire line. All lines have the same length.
    let line_len = 1 + input.iter().position(|&b| b == b'\n').unwrap();

    // length of the "Game x: " prefix
    let prefix_len = 1 + input.iter().position(|&b| b == b':').unwrap();

    input
        .chunks(line_len)
        .map(|line| {
            let line = &line[prefix_len..line_len - 1];
            let mut raw_nums = line.split(|&b| b == b' ').filter(|b| b != b"");

            let mut winning_numbers: HashSet<u64> = Default::default();
            let mut numbers: HashSet<u64> = Default::default();

            while let Some(n) = raw_nums.next() {
                if n == b"|" {
                    break;
                }
                winning_numbers.insert(parse_u64(n).unwrap());
            }

            while let Some(n) = raw_nums.next() {
                numbers.insert(parse_u64(n).unwrap());
            }

            Card {
                winning_numbers,
                numbers,
            }
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let cards = parse(input);

    cards
        .into_iter()
        .map(|card| {
            let winning_count = card.numbers.intersection(&card.winning_numbers).count() as u64;
            // 0 1 2 4 8
            (1 << winning_count) >> 1
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let cards = parse(input);

    // map from card index to copies count
    let mut extra_copies: HashMap<usize, usize> = Default::default();

    // total number of scratchcards we've got
    let mut total_cards = 0;

    for (i, card) in cards.into_iter().enumerate() {
        let winning_count = card.numbers.intersection(&card.winning_numbers).count();

        let copies_of_this_card = *extra_copies.entry(i).or_default() + 1;
        total_cards += copies_of_this_card;
        for j in ((i + 1)..).take(winning_count) {
            *extra_copies.entry(j).or_default() += copies_of_this_card;
        }
    }

    total_cards
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 13);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 30);
    }
}

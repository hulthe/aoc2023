#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.kind(), &self.cards).cmp(&(other.kind(), &other.cards))
    }
}

impl Hand {
    /// Get the [HandKind] for this hand, but assume there are no jokers in hand.
    fn kind_without_joker(&self) -> HandKind {
        let mut sorted = self.cards;
        sorted.sort();

        let (deduped, rest) = sorted.partition_dedup();
        rest.sort();
        let (deduped2, _) = rest.partition_dedup();

        match deduped.len() {
            1 => HandKind::FiveOfAKind,
            2 if deduped2.len() == 1 => HandKind::FourOfAKind,
            2 if deduped2.len() == 2 => HandKind::FullHouse,
            3 if deduped2.len() == 1 => HandKind::ThreeOfAKind,
            3 => HandKind::TwoPair,
            4 => HandKind::OnePair,
            5 => HandKind::HighCard,
            _ => unreachable!("i think..."),
        }
    }

    /// Get the [HandKind] for this hand. This method takes jokers into account.
    fn kind(&self) -> HandKind {
        if self.cards.iter().all(|c| c == &Card::Joker) {
            return HandKind::FiveOfAKind;
        }

        if self.cards.iter().all(|c| c != &Card::Joker) {
            return self.kind_without_joker();
        }

        let mut tmp_hand = *self;
        let (unique_cards, _) = tmp_hand.cards.partition_dedup();

        // For each unique kind of card in the hand, create a hand with all jokers turned into that
        // kind. Then compare them all to find the best hand.
        unique_cards
            .iter()
            .filter(|&kind| kind != &Card::Joker)
            .map(|&kind| {
                let mut new_hand = *self;
                new_hand
                    .cards
                    .iter_mut()
                    .filter(|c| c == &&Card::Joker)
                    .for_each(|c| *c = kind);
                new_hand.kind_without_joker()
            })
            .max()
            .unwrap()
    }
}

pub fn parse(input: &str) -> Vec<(Hand, u64)> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            if hand.len() != 5 {
                panic!("invalid hand len");
            }

            let mut cards = [Card::Ace; 5];
            hand.chars()
                .map(|c| match c {
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'J' => Card::Jack,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!("invalid card: {c:?}"),
                })
                .enumerate()
                .for_each(|(i, c)| cards[i] = c);

            (Hand { cards }, bid.parse().unwrap())
        })
        .collect()
}

fn sort_and_sum(mut plays: Vec<(Hand, u64)>) -> u64 {
    plays.sort_unstable_by_key(|(hand, _)| *hand);
    plays
        .iter()
        //.map(|hand| dbg!(hand))
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid)
        .sum()
}

pub fn part1(input: &str) -> u64 {
    sort_and_sum(parse(input))
}

pub fn part2(input: &str) -> u64 {
    let mut plays = parse(input);

    // replace jacks with jokers
    plays
        .iter_mut()
        .flat_map(|(hand, _)| hand.cards.iter_mut())
        .filter(|c| *c == &Card::Jack)
        .for_each(|c| *c = Card::Joker);

    sort_and_sum(plays)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 6440);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 5905);
    }
}

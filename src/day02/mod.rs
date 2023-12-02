use crate::util::HashMap;
use std::cmp::max;

pub struct Game {
    id: u64,
    revealed: Vec<Vec<ColorCount>>,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Red,
    Green,
    Blue,
}

type ColorCount = (Color, u64);

pub fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let (id, line) = line.split_once(": ").unwrap();
            let id = id.trim_start_matches("Game ").parse().unwrap();
            let revealed = line
                .split("; ")
                .map(|game| {
                    game.split(", ")
                        .map(|c| {
                            let (count, color) = c.split_once(" ").unwrap();
                            let color = match color.as_bytes()[0] {
                                b'r' => Color::Red,
                                b'g' => Color::Green,
                                b'b' => Color::Blue,
                                _ => panic!(),
                            };
                            (color, count.parse().unwrap())
                        })
                        .collect()
                })
                .collect();

            Game { id, revealed }
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let games = parse(input);
    let bag: HashMap<_, u64> = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]
        .into_iter()
        .collect();

    let mut revealed: HashMap<Color, u64> = Default::default();

    games
        .into_iter()
        .filter_map(|game| {
            for r in &game.revealed {
                revealed.clear();
                revealed.extend(r.iter().copied());

                for (&color, &count) in &revealed {
                    let max = bag.get(&color).unwrap_or(&0);
                    if count > *max {
                        return None;
                    }
                }
            }

            Some(game.id)
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let games = parse(input);

    let mut revealed: HashMap<Color, u64> = Default::default();

    games
        .into_iter()
        .map(|game| {
            revealed.clear();
            for r in &game.revealed {
                for &(color, count) in r {
                    let e = revealed.entry(color).or_default();
                    *e = max(*e, count);
                }
            }

            let power: u64 = revealed.iter().map(|(_, &count)| count).product();
            power
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 8);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 2286);
    }
}

use std::{collections::HashMap, iter};

#[derive(Debug)]
pub enum Thingy {
    Symbol(char),
    Number { n: u64, x2: usize },
}

pub fn parse(input: &str) -> HashMap<(usize, usize), Thingy> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut xs = (0..line.len()).peekable();

            iter::from_fn(move || loop {
                let Some(x) = xs.next() else {
                    return None;
                };

                let &c = &line[x..].chars().next().unwrap();

                if c.is_ascii_digit() {
                    let mut end = x;
                    while let Some(&x2) = xs.peek() {
                        let c = &line[x2..].chars().next().unwrap();
                        if c.is_ascii_digit() {
                            xs.next();
                            end = x2;
                        } else {
                            break;
                        }
                    }
                    let n = line[x..=end].parse().unwrap();
                    return Some(((x, y), Thingy::Number { n, x2: end }));
                } else if c != '.' {
                    return Some(((x, y), Thingy::Symbol(c)));
                } else {
                    continue;
                }
            })
        })
        .collect()
}

fn neighbors(x1: usize, x2: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let start_x = x1.saturating_sub(1);
    let end_x = x2 + 1;

    let top = (start_x..=end_x).filter_map(move |x| Some((x, y.checked_sub(1)?)));
    let btm = (start_x..=end_x).map(move |x| (x, y + 1));

    top.chain([(start_x, y), (end_x, y)]).chain(btm)
}

pub fn part1(input: &str) -> u64 {
    let data = parse(input);

    data.iter()
        .filter_map(|(&(x1, y), thingy)| {
            let &Thingy::Number { n, x2 } = thingy else {
                return None;
            };

            for (nx, ny) in neighbors(x1, x2, y) {
                if let Some(Thingy::Symbol(_)) = data.get(&(nx, ny)) {
                    return Some(n);
                };
            }

            return None;
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let data = parse(input);

    // map coordinates of gears to the neighboring numbers
    let mut gear_neighbors: HashMap<(usize, usize), Vec<u64>> = HashMap::default();

    for (&(x1, y), thingy) in data.iter() {
        let &Thingy::Number { n, x2 } = thingy else {
            continue;
        };

        for (nx, ny) in neighbors(x1, x2, y) {
            if let Some(Thingy::Symbol('*')) = data.get(&(nx, ny)) {
                gear_neighbors.entry((nx, ny)).or_default().push(n);
            };
        }
    }

    gear_neighbors
        .values()
        .filter(|n| n.len() >= 2)
        .map(|n| n.iter().product::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 4361);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 467835);
    }
}

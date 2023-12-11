use crate::util::{HashMap, HashSet};

pub type Coord = (i64, i64);

pub fn parse(input: &str) -> HashSet<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|&(_, &c)| c == b'#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect()
}

pub fn part1(input: &str) -> u64 {
    let galaxies = parse(input);
    expand_galaxies(galaxies, 1)
}

pub fn part2(input: &str) -> u64 {
    let galaxies = parse(input);
    expand_galaxies(galaxies, 999999)
}

fn expand_galaxies(mut galaxies: HashSet<Coord>, expansion: i64) -> u64 {
    let min_x = 0;
    let min_y = 0;
    let max_x = galaxies.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = galaxies.iter().map(|&(_, y)| y).max().unwrap();

    // expand universe in x direction
    let mut expand = vec![];
    for column in min_x + 1..max_x {
        if galaxies.iter().all(|&(x, _)| x != column) {
            expand.push(column);
        }
    }
    for column in expand.into_iter().rev() {
        galaxies = galaxies
            .drain()
            .map(|(x, y)| {
                if x > column {
                    (x + expansion, y)
                } else {
                    (x, y)
                }
            })
            .collect();
    }

    // expand universe in y direction
    let mut expand = vec![];
    for row in min_y + 1..max_y {
        if galaxies.iter().all(|&(_, y)| y != row) {
            expand.push(row);
        }
    }
    for row in expand.into_iter().rev() {
        galaxies = galaxies
            .drain()
            .map(|(x, y)| if y > row { (x, y + expansion) } else { (x, y) })
            .collect();
    }

    // calculate distances between pairs
    let mut pair_distances: HashMap<[Coord; 2], u64> = HashMap::default();
    for &c1 @ (x1, y1) in galaxies.iter() {
        for &c2 @ (x2, y2) in galaxies.iter() {
            if c1 == c2 {
                continue;
            }

            let mut pair = [c1, c2];
            pair.sort();

            let distance = x1.abs_diff(x2) + y1.abs_diff(y2);
            pair_distances.insert(pair, distance);
        }
    }

    pair_distances.values().sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 374);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 82000210);
    }
}

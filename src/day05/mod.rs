use std::{
    collections::{btree_map::Entry, BTreeMap},
    iter::{self, once},
    ops::{Bound, Range},
};

pub struct Data<'a> {
    seeds: Vec<i64>,
    mappings: Vec<(&'a str, Mapping)>,
}

#[derive(Debug, Clone, Copy)]
struct Offset(i64);

#[derive(Default, Debug, Clone)]
struct Mapping {
    // Map from range (non-inclusive) upper bounds to offsets.
    ranges: BTreeMap<i64, Offset>,
}

impl Mapping {
    /// Get the mapping offset at a given point.
    fn offset_at(&self, value: i64) -> Offset {
        match self.ranges.lower_bound(Bound::Excluded(&value)).value() {
            Some(&offset) => offset,
            None => Offset(0),
        }
    }

    /// Put a single value through our mapping function.
    fn map(&self, value: i64) -> i64 {
        let Offset(offset) = self.offset_at(value);
        value + offset
    }

    /// Put a range of values through our mapping function.
    fn map_range(&self, range: Range<i64>) -> impl Iterator<Item = Range<i64>> + '_ {
        let start = range.start;
        let end = range.end;

        // get the mapping offset at the start of `range`
        let start_offset = *self
            .ranges
            .lower_bound(Bound::Excluded(&start))
            .value()
            .unwrap_or(&Offset(0));

        // get the mapping offset at the end of `range`
        let end_offset = *self
            .ranges
            .lower_bound(Bound::Included(&end))
            .value()
            .unwrap_or(&Offset(0));

        // get any mapping offset points that appear in the middle of `range`
        let intermediates = self
            .ranges
            .range((Bound::Excluded(start), Bound::Excluded(end)))
            .map(|(&k, &v)| (k, v));

        let mut points = once((start, start_offset))
            .chain(intermediates)
            .chain(once((end, end_offset)))
            .peekable();

        // make a window over all the pairs of offset points we're looking at
        iter::from_fn(move || {
            let (start, _) = points.next()?;

            // the offset for any given value is determined by the offset point that comes after
            // that value in `self.ranges`, so we only need to look at the end offset.
            let &(end, Offset(offset)) = points.peek()?;

            let out = start + offset..end + offset;

            Some(out)
        })
    }

    /// Merge two [Mapping]s into a single [Mapping] that yields the same result as applying the
    /// two [Mapping]s successively.
    #[allow(dead_code)]
    fn merge(self, _other: Mapping) -> Mapping {
        todo!("this could be a cool optimization")
    }
}

pub fn parse(input: &str) -> Data<'_> {
    let mut paragraphs = input.split("\n\n");

    let seeds = paragraphs.next().unwrap().trim_start_matches("seeds: ");
    let seeds = seeds.split(' ').map(|seed| seed.parse().unwrap()).collect();

    let mappings = paragraphs
        .map(|p| {
            let mut lines = p.lines();
            let name = lines.next().unwrap().trim_end_matches(" map:");

            let mut mappings = Mapping::default();
            for line in lines {
                let (dest_start, line) = line.split_once(' ').unwrap();
                let (source_start, len) = line.split_once(' ').unwrap();

                let len: i64 = len.parse().unwrap();
                let dest_start: i64 = dest_start.parse().unwrap();
                let source_start: i64 = source_start.parse().unwrap();
                let source_end = source_start + len;
                let offset = Offset(dest_start - source_start);

                match mappings.ranges.entry(source_end) {
                    Entry::Vacant(slot) => {
                        slot.insert(offset);
                    }
                    Entry::Occupied(mut slot) => match slot.get() {
                        Offset(0) => {
                            slot.insert(offset);
                        }
                        Offset(n) => panic!("overlapping range at {n}"),
                    },
                }

                match mappings.ranges.entry(source_start) {
                    Entry::Occupied(_) => {} // another range ends here, this is fine.
                    Entry::Vacant(slot) => {
                        slot.insert(Offset(0));
                    }
                }

                assert_eq!(
                    2,
                    mappings.ranges.range(source_start..=source_end).count(),
                    "overlapping range at {source_start}..{source_end}"
                )
            }

            (name, mappings)
        })
        .collect();

    Data { seeds, mappings }
}

pub fn part1(input: &str) -> i64 {
    let data = parse(input);
    let mut seeds = data.seeds;

    for (_name, mappings) in &data.mappings {
        seeds = seeds.into_iter().map(|seed| mappings.map(seed)).collect();
    }

    seeds.into_iter().min().expect("no seeds :(")
}

pub fn part2(input: &str) -> i64 {
    let data = parse(input);

    // convert the seeds to ranges of seeds
    let mut seeds: Vec<Range<i64>> = data
        .seeds
        .into_iter()
        .array_chunks()
        .map(|[start, length]| (start..start + length))
        .collect();

    for (_name, mappings) in &data.mappings {
        seeds = seeds
            .into_iter()
            .flat_map(|seed_range| mappings.map_range(seed_range))
            .collect();
    }

    seeds
        .into_iter()
        .map(|seed_range| seed_range.start)
        .min()
        .expect("no seeds :(")
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    //#[test]
    //pub fn test_merge_mappings() {
    //    let input = include_str!("test-input");
    //    let data = parse(input);

    //    let merged_mappings = data
    //        .mappings
    //        .clone()
    //        .into_iter()
    //        .map(|(_, mappings)| mappings)
    //        .reduce(|a, b| a.merge(b))
    //        .expect("no mappings");

    //    for value in 0..=100 {
    //        let mut value_1 = value;
    //        for (_name, mappings) in &data.mappings {
    //            value_1 = mappings.map(value_1);
    //        }

    //        let value_2 = merged_mappings.map(value);
    //        assert_eq!(value_1, value_2, "mappings.merge didn't work");
    //    }

    //    assert_eq!(part1(input), 35);
    //}

    #[test]
    pub fn test_part1() {
        let input = include_str!("test-input");
        assert_eq!(part1(input), 35);
    }

    #[test]
    pub fn test_part2() {
        let input = include_str!("test-input");
        assert_eq!(part2(input), 46);
    }
}

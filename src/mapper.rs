use anyhow;
use std::cmp::Ordering;
use std::num::ParseIntError;

#[derive(Debug)]
struct Interval {
    from_src: u64,
    to_src: u64,
    from_dst: u64,
}

impl Interval {
    fn new(from_src: u64, to_src: u64, from_dst: u64) -> Interval {
        Interval {
            from_src,
            to_src,
            from_dst,
        }
    }
}

#[derive(Debug)]
struct BlockMap {
    intervals: Vec<Interval>,
}

impl BlockMap {
    pub(crate) fn new() -> BlockMap {
        BlockMap {
            intervals: Vec::new(),
        }
    }

    fn add_interval(&mut self, interval: Interval) {
        self.intervals.push(interval);
    }

    fn finalize(self) -> FinalBlockMap {
        FinalBlockMap::new(self.intervals)
    }
}

#[derive(Debug)]
struct FinalBlockMap {
    intervals: Vec<Interval>,
}

impl FinalBlockMap {
    fn new(mut intervals: Vec<Interval>) -> FinalBlockMap {
        intervals.sort_by(|a, b| a.from_src.cmp(&b.from_src));
        FinalBlockMap { intervals }
    }

    fn find_interval(&self, value: u64) -> Option<&Interval> {
        self.intervals
            .binary_search_by(|interval| {
                if interval.to_src < value {
                    Ordering::Less
                } else if interval.from_src > value {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .ok()
            .and_then(|index| self.intervals.get(index))
    }
}

#[derive(Debug)]
pub(crate) struct Mapper {
    maps: Vec<FinalBlockMap>,
}

impl Mapper {
    pub(crate) fn new() -> Mapper {
        Mapper { maps: vec![] }
    }

    pub(crate) fn parse_seeds<T: AsRef<str> + Clone>(seeds: T) -> anyhow::Result<Vec<u64>> {
        let seeds: Result<Vec<u64>, ParseIntError> = seeds
            .as_ref()
            .trim_start_matches("seeds: ")
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>())
            .collect();

        Ok(seeds?)
    }

    pub(crate) fn add_map(&mut self, intervals: &Vec<&str>) -> anyhow::Result<&mut Self> {
        let mut block_map = BlockMap::new();

        for interval in intervals {
            let iter = interval.split_ascii_whitespace();
            let numbers = iter
                .map(|n| n.parse())
                .collect::<Result<Vec<u64>, ParseIntError>>()?;

            let delta = numbers[2];
            let tmp = Interval::new(numbers[1], numbers[1] + delta - 1, numbers[0]);

            // contains mappings for one block
            block_map.add_interval(tmp);
        }

        self.maps.push(block_map.finalize());
        Ok(self)
    }

    pub(crate) fn project(&self, mut value: u64) -> u64 {
        for map in self.maps.iter() {
            if let Some(interval) = map.find_interval(value) {
                value = interval.from_dst + (value - interval.from_src);
            }
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut mapper = Mapper::new();

        let intervals = &mut vec!["12 40 4", "1 5 3", "7 10 2"];
        mapper
            .add_map(&intervals)
            .expect("Error adding map");

        let _seeds = Mapper::parse_seeds("seeds: 1 2 6 15 100").expect("Error adding seeds");

        assert_eq!(mapper.maps.len(), 1);

        assert_eq!(mapper.project(100), 100);
        assert_eq!(mapper.project(5), 1);
        assert_eq!(mapper.project(11), 8);
        assert_eq!(mapper.project(0), 0);
    }
}

use std::cmp::Ordering;
use std::ops::Range;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use aoc::collect_statements;
use aoc::time_it;
use itertools::Itertools;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let packet_pairs = collect_statements::<PacketPair>("input/day13.txt")?;
    Ok(packet_pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, pair)| match pair.compare() {
            Ordering::Greater => None,
            Ordering::Less => Some(i + 1),
            Ordering::Equal => panic!("Did not expect equal!"),
        })
        .sum())
}

#[derive(Debug)]
struct PacketPair {
    left: PacketData,
    right: PacketData,
}

impl PacketPair {
    fn compare(&self) -> Ordering {
        self.left.cmp(&self.right)
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum PacketData {
    Value(usize),
    Array(Array),
}

type Array = Vec<PacketData>;

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::Value(left), PacketData::Value(right)) => left.cmp(&right),
            (PacketData::Array(left), PacketData::Array(right)) => left
                .iter()
                .zip(right.iter())
                .find_map(|(left, right)| match left.cmp(&right) {
                    Ordering::Equal => None,
                    x => Some(x),
                })
                .unwrap_or_else(|| left.len().cmp(&right.len())),
            (PacketData::Value(left), right) => {
                PacketData::Array(vec![PacketData::Value(*left)]).cmp(&right)
            }
            (left, right) => right.cmp(&left).reverse(),
        }
    }
}

impl FromStr for PacketData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_char = s.chars().next().ok_or(anyhow!("Empty String!!!"))?;

        if first_char != '[' {
            Ok(PacketData::Value(s.parse()?))
        } else {
            Ok(PacketData::Array(
                split_array(s)?
                    .into_iter()
                    .map(|element| element.parse().unwrap())
                    .collect(),
            ))
        }
    }
}

fn split_array(s: &str) -> Result<Vec<&str>> {
    Ok(get_array_indices(s)?
        .into_iter()
        .map(|range| &s[range])
        .collect())
}

fn get_array_indices(s: &str) -> Result<Vec<Range<usize>>> {
    let mut chars = s.chars();
    if chars.next() != Some('[') {
        Err(anyhow!("Packet data does not start with an open bracket!!"))?;
    } else if s == "[]" {
        return Ok(Vec::new());
    }

    let mut open: usize = 0;
    let mut start: usize = 1;

    Ok(chars
        .enumerate()
        .filter_map(|(idx, char)| {
            if (open, char) == (0, ',') {
                let rslt = Some(start..idx + 1);
                start = idx + 2;
                return rslt;
            } else if (open, char) == (0, ']') {
                return Some(start..idx + 1);
            } else if char == '[' {
                open += 1;
            } else if char == ']' {
                open -= 1;
            }
            return None;
        })
        .collect())
}

impl FromStr for PacketPair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.lines().collect_tuple() {
            Ok(PacketPair {
                left: left.parse()?,
                right: right.parse()?,
            })
        } else {
            Err(anyhow!("Poo!"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("[1,1,3,1,1]", vec![1..2, 3..4, 5..6, 7..8, 9..10])]
    #[case("[[1],[2,3,4]]", vec![1..4, 5..12])]
    #[case("[[1],4]", vec![1..4, 5..6])]
    #[case("[9]", vec![1..2])]
    #[case("[]", vec![])]
    #[case("[[8,7,6]]", vec![1..8])]
    #[case("[[[]]]", vec![1..5])]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]", vec![1..2, 3..22, 23..24, 25..26])]
    fn get_array_indices_test(#[case] input: &str, #[case] expected: Vec<Range<usize>>) {
        assert_eq!(get_array_indices(input).unwrap(), expected);
    }

    #[rstest]
    #[case("[1,1,3,1,1]", vec!["1", "1", "3", "1", "1"])]
    #[case("[[1],[2,3,4]]", vec!["[1]", "[2,3,4]"])]
    #[case("[[1],4]", vec!["[1]", "4"])]
    #[case("[9]", vec!["9"])]
    #[case("[]", vec![])]
    #[case("[[8,7,6]]", vec!["[8,7,6]"])]
    #[case("[[[]]]", vec!["[[]]"])]
    #[case("[1,[2,[3,[4,[5,6,7]]]],8,9]", vec!["1", "[2,[3,[4,[5,6,7]]]]", "8", "9"])]
    fn split_array_test(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert_eq!(split_array(input).unwrap(), expected);
    }

    #[rstest]
    #[case("[1,1,3,1,1]\n[1,1,5,1,1]", Ordering::Less)]
    #[case("[[1],[2,3,4]]\n[[1],4]", Ordering::Less)]
    #[case("[9]\n[[8,7,6]]", Ordering::Greater)]
    #[case("[[4,4],4,4]\n[[4,4],4,4,4]", Ordering::Less)]
    #[case("[7,7,7,7]\n[7,7,7]", Ordering::Greater)]
    #[case("[]\n[3]", Ordering::Less)]
    #[case("[[[]]]\n[[]]", Ordering::Greater)]
    #[case(
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]",
        Ordering::Greater
    )]
    fn pair_compare_test(#[case] input: &str, #[case] expected: Ordering) {
        assert_eq!(PacketPair::from_str(input).unwrap().compare(), expected);
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 6086);
    }
}

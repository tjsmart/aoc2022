use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use itertools::Itertools;

use aoc::collect_statements;
use aoc::time_it;

const NUM_ROUNDS: usize = 20;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let mut monkeys = collect_statements::<Monkey>("input/day11.txt")?;

    for _ in 0..NUM_ROUNDS {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                let item = monkeys[idx].inspect(item);
                let dst = monkeys[idx].get_receiver(item);
                monkeys[dst].items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspect_count.cmp(&m1.inspect_count));

    Ok(monkeys[0].inspect_count * monkeys[1].inspect_count)
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    dividend: usize,
    receivers: (usize, usize),
    inspect_count: usize,
}

impl Monkey {
    fn inspect(&mut self, item: usize) -> usize {
        self.inspect_count += 1;
        self.operation.call(item) / 3
    }

    fn get_receiver(&self, item: usize) -> usize {
        let (_, r) = item.divmod(self.dividend);
        if r == 0 {
            self.receivers.0
        } else {
            self.receivers.1
        }
    }
}

trait Divmod {
    fn divmod(&self, dividend: usize) -> (usize, usize);
}

impl Divmod for usize {
    fn divmod(&self, dividend: usize) -> (usize, usize) {
        let q = self / dividend;
        let r = self - (q * dividend);
        (q, r)
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);

        let items = lines
            .next()
            .unwrap()
            .split(":")
            .last()
            .unwrap()
            .split(",")
            .map(|item| item.trim().parse().unwrap())
            .collect();
        let operation = lines.next().unwrap().parse()?;
        let dividend = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let receivers = lines
            .take(2)
            .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Monkey {
            items,
            operation,
            dividend,
            receivers,
            inspect_count: 0,
        })
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Mult,
    Pow,
}

#[derive(Debug, PartialEq)]
struct Operation {
    operator: Operator,
    operand: usize,
}

impl Operation {
    fn call(&self, arg: usize) -> usize {
        match self.operator {
            Operator::Add => arg + self.operand,
            Operator::Mult => arg * self.operand,
            Operator::Pow => arg.pow(self.operand.try_into().unwrap()),
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, operator, rhs) = s
            .split_once('=')
            .unwrap()
            .1
            .split_whitespace()
            .collect_tuple()
            .unwrap();

        match (lhs, operator, rhs) {
            ("old", "+", "old") => Ok(Operation {
                operator: Operator::Mult,
                operand: 2,
            }),
            ("old", "*", "old") => Ok(Operation {
                operator: Operator::Pow,
                operand: 2,
            }),
            ("old", "+", x) | (x, "+", "old") => {
                let x = x.parse()?;
                Ok(Operation {
                    operator: Operator::Add,
                    operand: x,
                })
            }
            ("old", "*", x) | (x, "*", "old") => {
                let x = x.parse()?;
                Ok(Operation {
                    operator: Operator::Mult,
                    operand: x,
                })
            }
            _ => Err(anyhow!("Unrecognized operation: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("  Operation: new = old + 6", Operation{operator: Operator::Add, operand: 6})]
    #[case("  Operation: new = 31 + old", Operation{operator: Operator::Add, operand: 31})]
    #[case("  Operation: new = old * 19", Operation{operator: Operator::Mult, operand: 19})]
    #[case("  Operation: new = 23 * old", Operation{operator: Operator::Mult, operand: 23})]
    #[case("  Operation: new = old + old", Operation{operator: Operator::Mult, operand: 2})]
    #[case("  Operation: new = old * old", Operation{operator: Operator::Pow, operand: 2})]
    fn operation_from_str(#[case] input: &str, #[case] expected: Operation) {
        assert_eq!(input.parse::<Operation>().unwrap(), expected);
    }

    #[rstest]
    #[case("  Operation: new = old + 6", 9)]
    #[case("  Operation: new = 31 + old", 34)]
    #[case("  Operation: new = old * 19", 57)]
    #[case("  Operation: new = 23 * old", 69)]
    #[case("  Operation: new = old + old", 6)]
    #[case("  Operation: new = old * old", 9)]
    fn operation_call(#[case] input: &str, #[case] expected: usize) {
        let operation = input.parse::<Operation>().unwrap();
        assert_eq!(operation.call(3), expected);
    }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 54054);
    }
}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

use aoc::collect_lines;
use aoc::time_it;

fn main() -> Result<()> {
    time_it(|| solution())?;
    Ok(())
}

fn solution() -> Result<usize> {
    let readings = collect_lines::<Reading>("input/day16.txt")?;
    let start = readings
        .iter()
        .position(|reading| reading.valve == "AA")
        .unwrap();

    let hiker: Hiker = readings.into();

    Ok(hiker.hike(start, 30))
}

#[derive(Debug, Clone)]
struct Node {
    loc: usize,
    elapsed: usize,
    steam_released: usize,
    visited: Vec<bool>,
    rate: usize,
}

impl Node {
    fn new(num_valves: usize, loc: usize, rate: usize) -> Self {
        let mut visited = vec![false; num_valves];
        visited[0] = true;

        Node {
            loc,
            elapsed: 0,
            steam_released: 0,
            rate,
            visited,
        }
    }

    fn spawn(&self, dest: usize, time: usize, rate: usize) -> Self {
        let mut new = self.clone();
        new.visited[dest] = true;
        new.loc = dest;
        new.elapsed += time;
        new.steam_released += time * self.rate;
        new.rate += rate;
        new
    }

    fn complete(&mut self, limit: usize) {
        let time = limit - self.elapsed;
        self.elapsed += time;
        self.steam_released += time * self.rate;
    }
}

#[derive(Debug)]
struct Hiker {
    valves: Vec<Valve>,
    costs: Vec<Vec<usize>>,
}

impl Hiker {
    fn new(valves: Vec<Valve>, costs: Vec<Vec<usize>>) -> Self {
        Self { valves, costs }
    }

    fn hike(&self, start: usize, limit: usize) -> usize {
        let mut nodes = VecDeque::from([Node::new(self.valves.len(), start, self.valves[0].rate)]);
        let mut max_steam_released = usize::min_value();

        while let Some(mut node) = nodes.pop_front() {
            let costs = &self.costs[node.loc];
            let start_node_len = nodes.len();

            // println!("Popped node: {:?}", node);

            costs
                .iter()
                .zip(&node.visited)
                .enumerate()
                .filter_map(|(dest, (cost, visited))| {
                    (!visited && cost + node.elapsed < limit && self.valves[dest].rate != 0)
                        .then_some(node.spawn(dest, *cost, self.valves[dest].rate))
                })
                // .inspect(|spawn| println!("    spawn: {:?}", spawn))
                .for_each(|spawn| nodes.push_back(spawn));

            if nodes.len() == start_node_len {
                node.complete(limit);
                if node.steam_released > max_steam_released {
                    max_steam_released = node.steam_released
                };
            }
        }
        max_steam_released
    }
}

impl From<Vec<Reading>> for Hiker {
    fn from(readings: Vec<Reading>) -> Self {
        let valves = parse_valves(&readings);
        let costs = parse_costs(&readings);
        Self::new(valves, costs)
    }
}

#[derive(Debug)]
struct Valve {
    rate: usize,
    state: bool,
}

impl Valve {
    fn new(rate: usize) -> Self {
        Self { rate, state: false }
    }
}
fn parse_valves(readings: &[Reading]) -> Vec<Valve> {
    readings
        .iter()
        .map(|reading| Valve::new(reading.rate))
        .collect()
}

fn parse_costs(readings: &[Reading]) -> Vec<Vec<usize>> {
    let valves = readings
        .iter()
        .map(|reading| reading.valve.as_str())
        .collect::<Vec<&str>>();

    let valve_to_leads: HashMap<&str, &Vec<String>> = readings
        .iter()
        .map(|reading| (reading.valve.as_str(), &reading.leads))
        .collect();

    calc_costs(&valves, &valve_to_leads)
}

fn calc_costs(valves: &[&str], tree: &HashMap<&str, &Vec<String>>) -> Vec<Vec<usize>> {
    valves
        .iter()
        .map(|start| {
            valves
                .iter()
                // we add one here because it costs an extra unit of time to open valve
                .map(|end| calc_cost(start, end, tree).unwrap() + 1)
                .collect()
        })
        .collect()
}

fn calc_cost(start: &str, end: &str, tree: &HashMap<&str, &Vec<String>>) -> Option<usize> {
    if start == end {
        return Some(0);
    }

    let mut visited: HashSet<&str> = HashSet::new();
    let mut nodes: VecDeque<&str> = VecDeque::from([start]);
    (1..).find(|_| {
        // update nodes
        for _ in 0..nodes.len() {
            let node = nodes.pop_front().expect("Nodes are empty");
            visited.insert(node);

            for lead in *tree
                .get(node)
                .expect(format!("Tree doesn't contain node: {}", node).as_str())
            {
                if !visited.contains(lead.as_str()) {
                    nodes.push_back(lead);
                }
            }
        }

        // check if any nodes have reached dest
        nodes.iter().any(|node| node == &end)
    })
}

#[derive(Debug)]
struct Reading {
    valve: String,
    rate: usize,
    leads: Vec<String>,
}

impl FromStr for Reading {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^Valve (\w+) has flow rate=(\d+); (tunnels lead to valves|tunnel leads to valve)? (.*)$"
            )
            .unwrap();
        }

        let cap = RE
            .captures(s)
            .ok_or(anyhow!("String does not match regex: \n{}", s))?;

        Ok(Self {
            valve: cap[1].to_string(),
            rate: cap[2].parse()?,
            leads: cap[4]
                .split(',')
                .map(|lead| lead.trim().to_string())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_calc_cost() {
    //     let tree: HashMap<&str, &Vec<String>> = HashMap::from([
    //         ("a", Vec::from(["b", "c"])),
    //         ("b", Vec::from(["d", "a"])),
    //         ("c", Vec::from(["f", "a"])),
    //         ("d", Vec::from(["e", "b"])),
    //         ("e", Vec::from(["d"])),
    //         ("f", Vec::from(["c"])),
    //     ]);
    //
    //     assert_eq!(calc_cost("a", "b", &tree).unwrap(), 1);
    //     assert_eq!(calc_cost("a", "d", &tree).unwrap(), 2);
    //     assert_eq!(calc_cost("a", "e", &tree).unwrap(), 3);
    //
    //     assert_eq!(
    //         calc_costs(&["a", "b", "c", "d", "e", "f"], &tree),
    //         vec![
    //             vec![0, 1, 1, 2, 3, 2], // a
    //             vec![1, 0, 2, 1, 2, 3], // b
    //             vec![1, 2, 0, 3, 4, 1], // c
    //             vec![2, 1, 3, 0, 1, 4], // d
    //             vec![3, 2, 4, 1, 0, 5], // e
    //             vec![2, 3, 1, 4, 5, 0], // f
    //         ]
    //     );
    // }

    #[test]
    fn sln() {
        assert_eq!(solution().unwrap(), 1584);
    }
}

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VALVE: Regex = Regex::new(
        r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([[A-Z]{2}[, ]?]+)"
    )
    .unwrap();
}

pub fn run(input: &str) {
    use std::time::Instant;
    let mut now = Instant::now();
    println!("Part 1: {} - {:.2?}", part1(input), now.elapsed());
    now = Instant::now();
    println!("Part 2: {} - {:.2?}", part2(input), now.elapsed());
}

fn part1(input: &str) -> usize {
    let (ids, nodes, edges) = parse(input);
    let optimal = optimal_valves(&nodes, &edges, 30);
    solve(ids["AA"], &nodes, &edges, &optimal, [30, 0])
}

fn part2(input: &str) -> usize {
    let (ids, nodes, edges) = parse(input);
    let optimal = optimal_valves(&nodes, &edges, 26);
    solve(ids["AA"], &nodes, &edges, &optimal, [26, 26])
}

fn solve(
    start: usize,
    nodes: &[usize],
    edges: &[Vec<Edge>],
    optimal_valves: &[Vec<(usize, usize, usize)>],
    max_minutes: [usize; 2],
) -> usize {
    let mut states = vec![State {
        opened: 1 << start,
        pressure: 0,
        actors: [
            ActorState {
                pos: start,
                minutes_remaining: max_minutes[0],
            },
            ActorState {
                pos: start,
                minutes_remaining: max_minutes[1],
            },
        ],
    }];
    let mut best: State = states[0].clone();
    while let Some(state) = states.pop() {
        let actor = &state.actors[0];
        for edge in &edges[actor.pos] {
            if actor.minutes_remaining < edge.weight || state.opened >> edge.target_id & 1 == 1 {
                continue;
            }
            let minutes_remaining = actor.minutes_remaining - edge.weight;
            let mut next_state = State {
                opened: state.opened | (1 << edge.target_id),
                pressure: state.pressure + nodes[edge.target_id] * minutes_remaining,
                actors: [
                    ActorState {
                        pos: edge.target_id,
                        minutes_remaining,
                    },
                    state.actors[1].clone(),
                ],
            };
            if next_state.actors[0].minutes_remaining < next_state.actors[1].minutes_remaining {
                next_state.actors.swap(0, 1);
            }
            if best_case_pressure(&next_state, optimal_valves) > best.pressure {
                states.push(next_state);
            }
        }
        if state.pressure > best.pressure {
            best = state;
        }
    }
    best.pressure
}

fn best_case_pressure(state: &State, optimal: &[Vec<(usize, usize, usize)>]) -> usize {
    let mut minutes = [
        state.actors[0].minutes_remaining,
        state.actors[1].minutes_remaining,
    ];
    let mut opened = state.opened;
    let mut best = state.pressure;
    'outer: loop {
        for (i, min_weight, flow) in &optimal[minutes[0]] {
            if opened >> i & 1 == 1 {
                continue;
            }
            minutes[0] -= min_weight;
            best += flow * minutes[0];
            if minutes[0] < minutes[1] {
                minutes.swap(0, 1);
            }
            opened |= 1 << i;
            continue 'outer;
        }
        break;
    }
    best
}

fn optimal_valves(
    nodes: &[usize],
    edges: &[Vec<Edge>],
    max_minutes: usize,
) -> Vec<Vec<(usize, usize, usize)>> {
    (0..=max_minutes)
        .map(|remaining_minutes| {
            let mut candidates = nodes
                .iter()
                .enumerate()
                .flat_map(
                    |(i, flow)| match &edges[i].iter().map(|edge| edge.weight).min() {
                        Some(min_weight) => match remaining_minutes > *min_weight {
                            true => Some((i, *min_weight, *flow)),
                            false => None,
                        },
                        None => None,
                    },
                )
                .collect::<Vec<_>>();
            candidates.sort_by(|a, b| {
                (a.2 * (remaining_minutes - a.1))
                    .cmp(&(b.2 * (remaining_minutes - b.1)))
                    .reverse()
            });
            candidates
        })
        .collect::<Vec<_>>()
}

fn parse(input: &str) -> (HashMap<String, usize>, Vec<usize>, Vec<Vec<Edge>>) {
    let mut ids: HashMap<String, usize> = HashMap::new();
    let mut valves: Vec<(usize, Vec<String>)> = Vec::new();
    for line in input.lines() {
        let captures = VALVE.captures(line).unwrap();
        ids.insert(captures[1].to_owned(), valves.len());
        valves.push((
            captures[2].parse().unwrap(),
            captures[3]
                .split(',')
                .map(|n| n.trim().to_owned())
                .collect(),
        ));
    }
    let num_nodes = valves.len();
    let mut distances = vec![vec![usize::MAX; num_nodes]; num_nodes];
    for (i, (_, neighbours)) in valves.iter().enumerate() {
        distances[i][i] = 0;
        for n in neighbours {
            distances[i][ids[n]] = 1;
        }
    }
    floyd_warshall(&mut distances, num_nodes);
    let edges = (0..num_nodes)
        .map(|from| {
            let mut from_edges = (0..num_nodes)
                .flat_map(|to| {
                    match from != to
                        && (from == ids["AA"] || valves[from].0 > 0)
                        && valves[to].0 > 0
                    {
                        true => Some(Edge {
                            target_id: to,
                            weight: distances[from][to] + 1,
                        }),
                        false => None,
                    }
                })
                .collect::<Vec<_>>();
            from_edges.sort_by(|a, b| valves[a.target_id].0.cmp(&valves[b.target_id].0).reverse());
            from_edges
        })
        .collect::<Vec<_>>();
    let nodes = valves.into_iter().map(|(flow, _)| flow).collect::<Vec<_>>();
    (ids, nodes, edges)
}

fn floyd_warshall(distances: &mut [Vec<usize>], n: usize) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if distances[i][k].saturating_add(distances[k][j]) < distances[i][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    opened: u64,
    pressure: usize,
    actors: [ActorState; 2],
}

#[derive(Debug, Clone)]
struct ActorState {
    pos: usize,
    minutes_remaining: usize,
}

#[derive(Debug)]
struct Edge {
    target_id: usize,
    weight: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
    }

    #[test]
    fn test_part1() {
        assert_eq!(1651, part1(input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1707, part2(input()));
    }
}

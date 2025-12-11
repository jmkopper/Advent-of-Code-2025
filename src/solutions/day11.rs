use std::collections::HashMap;
use std::error::Error;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let graph = parse_graph(input)?;

    println!("Part 1: {}", part1(&graph));
    println!("Part 2: {}", part2(&graph));

    Ok(())
}

type Graph = HashMap<String, Vec<String>>;

fn parse_graph(input: &str) -> Result<Graph, Box<dyn Error>> {
    let mut graph = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        let (node, rest) = line.split_once(':').ok_or("bad line")?;
        let children: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
        graph.insert(node.to_string(), children);
    }

    Ok(graph)
}

fn count_paths(graph: &Graph, start: &str, end: &str, memo: &mut HashMap<String, usize>) -> usize {
    if start == end {
        return 1;
    }

    if let Some(&v) = memo.get(start) {
        return v;
    }

    let total = graph
        .get(start)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|c| count_paths(graph, c, end, memo))
        .sum();

    memo.insert(start.to_string(), total);
    total
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct State {
    node: String,
    seen_fft: bool,
    seen_dac: bool,
}

fn count_paths_both(
    graph: &Graph,
    state: State,
    end: &str,
    memo: &mut HashMap<State, usize>,
) -> usize {
    if let Some(&v) = memo.get(&state) {
        return v;
    }

    let mut seen_fft = state.seen_fft;
    let mut seen_dac = state.seen_dac;

    if state.node == "fft" {
        seen_fft = true;
    } else if state.node == "dac" {
        seen_dac = true;
    }

    if state.node == end {
        if seen_dac && seen_fft {
            memo.insert(state, 1);
            return 1;
        } else {
            memo.insert(state, 0);
            return 0;
        }
    }
    let total = graph
        .get(&state.node)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|c| {
            count_paths_both(
                graph,
                State {
                    node: c.clone(),
                    seen_fft,
                    seen_dac,
                },
                end,
                memo,
            )
        })
        .sum();

    memo.insert(state, total);
    total
}

fn part1(graph: &Graph) -> usize {
    let mut memo: HashMap<_, _> = HashMap::new();
    count_paths(graph, "you", "out", &mut memo)
}

fn part2(graph: &Graph) -> usize {
    let mut memo: HashMap<_, _> = HashMap::new();
    let start = State {
        node: "svr".to_string(),
        seen_fft: false,
        seen_dac: false,
    };
    count_paths_both(graph, start, "out", &mut memo)
}

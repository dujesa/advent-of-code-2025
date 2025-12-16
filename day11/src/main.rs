use std::{collections::{HashMap, VecDeque}, fs};

fn read_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (source, destinations_str) = line.split_once(": ").unwrap();
            let destinations = destinations_str
                .split_whitespace()
                .collect();

            (source, destinations) 
        })
        .collect()
}

const START_1: &str = "you";
const START_2: &str = "svr";
const END: &str = "out";
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let graph_one = read_graph(&input);
    println!("part one - {}", traverse(&graph_one));
    
    let graph_two = read_graph(&input);
    println!("part one - {}", traverse_conditional(
        &graph_two, 
        START_2, 
        END, 
        false, 
        false,
        &mut HashMap::new()
    ));
}

fn traverse(graph: &HashMap<&str, Vec<&str>>) -> i32 {
    let mut paths_count = 0;
    let mut dequeue: VecDeque<&str> = VecDeque::new();

    for start_nodes in graph.get(START_1).unwrap() {
        dequeue.push_back(start_nodes);
    }
    
    while let Some(node) = dequeue.pop_front() {
        if node == END {
            paths_count += 1;
        } else {
            for destination in graph.get(node).unwrap() {
                dequeue.push_back(destination);
            }
        }
    }

    paths_count
}

fn traverse_conditional<'a>(
    graph: &'a HashMap<&str, Vec<&str>>, 
    from: &'a str, 
    to: &str, 
    is_dac_hit: bool, 
    is_fft_hit: bool,
    cache: &mut HashMap<(&'a str, bool, bool), i64>
) -> i64 {
    if from == to && is_dac_hit && is_fft_hit  {
        return 1;
    }

    if let Some(paths_count) = cache.get(&(from, is_dac_hit, is_fft_hit)) {
        return *paths_count
    }

    let paths_count = graph.get(from)
        .map(|destinations| {
            destinations
                .iter()
                .map(|new_from| {
                    let is_dac = is_dac_hit || *new_from == "dac";
                    let is_fft = is_fft_hit || *new_from == "fft";

                    traverse_conditional(graph, new_from, to, is_dac, is_fft, cache)
                })
                .sum()
        })
        .unwrap_or(0);

    cache.insert((from, is_dac_hit, is_fft_hit), paths_count);
    paths_count
}


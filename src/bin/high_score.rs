use std::{
    collections::VecDeque,
    io::{self, BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(io::stdin());
    let (result, _) = solution(reader);
    println!("{result}");
}

pub fn solution<B: BufRead>(mut r: B) -> (String, usize) {
    let lines = read_lines(&mut r, 1);
    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    let n = parts[0].parse::<i32>().unwrap_or(0);
    let m = parts[1].parse::<i32>().unwrap_or(0);

    let lines = read_lines(&mut r, m);

    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n as usize];

    for (from, to, weight) in lines.iter().take(m as usize).map(|s| {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let from = parts[0].parse::<i32>().unwrap_or(0);
        let to = parts[1].parse::<i32>().unwrap_or(0);
        let weight = parts[2].parse::<i64>().unwrap_or(0);
        (from, to, weight)
    }) {
        let from = (from - 1) as usize;
        graph[from].push(Edge {
            vertex: to - 1,
            weight,
        });
    }

    let mut q = VecDeque::from([Edge {
        vertex: 0,
        weight: 0,
    }]);

    let mut visited = vec![i64::MIN; n as usize];
    let mut in_queue = vec![false; n as usize];
    in_queue[0] = true;

    while let Some(node) = q.pop_front() {
        let current = visited[node.vertex as usize];
        in_queue[node.vertex as usize] = false;
        if node.weight < current {
            continue;
        }
        for a in &graph[node.vertex as usize] {
            let next = Edge {
                vertex: a.vertex,
                weight: node.weight + a.weight,
            };

            let next_vertex = next.vertex as usize;
            if visited[next_vertex] < next.weight {
                visited[next_vertex] = next.weight;
                if !in_queue[next_vertex] {
                    in_queue[next_vertex] = true;
                    q.push_back(next);
                }
            }
        }
    }
    let result = visited[visited.len() - 1];
    (result.to_string(), 1)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Edge {
    pub weight: i64,
    pub vertex: i32,
}

fn read_lines<B: BufRead>(src: &mut B, n: i32) -> Vec<String> {
    let mut res = vec![];

    for _ in 0..n {
        let mut input = String::new();

        match src.read_line(&mut input) {
            Ok(_n) => {
                res.push(input);
            }

            Err(error) => println!("error: {}", error),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::{
        ffi::OsStr,
        fs::{self, File},
        path::Path,
    };

    use cses::run_test_suite;

    use super::*;

    #[test]
    fn test_suite() {
        run_test_suite!("/home/egr/Downloads/high_score_tests", solution, true);
    }
}

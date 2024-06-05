use std::{
    io::{self, BufRead, BufReader},
    {cmp::Ordering, collections::BinaryHeap},
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

    let mut q = BinaryHeap::from([Edge {
        vertex: 0,
        weight: 0,
    }]);

    let mut visited = vec![i64::MIN; n as usize];
    let mut len = vec![0; n as usize];

    while let Some(node) = q.pop() {
        let adj = &graph[node.vertex as usize];
        let current = visited[node.vertex as usize];
        if node.weight < current {
            continue;
        }
        for a in adj {
            let next = Edge {
                vertex: a.vertex,
                weight: node.weight + a.weight,
            };

            if next.weight > visited[next.vertex as usize] {
                let next_vertex = next.vertex as usize;
                len[next_vertex] = len[node.vertex as usize] + 1;
                if len[next_vertex] == n {
                    return ("-1".to_string(), 1);
                }
                visited[next_vertex] = next.weight;
                q.push(next);
            }
        }
    }
    visited[0] = 0;
    visited.sort();
    let result = visited[visited.len() - 1];
    (result.to_string(), 1)
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Edge {
    pub weight: i64,
    pub vertex: i32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight
            .cmp(&other.weight)
            .then_with(|| self.vertex.cmp(&other.vertex))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

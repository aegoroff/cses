use std::io::{self, BufRead, BufReader};

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

    let mut reverse_graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
    let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];

    for (from, to) in lines.iter().take(m as usize).map(|s| {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let from = parts[0].parse::<i32>().unwrap_or(0);
        let to = parts[1].parse::<i32>().unwrap_or(0);
        (from, to)
    }) {
        reverse_graph[(to - 1) as usize].push(from - 1);
        graph[(from - 1) as usize].push(to - 1);
    }

    let mut visited: Vec<i32> = vec![-1; n as usize];
    let mut sorted = Vec::with_capacity(n as usize);
    for v in 0..n {
        if visited[v as usize] == -1 {
            dfs(&reverse_graph, &mut visited, &mut sorted, v, 1);
        }
    }
    sorted.reverse();

    let mut visited: Vec<i32> = vec![-1; n as usize];
    let mut new_sorted = Vec::with_capacity(n as usize);
    let mut color = 0;
    for v in sorted {
        if visited[v as usize] == -1 {
            color += 1;
            dfs(&graph, &mut visited, &mut new_sorted, v, color);
        }
    }
    let strings: Vec<String> = visited.iter().map(|c| c.to_string()).collect();
    (format!("{color}\n{}", strings.join(" ")), 2)
}

fn dfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<i32>, sorted: &mut Vec<i32>, v: i32, color: i32) {
    visited[v as usize] = color;
    for u in &graph[v as usize] {
        let to_color = visited[*u as usize];
        if to_color == -1 {
            dfs(graph, visited, sorted, *u, color);
        }
    }
    sorted.push(v);
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

    use super::*;

    use cses::run_test_suite;

    #[test]
    fn test_suite() {
        run_test_suite!(
            "/home/egr/Downloads/planets_and_kingdom_tests",
            solution,
            false
        );
    }
}

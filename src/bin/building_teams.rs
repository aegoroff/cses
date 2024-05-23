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

    let mut graph: Vec<Vec<i32>> = vec![vec![]; n as usize];
    let mut visited: Vec<i32> = vec![-1; n as usize];
    for (a, b) in lines.iter().take(m as usize).map(|s| {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let from = parts[0].parse::<i32>().unwrap_or(0);
        let to = parts[1].parse::<i32>().unwrap_or(0);
        (from, to)
    }) {
        graph[(a - 1) as usize].push(b);
        graph[(b - 1) as usize].push(a);
    }

    let mut success = true;
    for v in 1..=n {
        if visited[(v - 1) as usize] == -1 && !dfs(&graph, &mut visited, v, 0) {
            success = false;
            break;
        }
    }

    if success {
        let strings: Vec<String> = visited.into_iter().map(|c| (c + 1).to_string()).collect();
        (strings.join(" "), 2)
    } else {
        ("IMPOSSIBLE".to_owned(), 1)
    }
}

fn dfs(graph: &[Vec<i32>], visited: &mut [i32], v: i32, color: i32) -> bool {
    let mut stack: Vec<(i32, i32)> = vec![(v, color)];
    while let Some((v, color)) = stack.pop() {
        visited[(v - 1) as usize] = color;
        let adj = &graph[(v - 1) as usize];
        for u in adj {
            let to_color = visited[(*u - 1) as usize];
            if to_color == -1 {
                stack.push((*u, color ^ 1));
            } else if to_color == color {
                return false;
            }
        }
    }
    true
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
        run_test_suite!("/home/egr/Downloads/building_teams_tests", solution, false);
    }
}

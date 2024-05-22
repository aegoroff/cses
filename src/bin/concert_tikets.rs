use std::collections::BTreeMap;
use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());
    let (result, _) = solution(reader);
    println!("{result}");
}

pub fn solution<B: BufRead>(mut r: B) -> (String, usize) {
    let lines = read_lines(&mut r, 1);
    let parts: Vec<&str> = lines[0].split_whitespace().collect();
    let n = parts[0].parse::<usize>().unwrap_or(0);
    let m = parts[1].parse::<usize>().unwrap_or(0);

    let lines = read_lines(&mut r, 2);

    let mut avaiable: BTreeMap<i32, i32> = BTreeMap::new();
    for ticket in lines[0].split_whitespace()
        .take(n)
        .map(|s| s.parse::<i32>().unwrap_or(0))
    {
        avaiable.entry(ticket).and_modify(|c| *c += 1).or_insert(1);
    }

    let mut customers_prices = vec![-1; m];

    for (ci, customer) in lines[1].split_whitespace()
        .take(m)
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .enumerate()
    {
        if avaiable.is_empty() {
            continue;
        }
        let mut r = avaiable.range(..=customer);
        let mut ticket = -1;
        let mut counter = 0;

        if let Some((k, v)) = r.next_back() {
            ticket = *k;
            counter = *v;
            customers_prices[ci] = ticket;
            counter -= 1;
        }

        avaiable.entry(ticket).and_modify(|t| *t -= 1);

        if counter == 0 {
            avaiable.remove(&ticket);
        }
    }

    let result: Vec<_> = customers_prices
        .into_iter()
        .map(|c| c.to_string())
        .collect();
    (result.join("\n"), m)
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

    #[test]
    fn test_suite() {
        let paths = fs::read_dir("/home/egr/Downloads/concert_tests").unwrap();
        let mut files: Vec<_> = paths.flatten().map(|d| d.path()).collect();
        files.sort_by(|x, y| {
            let a = x.file_name().unwrap();
            let b = y.file_name().unwrap();
            let a = Path::new(a)
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let b = Path::new(b)
                .file_stem()
                .and_then(OsStr::to_str)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            match a.cmp(&b) {
                Ordering::Equal => {
                    let a = x.extension().unwrap();
                    let b = y.extension().unwrap();
                    let a = Path::new(a).file_stem().and_then(OsStr::to_str).unwrap();
                    let b = Path::new(b).file_stem().and_then(OsStr::to_str).unwrap();
                    a.cmp(b)
                }
                v => v,
            }
        });

        let mut result = String::new();
        let mut m = 0;
        let mut results = vec![];
        for path in files {
            let file = path.file_name().unwrap();

            let ext = Path::new(file).extension().and_then(OsStr::to_str).unwrap();

            if ext == "in" {
                let f = File::open(&path).unwrap();
                let reader = BufReader::new(f);
                (result, m) = solution(reader);
            }
            if ext == "out" {
                let f = File::open(&path).unwrap();
                let mut reader = BufReader::new(f);
                let lines = read_lines(&mut reader, m as i32);
                let expectation = lines.join("").trim_end().to_string();
                let success = result == expectation;
                println!("test:\t{} SUCCESS: {success}", path.display());
                results.push(success);
            }
        }
        let success_tests = results.iter().filter(|r| **r).count();
        let failed_tests = results.iter().filter(|r| !**r).count();
        println!("Success tests: {success_tests} Failed tests: {failed_tests}");
        assert!(results.iter().all(|r| *r));
    }
}

#[macro_export]
macro_rules! run_test_suite {
    ($suite_path:expr, $method:ident, $details:expr) => {{
        let paths = fs::read_dir($suite_path).unwrap();
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
                (result, m) = $method(reader);
            }
            if ext == "out" {
                let f = File::open(&path).unwrap();
                let mut reader = BufReader::new(f);
                let lines = read_lines(&mut reader, m as i32);
                let expectation = lines.join("").trim_end().to_string();
                let success = result == expectation;
                if $details {
                    println!(
                        "test:\t{} expected:\t{expectation} actual:\t{result}\tSUCCESS: {success}",
                        path.display()
                    );
                } else {
                    println!("test:\t{} SUCCESS: {success}", path.display());
                }
                results.push(success);
            }
        }
        let success_tests = results.iter().filter(|r| **r).count();
        let failed_tests = results.iter().filter(|r| !**r).count();
        println!("Success tests: {success_tests} Failed tests: {failed_tests}");
        assert!(results.iter().all(|r| *r));
    }};
}

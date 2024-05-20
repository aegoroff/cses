#[macro_export]
macro_rules! run_test_suite {
    ($suite_path:expr, $method:ident) => {{
        let paths = fs::read_dir($suite_path).unwrap();
        let mut files: Vec<_> = paths.flatten().map(|d| d.path()).collect();
        files.sort();
        let mut result = String::new();
        let mut results = vec![];
        for path in files {
            let file = path.file_name().unwrap();

            let ext = Path::new(file).extension().and_then(OsStr::to_str).unwrap();

            if ext == "in" {
                let f = File::open(&path).unwrap();
                let reader = BufReader::new(f);
                result = format!("{}", $method(reader));
            }
            if ext == "out" {
                let f = File::open(&path).unwrap();
                let mut reader = BufReader::new(f);
                let lines = read_lines(&mut reader, 1);
                let expectation = lines[0].trim();
                let success = result == expectation;
                println!(
                    "test:\t{} expected:\t{expectation} actual:\t{result}\tSUCCESS: {success}",
                    path.display()
                );
                results.push(success);
            }
        }
        assert!(results.iter().all(|r| *r));
    }};
}

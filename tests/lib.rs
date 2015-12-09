#![feature(io)]

extern crate compiler_test as test;

const TEST_DIR: &'static str = "tests";

#[test]
fn integrated_test_suite() {
    let test_paths = test::find::in_path(TEST_DIR).unwrap().into_iter();

    let mut context = test_paths.fold(test::Context::new(), |c,file| {
        let test = util::parse_test(&file).unwrap();
        c.test(test)
    });

    context.add_search_dir(util::tool_dir());

    let results = context.run();

    for result in results.iter() {
        match result.kind {
            test::TestResultKind::Pass => continue,
            test::TestResultKind::Skip => {
                test::print::result(result);
                panic!("Test does not contain any directives: {}",
                       result.path);
            },
            test::TestResultKind::Fail(..) => {
                test::print::result(result);
                panic!("Test failed: {}", result.path);
            },
        }
    }
}

mod util
{
    use std;
    use test;

    pub fn tool_dir() -> String {
        let current_exec = match std::env::current_exe() {
            Ok(e) => e,
            Err(e) => {
                panic!("failed to get current executable path: {}", e)
            },
        };

        current_exec.parent().map(|p| p.to_str().unwrap().to_owned()).unwrap()
    }

    pub fn parse_test(file_name: &str) -> Result<test::Test,String> {
        use std::io::Read;
        let chars = open_file(file_name).chars().map(|c| c.unwrap());
        test::Test::parse(file_name, chars)
    }

    fn open_file(path: &str) -> std::fs::File {
        use std::error::Error;
        match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("could not open {}: {}",
                             path, e.description()),
        }
    }
}

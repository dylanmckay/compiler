#![feature(io)]

extern crate compiler;
extern crate argparse;
extern crate term;

use argparse::ArgumentParser;
use std::borrow::Borrow;

use compiler::test::{self,TestResult, TestResultKind, Context};

fn main() {
    let mut files: Vec<String> = Vec::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Runs tests");

        ap.refer(&mut files)
            .add_argument("files", argparse::List,
                          r#"Files to test"#);
        ap.parse_args_or_exit();
    }

    if files.is_empty() {
        util::abort("no filenames given")
    }

    let paths = files.iter()
                     .map(|s| s.borrow());

    let test_paths = match test::find::in_paths(paths) {
        Ok(paths) => paths,
        Err(e) => util::abort(format!("could not find files: {}", e)),
    };

    if test_paths.is_empty() {
        print::warning("could not find any tests");
        return;
    }

    let mut context = test_paths.into_iter().fold(Context::new(), |c,file| {
        let test = util::parse_test(&file).unwrap();
        c.test(test)
    });

    match util::tool_dir() {
        Some(dir) => context.add_search_dir(dir),
        None => print::warning("could not find tool directory"),
    }

    let results = context.run();

    for result in results.iter() {
        print_result(result)
    }
}

fn print_result(result: &TestResult) {
    match result.kind {
        TestResultKind::Pass => {
            print::success(format!("PASS :: {}", result.path));
        },
        TestResultKind::Skip => {
            print::line();
            print::warning(format!(
                "SKIP :: {} (test does not contain any directives)",
                     result.path));
            print::line();
        },
        TestResultKind::Fail(ref msg, ref desc) => {
            print::line();

            print::failure(format!("FAIL :: {}", result.path));
            print::text(msg.clone());

            // Only print stderr if there was output
            if !desc.is_empty() {
                print::line();
                print::text("stderr:");
                print::line();
                print::text(desc.clone());
            }

            print::line();
        },
    }
}

mod util
{
    use compiler::test::Test;

    use std::error::Error;
    use std::io::Read;
    use std;

    pub fn tool_dir() -> Option<String> {
        let current_exec = match std::env::current_exe() {
            Ok(e) => e,
            Err(e) => abort(
                format!("failed to get current executable path: {}", e)),
        };

        current_exec.parent().map(|p| p.to_str().unwrap().to_owned())
    }

    pub fn parse_test(file_name: &str) -> Result<Test,String> {
        let chars = open_file(file_name).chars().map(|c| c.unwrap());
        Test::parse(file_name, chars)
    }

    fn open_file(path: &str) -> std::fs::File {
        match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => abort(format!("could not open {}: {}",
                                    path, e.description())),
        }
    }
    pub fn abort<S>(msg: S) -> !
        where S: Into<String> {
        super::print::failure(format!("error: {}", msg.into()));

        std::process::exit(1);
    }
}

pub mod print
{
    use std;
    use term;

    pub fn line() {
        with("\n",
             term::stdout().unwrap(),
             term::color::WHITE);
    }

    pub fn text<S>(msg: S)
        where S: Into<String> {
        with(format!("{}\n", msg.into()),
             term::stdout().unwrap(),
             term::color::WHITE);
    }

    pub fn success<S>(msg: S)
        where S: Into<String> {
        with(format!("{}\n", msg.into()),
             term::stdout().unwrap(),
             term::color::GREEN);
    }

    pub fn warning<S>(msg: S)
        where S: Into<String> {
        with(format!("{}\n", msg.into()),
             term::stdout().unwrap(),
             term::color::YELLOW);
    }

    pub fn failure<S>(msg: S)
        where S: Into<String> {
        with(format!("{}\n", msg.into()),
             term::stderr().unwrap(),
             term::color::RED);
    }

    pub fn with<S,W>(msg: S,
                           mut term: Box<term::Terminal<Output=W>>,
                           color: term::color::Color)
        where S: Into<String>, W: std::io::Write {

        term.fg(color).unwrap();
        write!(term, "{}", msg.into()).unwrap();
    }
}

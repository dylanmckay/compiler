#![feature(io)]


extern crate compiler;
extern crate argparse;
extern crate walkdir;
extern crate term;

use argparse::ArgumentParser;
use walkdir::WalkDir;
use std::process::Command;
use std::borrow::Borrow;

mod tool
{
    use super::test::Test;

    /// A constant.
    #[derive(Copy,Clone,Debug,PartialEq,Eq)]
    pub enum Constant
    {
        /// The path of the test that is being run.
        TestPath,
    }

    impl Constant
    {
        /// Maps a constant name to a constant.
        /// Returns `None` if no mapping exists.
        pub fn lookup(name: &str) -> Option<Constant> {
            match name {
                "file" => Some(Constant::TestPath),
                _ => None,
            }
        }
    }

    /// An argument to a tool.
    #[derive(Clone,Debug,PartialEq,Eq)]
    pub enum Argument
    {
        Normal(String),
        Substitute(Constant),
    }

    impl Argument
    {
        /// Parses an argument to a tool.
        ///
        /// If it is prefixed with `@`, then it will be taken
        /// as a constant substitution, otherwise it will
        /// be read verbatim as a tool argument.
        pub fn parse(string: String) -> Result<Self,String> {
            // check if we are parsing a substitution
            if string.chars().next().unwrap() == '@' {
                let name: String = string.chars().skip(1).collect();

                match Constant::lookup(&name) {
                    Some(constant) => Ok(Argument::Substitute(constant)),
                    None => Err(format!("constant does not exist: {}", name)),
                }
            } else { // it is a plain old argument
                Ok(Argument::Normal(string))
            }
        }

        pub fn resolve(&self, test: &Test) -> String {
            match *self {
                Argument::Normal(ref s) => s.clone(),
                Argument::Substitute(constant) => match constant {
                    Constant::TestPath => test.path.clone(),
                },
            }
        }
    }

    /// A tool invocation.
    #[derive(Clone,Debug,PartialEq,Eq)]
    pub struct Invocation
    {
        pub executable: String,
        pub arguments: Vec<Argument>,
    }

    impl Invocation
    {
        /// Parses a tool invocation.
        ///
        /// It is in the format:
        ///
        /// ``` bash
        /// <tool-name> [arg1] [arg2] ...
        /// ```
        pub fn parse<'a,I>(mut words: I) -> Result<Self,String>
            where I: Iterator<Item=&'a str> {
            let executable = match words.next() {
                Some(exec) => exec,
                None => return Err("no executable specified".into()),
            }.into();

            let mut arguments = Vec::new();

            for arg_str in words {
                let arg = try!(Argument::parse(arg_str.into()));
                arguments.push(arg);
            }

            Ok(Invocation {
                executable: executable,
                arguments: arguments,
            })
        }
    }
}


pub mod test
{
    use super::tool;
    use std;

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub enum Directive
    {
        Run(tool::Invocation),
    }

    impl Directive
    {
        pub fn maybe_parse(string: String) -> Option<Result<Self,String>> {
            let directive_str = string.split_whitespace().next().unwrap();
            let inner_words = string.split_whitespace().skip(1);

            match directive_str {
                // FIXME: better message if we have 'RUN :'
                "RUN:" => {
                    let invocation = match tool::Invocation::parse(inner_words) {
                        Ok(i) => i,
                        Err(e) => return Some(Err(e)),
                    };

                    Some(Ok(Directive::Run(invocation)))
                },
                _ => {
                    if directive_str.ends_with(':') {
                        Some(Err(format!("directive '{}' not known", directive_str)))
                    } else {
                        None
                    }
                },
            }
        }
    }

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub struct Test
    {
        pub path: String,
        pub directives: Vec<Directive>,
    }

    impl Test
    {
        pub fn is_empty(&self) -> bool {
            self.directives.is_empty()
        }
    }

    pub enum TestResultKind
    {
        Pass,
        Fail(String, String),
        Skip,
    }

    pub struct TestResult
    {
        pub path: String,
        pub kind: TestResultKind,
    }

    #[derive(Clone,Debug,PartialEq,Eq)]
    pub struct Context
    {
        pub exec_search_dirs: Vec<String>,
        pub tests: Vec<Test>,
    }

    impl Context
    {
        pub fn new() -> Self {
            Context {
                exec_search_dirs: Vec::new(),
                tests: Vec::new(),
            }
        }

        pub fn test(mut self, test: Test) -> Self {
            self.tests.push(test);
            self
        }

        pub fn add_search_dir(&mut self, dir: String) {
            self.exec_search_dirs.push(dir);
        }

        pub fn find_in_search_dir(&self, path: &str)
            -> Option<String> {
            for dir in self.exec_search_dirs.iter() {
                for entry in std::fs::read_dir(dir).unwrap() {
                    let entry = entry.unwrap();
                    let cur_path = entry.path();

                    if std::fs::metadata(&cur_path).unwrap().is_file() {
                        if cur_path.file_name().unwrap() == path {
                            return Some(cur_path.to_str().unwrap().to_owned());
                        }
                    }

                }
            }
            None
        }

        pub fn executable_path(&self, path: &str) -> String {
            match self.find_in_search_dir(path) {
                Some(p) => p,
                None => path.to_owned(),
            }
        }
    }
}

use test::{Test,Directive,TestResult,TestResultKind,Context};

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

    let paths: Vec<&str> = files.iter()
                                 .map(|s| s.borrow())
                                 .collect();

    if paths.is_empty() {
        util::abort("no filenames given")
    }

    let test_paths = find_tests(&paths);

    if test_paths.is_empty() {
        print::warning("could not find any tests");
        return;
    }

    let mut context = test_paths.into_iter().fold(Context::new(), |c,file| {
        let test = read_test(&file).unwrap();
        c.test(test)
    });

    match util::tool_dir() {
        Some(dir) => context.add_search_dir(dir),
        None => print::warning("could not find tool directory"),
    }

    let results = run_tests(&context);

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

/// Recursively finds tests for the given paths.
fn find_tests(paths: &[&str]) -> Vec<String> {
    paths.iter().flat_map(|path| {
                    find_tests_in_path(path).into_iter()
                })
                .collect()
}

fn find_tests_in_path(path: &str) -> Vec<String> {
    let metadata = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(e) => util::abort(format!("failed to open '{}': {}",
                                      path, e)),
    };

    if metadata.is_dir() {
        find_tests_in_dir(path)
    } else {
        vec![path.to_owned()]
    }
}

fn find_tests_in_dir(path: &str) -> Vec<String> {
    find_files_in_dir(path).into_iter()
                           .filter(|f| f.ends_with(".ir"))
                           .collect()
}

fn find_files_in_dir(path: &str) -> Vec<String> {
    let mut dir_tests = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        // don't go into an infinite loop
        if entry.path().to_str().unwrap() == path {
            continue;
        }

        if entry.metadata().unwrap().is_file() {
            dir_tests.push(entry.path().to_str().unwrap().to_owned());
        }
    }

    dir_tests
}

fn read_test(path: &str) -> Result<Test,String> {
    // TODO: redundant indirection
    util::parse_test(path)
}

fn run_tests(context: &Context) -> Vec<TestResult> {
    context.tests.iter().map(|test| run_test(test, context)).collect()
}

fn run_test(test: &Test, context: &Context) -> TestResult {
    if test.is_empty() {
        println!("SKIP :: {} (it includes no test directives", test.path);
        return TestResult {
            path: test.path.clone(),
            kind: TestResultKind::Skip,
        }
    }

    for directive in test.directives.iter() {
        let kind = run_directive(directive, test, context);

        match kind {
            TestResultKind::Pass => continue,
            TestResultKind::Skip => continue,
            TestResultKind::Fail(msg, desc) => {
                return TestResult {
                    path: test.path.clone(),
                    kind: TestResultKind::Fail(msg, desc),
                }
            },
        }
    }

    TestResult {
        path: test.path.clone(),
        kind: TestResultKind::Pass,
    }
}

fn run_directive(directive: &Directive, test: &Test, context: &Context)
    -> TestResultKind {
    match *directive {
        Directive::Run(ref invocation) => {
            let exe_path = context.executable_path(&invocation.executable);
            let mut cmd = Command::new(&exe_path);

            for arg in invocation.arguments.iter() {
                let arg_str = arg.resolve(test);
                cmd.arg(arg_str);
            }

            let output = match cmd.output() {
                Ok(o) => o,
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => {
                        return TestResultKind::Fail(
                            format!("executable not found: {}",
                                    exe_path), "".to_owned());
                    },
                    _ => {
                        return TestResultKind::Fail(
                            format!("could not execute: '{}', {}",
                                    exe_path, e), "".to_owned());
                    },
                },
            };

            if output.status.success() {
                TestResultKind::Pass
            } else {
                let stderr = String::from_utf8(output.stderr).unwrap();

                TestResultKind::Fail(format!(
                    "{} exited with code {}", exe_path,
                    output.status.code().unwrap()),
                    stderr
                    )
            }
        },
    }
}

mod util
{
    use super::test::{Test,Directive};

    use std::error::Error;
    use std::io::Read;
    use compiler::ir;
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
        use compiler::ir::read::Token;

        let chars = open_file(file_name).chars().map(|c| c.unwrap());

        let tokenizer = ir::read::Tokenizer::new(chars).preserve_comments();
        let tokens = tokenizer.map(|result| result.unwrap());

        let mut directives = Vec::new();

        for token in tokens {
            match token {
                Token::Comment { ref text, .. } => {
                    match maybe_parse_directive(text) {
                        Some(Ok(directive)) => directives.push(directive),
                        Some(Err(e)) => {
                            return Err(format!(
                                "could not parse directive: {}", e)
                            );
                        },
                        None => continue,
                    }
                },
                _ => continue,
            };
        }

        Ok(Test {
            path: file_name.to_owned(),
            directives: directives,
        })
    }

    pub fn maybe_parse_directive(comment_text: &str)
        -> Option<Result<Directive,String>> {
        Directive::maybe_parse(comment_text.trim().to_owned())
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

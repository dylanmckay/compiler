use {Context, Test, Directive, Command, TestResultKind};
use std::process;

use tool;
use std;

pub struct Instance
{
    pub invocation: tool::Invocation,
}

impl Instance
{
    pub fn new(invocation: tool::Invocation) -> Self {
        Instance { invocation: invocation }
    }

    pub fn run(self, test: &Test, context: &Context) -> TestResultKind {
        let exe_path = context.executable_path(&self.invocation.executable);
        let mut cmd = self.build_command(test, context);

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

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).unwrap();

            return TestResultKind::Fail(format!(
                "{} exited with code {}", exe_path,
                output.status.code().unwrap()),
                stderr
            );
        }

        let stdout = String::from_utf8(output.stdout).unwrap();

        let stdout_lines: Vec<_> = stdout.lines().map(|l| l.trim().to_owned()).collect();
        let stdout: String = stdout_lines.join("\n");

        Checker { stdout: stdout }.run(&test)
    }

    pub fn build_command(&self, test: &Test, context: &Context) -> process::Command {
        let exe_path = context.executable_path(&self.invocation.executable);
        let mut cmd = process::Command::new(&exe_path);

        for arg in self.invocation.arguments.iter() {
            let arg_str = arg.resolve(test);
            cmd.arg(arg_str);
        }

        cmd
    }
}

struct Checker
{
    stdout: String,
}

impl Checker
{
    fn run(&mut self, test: &Test) -> TestResultKind {
        for directive in test.directives.iter() {
            match directive.command {
                Command::Run(..) => (),
                Command::Check(ref regex) => {
                    let beginning_line = match self.stdout.lines().next() {
                        Some(l) => l.to_owned(),
                        None => return TestResultKind::fail(
                            format_error(test, directive, "reached end of file", "")
                        ),
                    };

                    let mut found_match = false;
                    let tmp: Vec<_> = self.stdout.lines().map(|l| l.to_owned()).skip_while(|line| {
                        found_match = regex.is_match(&line);
                        !found_match
                    }).skip(1).collect();
                    self.stdout = tmp.join("\n");

                    if !found_match {
                        return TestResultKind::fail(
                            format_error(test,
                                         directive,
                                         &format!("could not find match: '{}'", regex),
                                         &beginning_line,
                            )
                        );
                    }
                },
                Command::CheckNext(ref regex) => {
                    if let Some(ref next_line) = self.stdout.lines().next().map(|l| l.to_owned()) {
                        if regex.is_match(next_line) {
                            let lines: Vec<_> = self.stdout.lines().skip(1).map(|l| l.to_owned()).collect();
                            self.stdout = lines.join("\n");
                        } else {
                            return TestResultKind::fail(
                                format_error(test,
                                             directive,
                                             &format!("could not find match: '{}'", regex),
                                             next_line)
                                );
                        }
                    } else {
                        return TestResultKind::fail(format!("check-next reached the end of file"));
                    }
                },
            }
        }

        TestResultKind::Pass
    }
}

fn format_error(test: &Test,
                directive: &Directive,
                msg: &str,
                next_line: &str) -> String {
    format!("{}:{}: {}\nnext line: '{}'", test.path, directive.line, msg, next_line)
}


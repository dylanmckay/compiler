pub use self::tool::*;
pub use self::test::*;

pub mod tool;
pub mod test;

extern crate compiler_ir as ir;

pub mod run
{
    use super::{Directive, Test, Context, TestResult, TestResultKind};
    use std::process::Command;
    use std;

    pub fn test(test: &Test, context: &Context) -> TestResult {
        if test.is_empty() {
            return TestResult {
                path: test.path.clone(),
                kind: TestResultKind::Skip,
            }
        }

        for directive in test.directives.iter() {
            let kind = self::directive(directive, test, context);

            match kind {
                TestResultKind::Pass => continue,
                TestResultKind::Skip => {
                    return TestResult {
                        path: test.path.clone(),
                        kind: TestResultKind::Pass,
                    }
                },
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

    fn directive(directive: &Directive, test: &Test, context: &Context)
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
}

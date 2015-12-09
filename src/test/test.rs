use tool;
use ir;
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Directive
{
    Run(tool::Invocation),
}

impl Directive
{
    pub fn maybe_parse(string: &str) -> Option<Result<Self,String>> {
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
    pub fn parse<S,I>(name: S, chars: I) -> Result<Self,String>
        where S: Into<String>, I: Iterator<Item=char> {
        use ir::read::Token;

        let tokenizer = ir::read::Tokenizer::new(chars).preserve_comments();
        let tokens = tokenizer.map(|result| result.unwrap());

        let mut directives = Vec::new();

        for token in tokens {
            match token {
                Token::Comment { ref text, .. } => {
                    match Directive::maybe_parse(text) {
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
            path: name.into(),
            directives: directives,
        })
    }

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


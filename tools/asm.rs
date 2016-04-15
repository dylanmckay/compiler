#![feature(io)]

extern crate compiler;
extern crate argparse;

use compiler::{ir,mir};
use std::error::Error;
use std::io::{Read,Write};

use argparse::ArgumentParser;

#[derive(Copy,Clone)]
enum Task
{
    Tokenize,
    Parse,

    Assemble,
}

fn main() {
    let mut files: Vec<String> = Vec::new();

    let mut task = Task::Assemble;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Assembles files");

        ap.refer(&mut files)
            .add_argument("files", argparse::List,
                          r#"Files to process"#);
        ap.refer(&mut task)
            .add_option(&["--tokenize"], argparse::StoreConst(Task::Tokenize),
                        "only tokenize the module")
            .add_option(&["--parse"], argparse::StoreConst(Task::Parse),
                        "only parse the module");
        ap.parse_args_or_exit();
    }

    if files.is_empty() {
        abort("expected a file_name to assemble");
    }

    for file_name in files.iter() {
        match task {
            Task::Tokenize => tokenize(&file_name),
            Task::Parse => parse(&file_name),
            Task::Assemble => assemble(&file_name),
        }
    }
}

fn tokenize(file_name: &str) {
    let chars = open_file(file_name).chars().map(|c| c.unwrap());

    let tokenizer = ir::read::Tokenizer::new(chars).preserve_comments();

    for token in tokenizer {
        println!("{:?}", token);
    }
}

fn assemble(file_name: &str) {
    let module = parse_module(&file_name);

    verify_module(&module);

    let module = parse_module(&file_name);

    for func in module.functions() {
        for block in func.blocks() {
            let dag = mir::Dag::from_block(block);

            println!("{:#?}", dag);
        }
    }
}

fn parse(file_name: &str) {
    let module = parse_module(&file_name);

    print_module(&module);
}

fn open_file(path: &str) -> std::fs::File {
    match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => abort(format!("could not open {}: {}",
                                path, e.description())),
    }
}

fn parse_module(file_name: &str) -> ir::Module {
    let chars = open_file(file_name).chars().map(|c| c.unwrap());

    match ir::read::textual(chars) {
        Ok(module) => module,
        Err(e) => abort(format!("could not parse IR file: {}", e)),
    }
}

fn verify_module(module: &ir::Module) {
    match ir::verifier::verify(module) {
        Ok(..) => (),
        Err(e) => {
            abort(format!("module verification failed: {}", e))
        },
    }
}

fn print_module(module: &ir::Module) {
    println!("{}", ir::printable(module));
}

fn abort<S>(msg: S) -> !
    where S: Into<String> {
    write!(std::io::stderr(),
           "{}\n", msg.into()).unwrap();
    std::process::exit(1);
}

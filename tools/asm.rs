#![feature(io)]

extern crate compiler;
extern crate argparse;

use compiler::ir;
use std::error::Error;
use std::io::{Read,Write};

use argparse::ArgumentParser;

fn main() {
    let mut files: Vec<String> = Vec::new();
    let mut tokenize = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Assembles files");

        ap.refer(&mut files)
            .add_argument("files", argparse::List,
                          r#"Files to process"#);
        ap.refer(&mut tokenize)
            .add_option(&["--tokenize"], argparse::StoreTrue,
                        "Only tokenize the input");
        ap.parse_args_or_exit();
    }

    if files.is_empty() {
        abort("expected a filename to assemble");
    }

    for filename in files.iter() {
        if tokenize {
            tokenize_module(&filename)
        } else {
            assemble_module(&filename)
        }
    }
}

fn tokenize_module(file_name: &str) {
    let chars = open_file(file_name).chars().map(|c| c.unwrap());

    let tokenizer = ir::read::Tokenizer::new(chars).preserve_comments();

    for token in tokenizer {
        println!("{:?}", token);
    }
}

fn assemble_module(file_name: &str) {
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

fn print_module(module: &ir::Module) {
    println!("{}", ir::printable(module));
}

fn abort<S>(msg: S) -> !
    where S: Into<String> {
    write!(std::io::stderr(),
           "{}\n", msg.into()).unwrap();
    std::process::exit(1);
}

#![feature(io)]

extern crate compiler;
extern crate argparse;

use compiler::{ir,target};
use std::error::Error;
use std::io::{Read,Write};

use argparse::ArgumentParser;

fn main() {
    let mut files: Vec<String> = Vec::new();
    let mut tokenize = false;
    let mut do_print_isel = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Assembles files");

        ap.refer(&mut files)
            .add_argument("files", argparse::List,
                          r#"Files to process"#);
        ap.refer(&mut tokenize)
            .add_option(&["--tokenize"], argparse::StoreTrue,
                        "Only tokenize the input");
        ap.refer(&mut do_print_isel)
            .add_option(&["--print-isel"], argparse::StoreTrue,
                        "Print the ISel graph");
        ap.parse_args_or_exit();
    }

    if files.is_empty() {
        abort("expected a filename to assemble");
    }

    for filename in files.iter() {
        if tokenize {
            tokenize_module(&filename)
        } else if do_print_isel {
            print_isel(&filename)
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

    // verify_module(&module);
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

fn print_isel(file_name: &str) {
    let module = parse_module(&file_name);

    for func in module.functions() {
        for block in func.blocks() {
            let dag = target::Dag::from_block(block);

            println!("{:#?}", dag);
        }
    }
}

fn abort<S>(msg: S) -> !
    where S: Into<String> {
    write!(std::io::stderr(),
           "{}\n", msg.into()).unwrap();
    std::process::exit(1);
}

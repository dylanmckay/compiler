#![feature(io)]

extern crate compiler;

use compiler::ir;
use std::error::Error;
use std::io::Read;

fn main() {
    let mut args = std::env::args().skip(1);

    let filename = match args.next() {
        Some(f) => f,
        None => abort("expected a filename to assemble"),
    };

    let module = parse_module(&filename);

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
    println!("{}", module);
}

fn abort<S>(msg: S) -> !
    where S: Into<String> {
    println!("failed: {}", msg.into());
    std::process::exit(1);
}

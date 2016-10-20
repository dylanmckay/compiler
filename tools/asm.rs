#![feature(io)]

extern crate compiler;
extern crate argparse;

use compiler::{ir,machine,target};
use compiler::target::Target;
use std::error::Error;
use std::io::{Read,Write};
use std::fs;

use argparse::ArgumentParser;

#[derive(Copy,Clone)]
enum Task
{
    Parse,
    Assemble,
    ListTargets,
}

impl Task
{
    fn requires_input_files(&self) -> bool {
        match *self {
            Task::Parse => true,
            Task::Assemble => true,
            Task::ListTargets => false,
        }
    }
}

fn main() {
    machine::AVR::register();

    let mut files: Vec<String> = Vec::new();

    let mut task = Task::Assemble;
    let mut target_name = "avr".to_owned();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Assembles files");

        ap.refer(&mut files)
            .add_argument("files", argparse::List,
                          r#"Files to process"#);
        ap.refer(&mut target_name)
            .add_option(&["--target"], argparse::Store,
                        "the target");
        ap.refer(&mut task)
            .add_option(&["--parse"], argparse::StoreConst(Task::Parse),
                        "only parse the module")
            .add_option(&["--list-targets"], argparse::StoreConst(Task::ListTargets),
                        "list all of the targets supported");
        ap.parse_args_or_exit();
    }

    if task.requires_input_files() && files.len() == 0 {
        abort("no files given");
    }

    match task {
        Task::ListTargets => list_targets(),
        Task::Parse => for file_name in files { parse(&file_name) },
        Task::Assemble => {
            if target_name.len() == 0 {
                abort("target not speficied on command line");
            }

            let target = match target::registry::lookup(&target_name) {
                Some(target) => target,
                None => {
                    abort(format!("target '{}' does not exist", target_name));
                },
            };

            for file_name in files.iter() {
                generate(target::OutputType::Assembly, target, &file_name);
            }
        }
    }
}

fn generate(output_type: target::OutputType, target: &Target, file_name: &str) {
    let mut file = fs::File::open(file_name).unwrap();
    let mut output: Vec<u8> = Vec::new();

    target.generate(output_type,
                    &mut file,
                    &mut output).unwrap();

    let asm = String::from_utf8(output).unwrap();
    print!("{}", asm);
}

fn list_targets() {
    for target in target::registry::list() {
        println!("{}", target.name());
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

fn print_module(module: &ir::Module) {
    println!("{}", ir::printable(module));
}

fn abort<S>(msg: S) -> !
    where S: Into<String> {
    write!(std::io::stderr(),
           "{}\n", msg.into()).unwrap();
    std::process::exit(1);
}

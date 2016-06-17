use MachineTarget;
use {ir, mir, target, regalloc};
use target::OutputType;

use std::io;

pub fn generate<T>(target: &T,
                   output_type: OutputType,
                   input: &mut io::Read,
                   output: &mut io::Write) -> Result<(), target::Error>
    where T: MachineTarget {
    match output_type {
        OutputType::Assembly => assemble(target, input, output),
    }
}

fn assemble<T>(target: &T,
               input: &mut io::Read,
               output: &mut io::Write) -> Result<(), target::Error>
    where T: MachineTarget {
    let module = try!(self::parse_ir(input));

    for func in module.functions() {
        let dags = mir::Dag::from_function(func);

        for dag in dags {
            let legalizer = target.create_legalizer();
            let mut selector = target.create_selector();

            let dag = legalizer.legalize(dag);

            let instructions = selector.select(dag);
            let instructions = regalloc::allocate(target, instructions);

            for instruction in instructions {
                let inst_str = format!("{:?}\n", instruction);
                try!(output.write(inst_str.as_bytes()));
            }
        }
    }

    Ok(())
}

fn parse_ir(input: &mut io::Read) -> Result<ir::Module, target::Error> {
    let mut input_module_str = String::new();
    try!(input.read_to_string(&mut input_module_str));

    let module = match ir::read::textual(input_module_str.chars()) {
        Ok(module) => module,
        Err(e) => return Err(target::Error::InvalidIR(e)),
    };

    if let Err(e) = ir::verifier::verify(&module) {
        return Err(target::Error::InvalidIR(e));
    }

    Ok(module)
}


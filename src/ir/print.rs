
use ir;
use util;
use std::fmt;

pub fn module(module: &ir::Module, fmt: &mut fmt::Formatter) -> fmt::Result {
    use lang::Module;

    for global in module.globals() {
        try!(self::global(global, fmt));
    }

    try!(write!(fmt, "\n"));
   
    for func in module.functions() {
         try!(self::function(func, fmt));
    }

    Ok(())
}

pub fn global(global: &ir::Global, fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "%{} = {} {}\n", global.name(), global.ty(), global.value())
}

pub fn function(func: &ir::Function, fmt: &mut fmt::Formatter) -> fmt::Result {
    use lang::Function;

    let mut accum = 1;

    try!(write!(fmt, "define {} @{}({}) {{\n",
                     util::comma_separated_values(func.signature.returns()),
                     func.name(),
                     util::comma_separated_values(func.signature.parameters())));

    for block in func.blocks() {
        try!(self::block(block, fmt, &mut accum));
    }

    write!(fmt, "}}")
}

pub fn block(block: &ir::Block,
             fmt: &mut fmt::Formatter,
             accum: &mut u64) -> fmt::Result {
    use lang::Block;

    try!(write!(fmt, "{}:\n", block.name()));

    for value in block.subvalues() {
        try!(self::value(&value, fmt, accum));
    }

    Ok(())
}

pub fn value(value: &ir::Value,
             fmt: &mut fmt::Formatter,
             accum: &mut u64) -> fmt::Result {
    use lang::Value;
    use ir::ValueTrait;

    // Recursively print all subvalues
    for subvalue in value.subvalues().iter().filter(|a| !a.is_literal())  {
        try!(self::value(subvalue, fmt, accum));
    }

    // Replace subvalues with reference to registers
    let modified = value.clone()
                        .map_subvalues(|a| {

        if a.is_literal() {
            return a;
        }
        let reg_name = ir::Name::named(format!("{}", accum));
        *accum += 1;

        ir::value::Register::new(reg_name, a.ty()).into()
    });

    try!(write!(fmt, "\t"));

    // only print the register if it isn't void typed
    if !modified.ty().is_void() {
        *accum += 1;
        try!(write!(fmt, "%{} = ", accum));
    }

    try!(write!(fmt, "{}\n", modified));


    Ok(())
}

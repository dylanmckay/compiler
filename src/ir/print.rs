
use ir;
use lang;
use util;
use ir::Value;
use std::fmt;


impl fmt::Display for ir::Module
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self::module(self, fmt)
    }
}

pub struct Printer<'a>
{
    module: &'a ir::Module,
    accum: u64,
}

pub fn module(module: &ir::Module, fmt: &mut fmt::Formatter) -> fmt::Result {

    let mut printer = Printer {
        module: module,
        accum: 0,
    };
    
    for global in module.globals() {
        try!(self::global(global, &mut printer, fmt));
    }

    try!(write!(fmt, "\n"));
   
    for func in module.functions() {
         try!(self::function(func, &mut printer, fmt));
    }

    Ok(())
}

pub fn global(global: &ir::Global,
              printer: &mut Printer,
              fmt: &mut fmt::Formatter) -> fmt::Result {
    write!(fmt, "%{} = ", global.name());
    self::root_value(global.value(), printer, fmt)
}

pub fn function(func: &ir::Function,
                printer: &mut Printer,
                fmt: &mut fmt::Formatter) -> fmt::Result {

    let mut accum = 1;

    try!(write!(fmt, "define {} @{}({}) {{\n",
                     util::comma_separated_values(func.signature.returns()),
                     func.name(),
                     util::comma_separated_values(func.signature.parameters())));

    for block in func.blocks() {
        try!(self::block(block, printer, fmt));
    }

    write!(fmt, "}}")
}

pub fn block(block: &ir::Block,
             printer: &mut Printer,
             fmt: &mut fmt::Formatter) -> fmt::Result {

    try!(write!(fmt, "{}:\n", block.name()));

    for value in block.subvalues() {
        try!(self::root_value(&value, printer, fmt));
    }

    Ok(())
}

pub fn root_value(value: &ir::Value,
                  printer: &mut Printer,
                  fmt: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(fmt, "\t"));
    try!(self::value::plain_value(value, printer, fmt));
    write!(fmt, "\n")
}

pub fn name(name: &lang::Name,
            printer: &mut Printer,
            fmt: &mut fmt::Formatter) -> fmt::Result {
    unimplemented!();
}


pub mod value
{
    use ir::{Module,Value,Instruction,value};
    use std::fmt;
    use util;
    use lang;
    use super::Printer;

    pub fn value(value: &Value,
                 printer: &mut Printer,
                 fmt: &mut fmt::Formatter) -> fmt::Result {
        use lang::Value;

        // simple values are not parenthesised.
        if !value.is_simple() {
            try!(write!(fmt, "("));
        }

        try!(self::plain_value(value, printer, fmt));

        if !value.is_simple() {
            try!(write!(fmt, ")"));
        }

        Ok(())
    }

    pub fn plain_value(value: &Value,
                       printer: &mut Printer,
                       fmt: &mut fmt::Formatter) -> fmt::Result {

        match value {
            &Value::Literal(ref val) => self::literal(val, fmt),
            &Value::Pointer(ref val) => self::pointer(val, printer, fmt),
            &Value::Register(ref val) => self::register(val, printer, fmt),
            &Value::Instruction(ref val) => self::instruction::instruction(val, printer, fmt),
            &Value::GlobalRef(ref val) => self::global_ref(val, printer, fmt),
            &Value::BlockRef(ref val) => self::block_ref(val, printer, fmt),
            &Value::FunctionRef(ref val) => self::function_ref(val, printer, fmt),
            &Value::RegisterRef(ref val) => self::register_ref(val, printer, fmt),
        }
    }

    pub fn literal(value: &value::Literal,
                   fmt: &mut fmt::Formatter) -> fmt::Result {
        match value {
            &value::Literal::Integer(ref val) => self::literal_integer(val, fmt),
            _ => unimplemented!(),
        }
    }

    pub fn literal_integer(value: &value::literal::Integer,
                           fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} {}", value.ty(), value.value())
    }

    pub fn literal_struct(value: &value::literal::Struct,
                          printer: &mut Printer,
                          fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
        //write!(fmt, "{{ {} }}", util::comma_separated_values(value.fields()))
    }

    pub fn pointer(value: &value::Pointer,
                   printer: &mut Printer,
                   fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(self::value(value.underlying(), printer, fmt));
        write!(fmt, "*")
    }

    pub fn register(value: &value::Register,
                    printer: &mut Printer,
                    fmt: &mut fmt::Formatter) -> fmt::Result {

        try!(write!(fmt, "%{} = ", value.name()));
        self::value(value.subvalue(), printer, fmt)
    }

    pub fn global_ref(value: &value::GlobalRef,
                      printer: &mut Printer,
                      fmt: &mut fmt::Formatter) -> fmt::Result {

        let global = printer.module.get_global(value.global_id());
        write!(fmt, "%{}", global.name())
    }

    pub fn block_ref(value: &value::BlockRef,
                     printer: &mut Printer,
                     fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }

    pub fn function_ref(value: &value::FunctionRef,
                        printer: &mut Printer,
                        fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }

    pub fn register_ref(value: &value::RegisterRef,
                        printer: &mut Printer,
                        fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "%<reg>")
    }

    pub mod instruction
    {
        use ir::{Module,Value,Instruction};
        use ir::instruction::{self,Binary};
        use std::fmt;
        use util;
        use super::super::Printer;

        pub fn instruction(inst: &Instruction,
                           printer: &mut Printer,
                           fmt: &mut fmt::Formatter) -> fmt::Result {
            match inst {
                &Instruction::Add(ref i) => arithmetic_binop("add", i, printer, fmt),
                &Instruction::Sub(ref i) => arithmetic_binop("sub", i, printer, fmt),
                &Instruction::Mul(ref i) => arithmetic_binop("mul", i, printer, fmt),
                &Instruction::Div(ref i) => arithmetic_binop("div", i, printer, fmt),
                &Instruction::Shl(ref i) => arithmetic_binop("shl", i, printer, fmt),
                &Instruction::Shr(ref i) => arithmetic_binop("shr", i, printer, fmt),

                &Instruction::Call(ref i) => call(i, printer, fmt),
                &Instruction::Break(ref i) => br(i, printer, fmt),
                &Instruction::Return(ref i) => ret(i, printer, fmt),
            }
        }

        pub fn arithmetic_binop<I>(mnemonic: &'static str,
                                   inst: &I,
                                   printer: &mut Printer,
                                   fmt: &mut fmt::Formatter) -> fmt::Result
            where I: Binary {

            let (lhs,rhs) = inst.operands();

            try!(write!(fmt, "{} ", mnemonic));
            try!(super::value(lhs, printer, fmt));
            try!(write!(fmt, ", "));
            try!(super::value(rhs, printer, fmt));
            Ok(())
        }

        pub fn call(inst: &instruction::Call,
                    printer: &mut Printer,
                    fmt: &mut fmt::Formatter) -> fmt::Result {
            let func = if let &Value::FunctionRef(ref f) = inst.target() {
                f
            } else {
                unreachable!(); // target must be function
            };
            write!(fmt, "call {} {}",
                   util::comma_separated_values(func.signature().returns()),
                   func.name())

        }
        
        pub fn br(inst: &instruction::Break,
                  printer: &mut Printer,
                  fmt: &mut fmt::Formatter) -> fmt::Result {
            unimplemented!();
        }

        pub fn ret(inst: &instruction::Return,
                   printer: &mut Printer,
                   fmt: &mut fmt::Formatter) -> fmt::Result {
            try!(write!(fmt, "ret "));

            match inst.subvalue() {
                Some(i) => super::value(i, printer, fmt),
                None => write!(fmt, "void"),
            }
        }
    }
}

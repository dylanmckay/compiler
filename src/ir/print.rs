
use ir;
use lang;
use util;
use ir::Value;
use std::fmt;

// TODO: pass the module to all functions so we can lookup globals et al

impl fmt::Display for ir::Module
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self::module(self, fmt)
    }
}

pub fn module(module: &ir::Module, fmt: &mut fmt::Formatter) -> fmt::Result {
    
    let mut global_accum = 0;
    for global in module.globals() {
        try!(self::global(global, module, fmt, &mut global_accum));
    }

    try!(write!(fmt, "\n"));
   
    for func in module.functions() {
         try!(self::function(func, module, fmt));
    }

    Ok(())
}

pub fn global(global: &ir::Global,
              module: &ir::Module,
              fmt: &mut fmt::Formatter,
              accum: &mut u64) -> fmt::Result {
    write!(fmt, "%{} = ", global.name());
    self::root_value(global.value(), module, fmt, accum)
}

pub fn function(func: &ir::Function,
                module: &ir::Module,
                fmt: &mut fmt::Formatter) -> fmt::Result {

    let mut accum = 1;

    try!(write!(fmt, "define {} @{}({}) {{\n",
                     util::comma_separated_values(func.signature.returns()),
                     func.name(),
                     util::comma_separated_values(func.signature.parameters())));

    for block in func.blocks() {
        try!(self::block(block, module, fmt, &mut accum));
    }

    write!(fmt, "}}")
}

pub fn block(block: &ir::Block,
             module: &ir::Module,
             fmt: &mut fmt::Formatter,
             accum: &mut u64) -> fmt::Result {

    try!(write!(fmt, "{}:\n", block.name()));

    for value in block.subvalues() {
        try!(self::root_value(&value, module, fmt, accum));
    }

    Ok(())
}

pub fn root_value(value: &ir::Value,
                  module: &ir::Module,
                  fmt: &mut fmt::Formatter,
                  accum: &mut u64) -> fmt::Result {
    try!(write!(fmt, "\t"));
    try!(self::value::plain_value(value, module, fmt, accum));
    write!(fmt, "\n")
}

pub fn name(name: &lang::Name,
            fmt: &mut fmt::Formatter,
            accum: &mut u64) -> fmt::Result {
    unimplemented!();
}


pub mod value
{
    use ir::{Module,Value,Instruction,value};
    use std::fmt;
    use util;
    use lang;

    pub fn value(value: &Value,
                 module: &Module,
                 fmt: &mut fmt::Formatter,
                 accum: &mut u64) -> fmt::Result {
        use lang::Value;

        // simple values are not parenthesised.
        if !value.is_simple() {
            try!(write!(fmt, "("));
        }

        try!(self::plain_value(value, module, fmt, accum));

        if !value.is_simple() {
            try!(write!(fmt, ")"));
        }

        Ok(())
    }

    pub fn plain_value(value: &Value,
                       module: &Module,
                       fmt: &mut fmt::Formatter,
                       accum: &mut u64) -> fmt::Result {

        match value {
            &Value::Literal(ref val) => self::literal(val, fmt),
            &Value::Pointer(ref val) => self::pointer(val, module, fmt, accum),
            &Value::Register(ref val) => self::register(val, module, fmt, accum),
            &Value::Instruction(ref val) => self::instruction::instruction(val, module, fmt, accum),
            &Value::GlobalRef(ref val) => self::global_ref(val, module, fmt, accum),
            &Value::BlockRef(ref val) => self::block_ref(val, module, fmt, accum),
            &Value::FunctionRef(ref val) => self::function_ref(val, module, fmt, accum),
            &Value::RegisterRef(ref val) => self::register_ref(val, module, fmt, accum),
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
                          module: &Module,
                          fmt: &mut fmt::Formatter,
                          accum: &mut u64) -> fmt::Result {
        unimplemented!();
        //write!(fmt, "{{ {} }}", util::comma_separated_values(value.fields()))
    }

    pub fn pointer(value: &value::Pointer,
                   module: &Module,
                   fmt: &mut fmt::Formatter,
                   accum: &mut u64) -> fmt::Result {
        try!(self::value(value.underlying(), module, fmt, accum));
        write!(fmt, "*")
    }

    pub fn register(value: &value::Register,
                    module: &Module,
                    fmt: &mut fmt::Formatter,
                    accum: &mut u64) -> fmt::Result {

        try!(write!(fmt, "%{} = ", value.name()));
        self::value(value.subvalue(), module, fmt, accum)
    }

    pub fn global_ref(value: &value::GlobalRef,
                      module: &Module,
                      fmt: &mut fmt::Formatter,
                      accum: &mut u64) -> fmt::Result {

        let global = module.get_global(value.global_id());
        write!(fmt, "%{}", global.name())
    }

    pub fn block_ref(value: &value::BlockRef,
                     module: &Module,
                     fmt: &mut fmt::Formatter,
                     accum: &mut u64) -> fmt::Result {
        unimplemented!();
    }

    pub fn function_ref(value: &value::FunctionRef,
                        module: &Module,
                        fmt: &mut fmt::Formatter,
                        accum: &mut u64) -> fmt::Result {
        unimplemented!();
    }

    pub fn register_ref(value: &value::RegisterRef,
                        module: &Module,
                        fmt: &mut fmt::Formatter,
                        accum: &mut u64) -> fmt::Result {
        write!(fmt, "%<reg>")
    }

    pub mod instruction
    {
        use ir::{Module,Value,Instruction};
        use ir::instruction::{self,Binary};
        use std::fmt;
        use util;

        pub fn instruction(inst: &Instruction,
                           module: &Module,
                           fmt: &mut fmt::Formatter,
                           accum: &mut u64) -> fmt::Result {
            match inst {
                &Instruction::Add(ref i) => arithmetic_binop("add", i, module, fmt, accum),
                &Instruction::Sub(ref i) => arithmetic_binop("sub", i, module, fmt, accum),
                &Instruction::Mul(ref i) => arithmetic_binop("mul", i, module, fmt, accum),
                &Instruction::Div(ref i) => arithmetic_binop("div", i, module, fmt, accum),
                &Instruction::Shl(ref i) => arithmetic_binop("shl", i, module, fmt, accum),
                &Instruction::Shr(ref i) => arithmetic_binop("shr", i, module, fmt, accum),

                &Instruction::Call(ref i) => call(i, module, fmt, accum),
                &Instruction::Break(ref i) => br(i, module, fmt, accum),
                &Instruction::Return(ref i) => ret(i, module, fmt, accum),
            }
        }

        pub fn arithmetic_binop<I>(mnemonic: &'static str,
                                   inst: &I,
                                   module: &Module,
                                   fmt: &mut fmt::Formatter,
                                   accum: &mut u64) -> fmt::Result
            where I: Binary {

            let (lhs,rhs) = inst.operands();

            try!(write!(fmt, "{} ", mnemonic));
            try!(super::value(lhs, module, fmt, accum));
            try!(write!(fmt, ", "));
            try!(super::value(rhs, module, fmt, accum));
            Ok(())
        }

        pub fn call(inst: &instruction::Call,
                    module: &Module,
                    fmt: &mut fmt::Formatter,
                    accum: &mut u64) -> fmt::Result {
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
                  module: &Module,
                  fmt: &mut fmt::Formatter,
                  accum: &mut u64) -> fmt::Result {
            unimplemented!();
        }

        pub fn ret(inst: &instruction::Return,
                   module: &Module,
                   fmt: &mut fmt::Formatter,
                   accum: &mut u64) -> fmt::Result {
            try!(write!(fmt, "ret "));

            match inst.subvalue() {
                Some(i) => super::value(i, module, fmt, accum),
                None => write!(fmt, "void"),
            }
        }
    }
}

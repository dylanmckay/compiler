
use ir;
use util;
use ir::Expression;
use util::Identifiable;
use std::fmt;
use std::collections::HashMap;

/// A hack to be able to implement `Display` on a module.
///
/// This is to work around Rust's orphan rules.
pub struct Printable<'a>(&'a ir::Module);

impl<'a> fmt::Display for Printable<'a>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self::module(self.0, fmt)
    }
}

/// Creates a printable module.
pub fn printable(module: &ir::Module) -> Printable {
    Printable(module)
}

/// Holds IR printing state.
pub struct Printer<'a>
{
    /// The module that is being printed.
    module: &'a ir::Module,

    /// Keeps track of the current register number for
    /// each function.
    register_accumulator: u32,

    /// Holds unnamed register ids and the register numbers we map
    /// them to. Can be cleared upon each function.
    register_map: HashMap<util::Id,u32>,
}

impl<'a> Printer<'a>
{
    /// Assigns a number to a register internally.
    /// Returns the registers newly assigned number.
    fn assign_register(&mut self, reg: &ir::value::Register) -> u32 {

        let id = reg.get_id();
        let num = self.register_accumulator;

        self.register_accumulator += 1;
        self.register_map.insert(id, num);

        num
    }

    /// Gets the assigned number of a register.
    fn register_number(&self, id: util::Id) -> u32 {
        self.register_map.get(&id).unwrap().clone()
    }

    /// Clears the stored register state for the current function.
    fn clear_registers(&mut self) {
        self.register_accumulator = 0;
        self.register_map.clear();
    }
}

/// Prints an IR module.
pub fn module(module: &ir::Module, fmt: &mut fmt::Formatter) -> fmt::Result {

    let mut printer = Printer {
        register_map: HashMap::new(),
        module: module,
        register_accumulator: 0,
    };

    for global in module.globals() {
        try!(self::global(global, &mut printer, fmt));
    }

    try!(write!(fmt, "\n"));

    for func in module.functions() {
         try!(self::function(func, &mut printer, fmt));
         try!(write!(fmt, "\n"));
    }

    Ok(())
}

pub fn global(global: &ir::Global,
              printer: &mut Printer,
              fmt: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(fmt, "%{} = ", global.name()));
    try!(plain_value(global.value(), printer, fmt));
    write!(fmt, "\n")
}

pub fn function(func: &ir::Function,
                printer: &mut Printer,
                fmt: &mut fmt::Formatter) -> fmt::Result {
    // Initialise register accounting
    printer.clear_registers();
    let signature = func.signature();

    try!(write!(fmt, "define {} @{}({}) {{\n",
                     util::comma_separated_values(signature.returns()),
                     func.name(),
                     util::comma_separated_values(signature.parameters())));

    for block in func.blocks() {
        try!(self::block(block, printer, fmt));
    }

    write!(fmt, "}}\n")
}

pub fn block(block: &ir::Block,
             printer: &mut Printer,
             fmt: &mut fmt::Formatter) -> fmt::Result {

    try!(write!(fmt, "{}:\n", block.name()));

    for value in block.values() {
        try!(self::root_value(value, printer, fmt));
    }

    Ok(())
}

pub fn condition(cond: &ir::Condition,
                 printer: &mut Printer,
                 fmt: &mut fmt::Formatter) -> fmt::Result {
    match *cond {
        // trivial conditions
        ir::Condition::True => write!(fmt, "true"),
        ir::Condition::False => write!(fmt, "false"),

        // binary conditions
        ir::Condition::Equal(ref lhs, ref rhs) |
        ir::Condition::NotEqual(ref lhs, ref rhs) |
        ir::Condition::GreaterThan(ref lhs, ref rhs) |
        ir::Condition::GreaterThanOrEq(ref lhs, ref rhs) |
        ir::Condition::LessThan(ref lhs, ref rhs) |
        ir::Condition::LessThanOrEq(ref lhs, ref rhs) => {
            try!(value(lhs, printer, fmt));
            try!(write!(fmt, " {} ", cond.abbreviation()));
            try!(value(rhs, printer, fmt));

            Ok(())
        },
    }
}

pub fn root_value(value: &ir::Value,
                  printer: &mut Printer,
                  fmt: &mut fmt::Formatter) -> fmt::Result {
    root_expression(value.expression(), printer, fmt)
}

pub fn root_expression(expression: &ir::Expression,
                       printer: &mut Printer,
                       fmt: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(fmt, "\t"));
    try!(self::expression::plain(expression, printer, fmt));
    write!(fmt, "\n")
}

pub fn value(value: &ir::Value,
             printer: &mut Printer,
             fmt: &mut fmt::Formatter) -> fmt::Result {
    expression::expression(value.expression(), printer, fmt)
}

pub fn plain_value(value: &ir::Value,
                   printer: &mut Printer,
                   fmt: &mut fmt::Formatter) -> fmt::Result {
    expression::plain(value.expression(), printer, fmt)
}

pub mod expression
{
    use ir::{Expression,value};
    use std::fmt;
    use lang;
    use util::Identifiable;
    use super::Printer;

    pub fn expression(expr: &Expression,
                      printer: &mut Printer,
                      fmt: &mut fmt::Formatter) -> fmt::Result {
        use lang::Value;

        // simple values are not parenthesised.
        if !expr.is_simple() {
            try!(write!(fmt, "("));
        }

        try!(self::plain(expr, printer, fmt));

        if !expr.is_simple() {
            try!(write!(fmt, ")"));
        }

        Ok(())
    }

    pub fn plain(expr: &Expression,
                 printer: &mut Printer,
                 fmt: &mut fmt::Formatter) -> fmt::Result {

        match *expr {
            Expression::Literal(ref val) => self::literal(val, fmt),
            Expression::Register(ref val) => self::register(val, printer, fmt),
            Expression::Instruction(ref val) => self::instruction::instruction(val, printer, fmt),
            Expression::GlobalRef(ref val) => self::global_ref(val, printer, fmt),
            Expression::BlockRef(ref val) => self::block_ref(val, printer, fmt),
            Expression::FunctionRef(ref val) => self::function_ref(val, printer, fmt),
            Expression::RegisterRef(ref val) => self::register_ref(val, printer, fmt),
            Expression::ArgumentRef(ref val) => self::argument_ref(val, printer, fmt),
        }
    }

    pub fn literal(literal: &value::Literal,
                   fmt: &mut fmt::Formatter) -> fmt::Result {
        match *literal {
            value::Literal::Integer(ref val) => self::literal_integer(val, fmt),
            _ => unimplemented!(),
        }
    }

    pub fn literal_integer(literal: &value::literal::Integer,
                           fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} {}", literal.ty(), literal.value())
    }

    pub fn literal_struct(_literal: &value::literal::Struct,
                          _printer: &mut Printer,
                          _fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
        //write!(fmt, "{{ {} }}", util::comma_separated_values(value.fields()))
    }

    pub fn register(reg: &value::Register,
                    printer: &mut Printer,
                    fmt: &mut fmt::Formatter) -> fmt::Result {

        try!(write!(fmt, "%"));

        match *reg.name() {
            lang::Name::Unnamed => {
                let number = printer.assign_register(reg);
                try!(write!(fmt, "{}", number));
            },
            // the register has an explicit name
            lang::Name::Named(ref name) => {
                try!(write!(fmt, "{}", name));
            }
        }

        try!(write!(fmt, " = "));
        super::value(reg.subvalue(), printer, fmt)
    }

    pub fn global_ref(global_ref: &value::GlobalRef,
                      printer: &mut Printer,
                      fmt: &mut fmt::Formatter) -> fmt::Result {

        let global = printer.module.get_global(global_ref.global_id());
        write!(fmt, "%{}", global.name())
    }

    pub fn block_ref(block_ref: &value::BlockRef,
                     printer: &mut Printer,
                     fmt: &mut fmt::Formatter) -> fmt::Result {
        let block = printer.module.get_block(block_ref.block_id());
        write!(fmt, "{}", block.name())
    }

    pub fn function_ref(_func_ref: &value::FunctionRef,
                        _printer: &mut Printer,
                        _fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }

    pub fn register_ref(reg_ref: &value::RegisterRef,
                        printer: &mut Printer,
                        fmt: &mut fmt::Formatter) -> fmt::Result {
        let number = printer.register_number(reg_ref.get_id());
        write!(fmt, "%{}", number)
    }

    pub fn argument_ref(arg_ref: &value::ArgumentRef,
                        printer: &mut Printer,
                        fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }

    pub mod instruction
    {
        use ir::{Expression,Instruction};
        use ir::instruction::{self,Binary};
        use std::fmt;
        use util;
        use super::super::value;
        use super::super::Printer;

        pub fn instruction(inst: &Instruction,
                           printer: &mut Printer,
                           fmt: &mut fmt::Formatter) -> fmt::Result {
            match *inst {
                Instruction::Add(ref i) => arithmetic_binop("add", i, printer, fmt),
                Instruction::Sub(ref i) => arithmetic_binop("sub", i, printer, fmt),
                Instruction::Mul(ref i) => arithmetic_binop("mul", i, printer, fmt),
                Instruction::Div(ref i) => arithmetic_binop("div", i, printer, fmt),
                Instruction::Shl(ref i) => arithmetic_binop("shl", i, printer, fmt),
                Instruction::Shr(ref i) => arithmetic_binop("shr", i, printer, fmt),

                Instruction::Call(ref i) => call(i, fmt),
                Instruction::Break(ref i) => br(i, printer, fmt),
                Instruction::Return(ref i) => ret(i, printer, fmt),
            }
        }

        pub fn arithmetic_binop<I>(mnemonic: &'static str,
                                   inst: &I,
                                   printer: &mut Printer,
                                   fmt: &mut fmt::Formatter) -> fmt::Result
            where I: Binary {

            let (lhs,rhs) = inst.operands();

            try!(write!(fmt, "{} ", mnemonic));
            try!(value(lhs, printer, fmt));
            try!(write!(fmt, ", "));
            try!(value(rhs, printer, fmt));
            Ok(())
        }

        pub fn call(inst: &instruction::Call,
                    fmt: &mut fmt::Formatter) -> fmt::Result {
            let func = if let Expression::FunctionRef(ref f) = *inst.target().expression()  {
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
            try!(write!(fmt, "break "));
            try!(super::super::condition(inst.condition(), printer, fmt));
            try!(write!(fmt, " "));
            value(inst.target(), printer, fmt)
        }

        pub fn ret(inst: &instruction::Return,
                   printer: &mut Printer,
                   fmt: &mut fmt::Formatter) -> fmt::Result {
            try!(write!(fmt, "ret "));

            match inst.subvalue() {
                Some(i) => value(i, printer, fmt),
                None => write!(fmt, "void"),
            }
        }
    }
}


#![feature(iter_arith,plugin)]
#![feature(associated_consts)]

#![plugin(clippy)]

extern crate num;
extern crate bit_vec;

/// Various utilities.
#[macro_use]
pub mod util;
/// The immediate representation (IR).
pub mod ir;
/// The domain specific language.
pub mod dsl;
/// Language-agnostic traits.
pub mod lang;
/// The pass infrastructure.
pub mod pass;
/// The target information module.
pub mod target;


fn main() {
    let mut module = self::create_module();
    let mut pm = self::create_ir_pass_manager();

    println!("Previously:\n\n{}", module);

    module = pm.run(module);

    module = module.flatten();
    println!("\n\nAfterwards:\n\n{}", module);

    print!("\nVerifying...");
    let result = ir::verifier::verify(&module);

    match result {
        Ok(..) => println!("passed!"),
        Err(ref msg) => println!("failed: {}", msg),
    }
}

fn create_module() -> ir::Module {

    let lhs = ir::Value::i32(23);
    let rhs = ir::Value::i32(2);

    let global = ir::Global::new("MyGlobal".into(), lhs.clone());

    let bb2 = {

        let inst_add1 = ir::Instruction::add(lhs.clone(), rhs.clone());
        let inst_mul = ir::Instruction::mul(inst_add1, rhs.clone());
        let inst_ret = ir::Instruction::ret(Some(inst_mul.clone().into()));

        let mut block = ir::Block::empty("other");
        block.append_value(inst_ret);
        block
    };

    let bb1 = {
        let inst_br = ir::Instruction::br(ir::Value::block_ref(&bb2));

        let mut block = ir::Block::empty("entry");
        block.append_value(inst_br);
        block
    };

    let sig = lang::Signature::new().ret(ir::Type::i32());
    let mut function = ir::Function::empty("main", sig);
    function.append_block(bb1);
    function.append_block(bb2);

    ir::Module::empty().function(function)
                       .global(global)
}

fn create_ir_pass_manager() -> pass::Manager<ir::Value> {
    pass::Manager::empty()
        //.add_pass(pass::transforms::ConstantFolding)
        .add_pass(pass::transforms::StrengthReduction)
        .add_pass(pass::transforms::DeadCodeElimination)
}

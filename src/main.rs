
#![feature(iter_arith)]
#![feature(associated_consts)]

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
        let inst_ret_void = ir::Instruction::ret_void();
        let mut block = ir::Block::empty(ir::Name::named("other".to_owned()));
        block.add(inst_ret_void);
        block
    };

    let bb1 = {
        let inst_add1 = ir::Instruction::add(lhs.clone(), rhs.clone());
        let inst_mul = ir::Instruction::mul(inst_add1, rhs.clone());
        let inst_ret = ir::Instruction::ret(Some(inst_mul.clone().into()));
        //let inst_jump = ir::Instruction::br(ir::Value::block_ref(&bb2));

        let mut block = ir::Block::empty(ir::Name::named("entry".to_owned()));
        block.add(inst_ret);
        block
    };

    let sig = lang::Signature::new().ret(ir::Type::i32());
    let function = ir::Function::empty("main".into(), sig).add(bb1)
                                                          .add(bb2);

    ir::Module::empty().function(function)
                       .global(global)
}

fn create_ir_pass_manager() -> pass::Manager<ir::Value> {
    pass::Manager::empty()
        //.add(pass::transforms::ConstantFolding)
        .add(pass::transforms::StrengthReduction)
        .add(pass::transforms::DeadCodeElimination)
}

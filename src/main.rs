
#![feature(iter_arith)]
#![feature(associated_consts)]

extern crate num;
extern crate bit_vec;

/// Various utilities.
#[macro_use]
pub mod util;
/// The immediate representation (IR).
pub mod ir;
/// The machine code backend.
pub mod mc;
/// The domain specific language.
pub mod dsl;
/// Language-agnostic traits.
pub mod lang;
/// The pass infrastructure.
pub mod pass;


fn main() {
    let mut module = self::create_module();
    let mut pm = self::create_ir_pass_manager();

    println!("Previously:\n{}", module);

    module = pm.run(module);

    println!("Afterwards:\n{}", module);

    print!("Verifying...");
    let result = ir::verifier::verify(&module);

    match result {
        Ok(..) => println!("passed!"),
        Err(ref msg) => println!("failed: {}", msg),
    }
}

fn create_module() -> ir::Module {
    let op_ty = ir::types::Integer::i32();

    let lhs = ir::Value::integer(op_ty, 23i32).unwrap();
    let rhs = ir::Value::integer(op_ty, 2i32).unwrap();

    let inst_add1 = ir::Instruction::add(op_ty.into(), lhs.clone(), rhs.clone());
    let inst_add2 = ir::Instruction::add(op_ty.into(), rhs.clone(), lhs.clone());
    let inst_mul = ir::Instruction::mul(op_ty.into(), inst_add1.clone().into(), rhs.clone());
    let inst_ret = ir::Instruction::ret(Some(inst_add1.clone().into()));

    let basicblock = ir::Block::empty(ir::Name::named("entry".to_owned())).add(inst_ret);

    let sig = ir::types::Function::new().ret(op_ty.into());
    let function = ir::Function::empty(ir::Name::named("main".to_owned()), sig).add(basicblock.clone())
                                                                               .add(basicblock.clone());

    ir::Module::empty().function(function)
}

fn create_ir_pass_manager() -> pass::Manager<ir::Module> {
    pass::Manager::empty()
//        .add(pass::transforms::ir::ConstantFolding)
        .add(pass::transforms::ir::StrengthReduction)
        .add(pass::transforms::DeadCodeElimination)
}

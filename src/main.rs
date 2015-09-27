
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
    use ir::TypeTrait;

    let op_ty = ir::types::Integer::i32();

    let lhs = ir::Value::constant_integer(op_ty, 23i32).unwrap();
    let rhs = ir::Value::constant_integer(op_ty, 55i32).unwrap();

    let inst_add1 = ir::Instruction::add(op_ty.upcast(), lhs.clone(), rhs.clone());
    let inst_add2 = ir::Instruction::add(op_ty.upcast(), rhs.clone(), lhs.clone());
    let inst_ret = ir::Instruction::ret(Some(lhs));

    let basicblock = ir::BasicBlock::empty(ir::Name::named("entry".to_owned())).add(inst_add1)
                                                                               .add(inst_add2)
                                                                               .add(inst_ret);

    let sig = ir::types::Signature::new().ret(op_ty.upcast());
    let function = ir::Function::empty(ir::Name::named("main".to_owned()), sig).add(basicblock.clone())
                                                                               .add(basicblock.clone());

    println!("{}", function);
}


extern crate compiler;

use compiler::{ir,pass};

fn main() {
    let mut module = self::create_module();
    let mut pm = self::create_ir_pass_manager();

    println!("Previously:\n\n{}", ir::printable(&module));

    module = pm.run(module);

    module = module.flatten();
    println!("\n\nAfterwards:\n\n{}", ir::printable(&module));

    print!("\nVerifying...");
    let result = ir::verifier::verify(&module);

    match result {
        Ok(..) => println!("passed!"),
        Err(ref msg) => println!("failed: {}", msg),
    }
}

fn create_module() -> ir::Module {

    let lhs = ir::Value::new(ir::Expression::i32(23));
    let rhs = ir::Value::new(ir::Expression::i32(2));

    let global = ir::Global::new("MyGlobal".into(), lhs.clone());

    let func2 = {
        let bb2 = {

            let inst_add1 = ir::Value::new(ir::Expression::add(lhs.clone(), rhs.clone()));
            let inst_mul = ir::Value::new(ir::Expression::mul(inst_add1, rhs.clone()));
            let inst_ret = ir::Value::new(ir::Expression::ret(inst_mul));

            let mut block = ir::Block::empty("other");
            block.append_value(inst_ret);
            block
        };

        let bb1 = {
            let inst_br = ir::Value::new(
                ir::Expression::br(ir::Condition::True,
                                   ir::Value::new(ir::Expression::block_ref(&bb2))),
            );

            let mut block = ir::Block::empty("entry");
            block.append_value(inst_br);
            block
        };

        let sig = ir::Signature::empty().ret(ir::Type::i32());
        let mut f = ir::Function::empty("do_thing", sig);

        f.append_block(bb1);
        f.append_block(bb2);
        f
    };

    let func1 = {
        let bb = {
            let inst_call = ir::Value::new(ir::Expression::call(
                    ir::Value::new(ir::Expression::function_ref(&func2))
            ));

            let inst_ret = ir::Value::new(ir::Expression::ret(inst_call));

            let mut block = ir::Block::empty("main");
            block.append_value(inst_ret);
            block
        };

        let sig = ir::Signature::empty().ret(ir::Type::i32());
        let mut f = ir::Function::empty("main", sig);
        f.append_block(bb);
        f
    };


    ir::Module::empty().function(func2)
                       .function(func1)
                       .global(global)
}

fn create_ir_pass_manager() -> pass::Manager {
    pass::Manager::empty()
        //.add_pass(pass::transforms::ConstantFolding)
        .add_pass(pass::transforms::StrengthReduction)
        .add_pass(pass::transforms::DeadCodeElimination)
        .add_pass(pass::transforms::Inliner)
}

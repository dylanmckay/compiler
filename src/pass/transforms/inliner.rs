use {Metadata,Id,Info,Transform};
use ir;

use util::Identifiable;

/// The threshold to inline stuff.
pub const INLINING_WEIGHT_THRESHOLD: f64 = 0.6;

/// An IR strength reduction pass.
pub struct Inliner;

#[derive(Copy,Clone,Debug)]
pub struct Weight(pub f64);

impl Weight
{
    pub fn always() -> Self { Weight(1.0) }
    pub fn never() -> Self { Weight(0.0) }

    pub fn should_inline(self) -> bool { self.0 > INLINING_WEIGHT_THRESHOLD }
}

impl Metadata for Inliner
{
    fn id(&self) -> Id { Id(0x32bbc291) }
    fn name(&self) -> &'static str { "inliner" }
}

impl Transform for Inliner
{
    fn run_module(&mut self,
                  module: ir::Module)
        -> ir::Module {
        let function_ids_to_inline: Vec<_> = module.functions().filter_map(|function| {
            let weight = inlining_weight(function, &module);

            if weight.should_inline() {
                Some(function.get_id())
            } else {
                None
            }
        }).collect();

        // FIXME: This won't work for most cases.
        // Because at this point the IR is in SSA form, we need to inspect the basic block
        // and look at the subvalues of the top level values to inline.

        module.map_functions(|f, _module| {
            f.map_blocks(|block| {
                let values = block.body.into_iter().flat_map(|value| {
                    println!("handling value: {:#?}", value);

                    let exprs = match value.node {
                        ir::Expression::Instruction(inst) => match inst {
                            ir::Instruction::Call(call_inst) => {
                                println!("call_inst_id: {}, func_ids: {:#?}", call_inst.target_id(), function_ids_to_inline);

                                if let Some(_func_id) = function_ids_to_inline.iter().find(|&id| *id == call_inst.target_id()) {
                                    // module.get_function(function_id);
                                    unimplemented!();
                                } else {
                                    vec![ir::Expression::Instruction(ir::Instruction::Call(call_inst))]
                                }
                            },
                            i => {
                                vec![ir::Expression::Instruction(i)]
                            },
                        },
                        e => vec![e],
                    };

                    exprs.into_iter().map(|expr| {
                        ir::Value::new(expr)
                    })
                }).collect();

                ir::Block {
                    body: values,
                    ..block
                }
            })
        })
    }
}

// TODO: blamket impl for all passes
impl Into<Info> for Box<Inliner>
{
    fn into(self) -> Info {
        Info::Transform(self)
    }
}

pub fn inlining_weight(f: &ir::Function, module: &ir::Module) -> Weight {
    let uses = module.users_of(f);

    if uses.len() == 1 {
        Weight::always()
    } else {
        Weight::never()
    }
}


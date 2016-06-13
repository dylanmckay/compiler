use Pattern;
use mir;

/// Selects instructions.
pub struct Selector<V>
{
    patterns: Vec<Pattern<V>>,
}

impl<V> Selector<V>
{
    /// Creates a new instruction selector.
    pub fn new(patterns: Vec<Pattern<V>>) -> Self {
        Selector {
            patterns: patterns,
        }
    }

    pub fn select(&mut self, dag: mir::Dag) {
        unimplemented!();
    }

    fn select_register(&mut self, register: mir::Register) {
        let node = register.value;
        //
        // // check if the node can be directly selected.
        // if let Some(result) = (self.f)(&node) {
        //     vec![Item::Processed(result)]
        // } else {
        //     match node {
        //         mir::Node::Branch(branch) => {
        //             let mut new_registers: Vec<mir::Register> = Vec::new();
        //
        //             let new_operands: Vec<_> = branch.operands.into_iter().map(|mut op| {
        //                 let promoted_register = op.promote_to_register();
        //                 new_registers.push(promoted_register);
        //                 op
        //             }).collect();
        //
        //             let node = mir::Node::Branch(mir::Branch {
        //                 operands: new_operands,
        //                 ..branch
        //             });
        //
        //             let register = mir::Register {
        //                 value: node,
        //                 ..register
        //             };
        //
        //             vec![Item::Unprocessed(register)]
        //         },
        //         _ => panic!("can't select this node"),
        // }
        // }
        unimplemented!();
    }
}


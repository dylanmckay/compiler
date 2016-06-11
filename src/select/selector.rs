use mir;

/// Selects instructions.
pub struct Selector<Out>
{
    f: Box<FnMut(&mir::Node) -> Option<Out>>,
}

pub enum Item<Out>
{
    Processed(Out),
    Unprocessed(mir::Register),
}

impl<Out> Item<Out>
{
    pub fn is_processed(&self) -> bool {
        if let Item::Processed(..) = *self { true } else { false }
    }
}

impl<Out> Selector<Out>
{
    /// Creates a new instruction selector.
    pub fn new<F>(f: Box<F>) -> Self
        where F: FnMut(&mir::Node) -> Option<Out>+'static {
        Selector {
            f: f,
        }
    }

    pub fn select(&mut self, dag: mir::Dag) -> Vec<Out> {
        let mut items: Vec<_> = dag.registers.into_iter().map(|register| {
            Item::Unprocessed(register)
        }).collect();

        loop {
            let next_node_idx = match items.iter().position(|i| !i.is_processed()) {
                Some(idx) => idx,
                None => break,
            };

            let next_node = if let Item::Unprocessed(reg) = items.remove(next_node_idx as _) {
                reg
            } else {
                unreachable!();
            };

            let new_items = self.select_register(next_node);
            for (idx, new_item) in new_items.into_iter().enumerate() {
                items.insert((next_node_idx+idx) as _, new_item);
            }
        }

        items.into_iter().map(|item| {
            if let Item::Processed(output) = item { output } else { unreachable!() }
        }).collect()
    }

    fn select_register(&mut self, register: mir::Register) -> Vec<Item<Out>> {
        let node = register.value;

        // check if the node can be directly selected.
        if let Some(result) = (self.f)(&node) {
            vec![Item::Processed(result)]
        } else {
            match node {
                mir::Node::Branch(branch) => {
                    let mut new_registers: Vec<mir::Register> = Vec::new();

                    let new_operands: Vec<_> = branch.operands.into_iter().map(|mut op| {
                        let promoted_register = op.promote_to_register();
                        new_registers.push(promoted_register);
                        op
                    }).collect();

                    let node = mir::Node::Branch(mir::Branch {
                        operands: new_operands,
                        ..branch
                    });

                    let register = mir::Register {
                        value: node,
                        ..register
                    };

                    vec![Item::Unprocessed(register)]
                },
                _ => panic!("can't select this node"),
            }
        }
    }
}


use PatternValue;
use mir;

/// An adjustment to a pattern.
///
/// Not all possible patterns are an identical match to
/// the MIR. To accomodate this, we have the concept of a
/// pattern _adjustment_.
///
/// When matching patterns, we can have several adjustments
/// which define permutations that would need to be made to the
/// original pattern in order to match.
///
/// We can then look at all the adjustments and figure out which
/// pattern is the most optimal to select.
///
/// Take the case where we have
///
/// ```
/// (add %a, (add %foo, %bar))
/// ```
///
/// This will likely have to have an adjustment to demote the
/// nested addition to a register, so that the code becomes.
///
/// ```
/// %tmp = (add %foo, %bar)
/// (add %a, %tmp)
/// ```
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Adjustment<V: PatternValue>
{
    /// Demotes a subnode to a register.
    DemoteToRegister {
        demotee: mir::Node,
    },
    Target(V::Adjustment),
}

impl<V: PatternValue> Adjustment<V>
{
    pub fn demote_to_register(node: &mir::Node) -> Self {
        Adjustment::DemoteToRegister {
            demotee: node.clone(),
        }
    }

    pub fn apply_several_to(original_root_node: mir::Node, adjustments: &[Self]) -> Vec<mir::Node> {
        let mut result = Vec::new();
        result.push(original_root_node.clone());

        for adjustment in adjustments.iter() {
            // The only adjustments we have so far always push the permuted root node
            // to the bottom of the results list.
            let current_node_idx = result.len() - 1;
            let current_node = result.remove(current_node_idx);

            let permuted = adjustment.apply_to(current_node);
            result.extend(permuted);
        }

        result
    }

    /// Applies an adjustment to a node.
    pub fn apply_to(&self, root_node: mir::Node) -> Vec<mir::Node> {
        let mut new_nodes = Vec::new();

        let root_node = match *self {
            Adjustment::DemoteToRegister { ref demotee } => {
                root_node.recursive_map(&mut |node| {
                    if node == *demotee {
                        let register_ref = mir::Node::new_register_ref(node.ty());
                        new_nodes.push(mir::Node::set(register_ref.clone(), node));
                        register_ref
                    } else {
                        node
                    }
                })
            },
            Adjustment::Target(ref adjustment) => {
                unimplemented!();
            },
        };

        new_nodes.push(root_node);
        new_nodes
    }
}

#[cfg(test)]
mod test
{
    mod apply_to {
        use super::super::*;
        use mir;
        use pattern::DummyPatternValue;

        #[test]
        fn demote_to_register() {
            let register_ref = mir::Node::new_register_ref(mir::Type::i8());
            let example_node = mir::Node::set(register_ref.clone(), mir::Node::i(8, 2));
            println!("example: {:#?}", example_node);

            let adjustment: Adjustment<DummyPatternValue> = Adjustment::DemoteToRegister {
                demotee: mir::Node::i(8, 2)
            };

            let result_nodes = adjustment.apply_to(example_node);
            println!("result: {:#?}", result_nodes);

            assert_eq!(result_nodes.len(), 2);

            let new_set_node = result_nodes[0].expect_branch();
            let new_register_ref = new_set_node.operands[0].clone();
            assert_eq!(new_set_node.operands[1], mir::Node::i(8, 2));

            let permuted_node = result_nodes[1].expect_branch();
            assert_eq!(permuted_node.operands[1], new_register_ref);
        }
    }
}


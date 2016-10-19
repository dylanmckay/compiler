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
/// ```ignore
/// (add %a, (add %foo, %bar))
/// ```
///
/// This will likely have to have an adjustment to demote the
/// nested addition to a register, so that the code becomes.
///
/// ```ignore
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
    /// A target-specific constraint.
    Target(V::Adjustment),
}

/// The result of an adjustment application.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct AdjustmentApplication
{
    /// An adjustment may create extra nodes preceding the node
    /// that was adjusted. For example, promoting an operand
    /// to a register would a register set node to precede the adjusted node.
    pub preceding_nodes: Vec<mir::Node>,

    /// The original node that was adjusted.
    pub adjusted_node: mir::Node,
}

impl AdjustmentApplication
{
    pub fn unadjusted(node: mir::Node) -> Self {
        AdjustmentApplication {
            preceding_nodes: Vec::new(),
            adjusted_node: node,
        }
    }

    /// Gets all of the nodes (in order) that were created or modified
    /// by the adjustment.
    pub fn nodes(&self) -> Vec<mir::Node> {
        let mut nodes = self.preceding_nodes.clone();
        nodes.push(self.adjusted_node.clone());
        nodes
    }

    fn merge(mut self, other: Self) -> Self {
        self.preceding_nodes.extend(other.preceding_nodes);
        self.adjusted_node = other.adjusted_node;
        self
    }
}

impl<V: PatternValue> Adjustment<V>
{
    pub fn demote_to_register(node: &mir::Node) -> Self {
        Adjustment::DemoteToRegister {
            demotee: node.clone(),
        }
    }

    pub fn apply_several_to(root_node: mir::Node, adjustments: &[Self]) -> AdjustmentApplication {
        adjustments.iter().fold(AdjustmentApplication::unadjusted(root_node), |last_application, adjustment| {
            let current_node = last_application.adjusted_node.clone();
            last_application.merge(adjustment.apply_to(current_node))
        })
    }

    /// Applies an adjustment to a node.
    pub fn apply_to(&self, root_node: mir::Node) -> AdjustmentApplication {
        match *self {
            Adjustment::DemoteToRegister { ref demotee } => {
                let mut preceding_nodes = Vec::new();

                let adjusted_node = root_node.recursive_map(&mut |node| {
                    if node == *demotee {
                        let register_ref = mir::Node::new_register_ref(node.ty());
                        preceding_nodes.push(mir::Node::set(register_ref.clone(), node));
                        register_ref
                    } else {
                        node
                    }
                });

                AdjustmentApplication { preceding_nodes: preceding_nodes, adjusted_node: adjusted_node }
            },
            Adjustment::Target(ref _adjustment) => {
                unimplemented!();
            },
        }
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

            let adjustment: Adjustment<DummyPatternValue> = Adjustment::DemoteToRegister {
                demotee: mir::Node::i(8, 2)
            };

            let application = adjustment.apply_to(example_node);

            assert_eq!(application.preceding_nodes.len(), 1);

            let new_set_node = application.preceding_nodes[0].expect_branch();
            let new_register_ref = new_set_node.operands[0].clone();
            assert_eq!(new_set_node.operands[1], mir::Node::i(8, 2));

            let permuted_node = application.adjusted_node.expect_branch();
            assert_eq!(permuted_node.operands[1], new_register_ref);
        }
    }
}


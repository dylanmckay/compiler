use {Dag, Node, Value, Branch, OpCode};

use std::collections::HashMap;
use util;

/// Expands a DAG into the most tree-like DAG it can.
pub fn dag(dag: Dag) -> Dag {
    Context::new().expand_dag(dag)
}

struct RegisterInfo
{
    set_count: i32,
    usage_count: i32,
    initial_value: Node,
}

impl RegisterInfo
{
    fn new(initial_value: Node) -> Self {
        RegisterInfo {
            set_count: 0,
            usage_count: 0,
            initial_value: initial_value,
        }
    }
}

struct Context
{
    registers: HashMap<util::Id, RegisterInfo>,
}

impl Context
{
    fn new() -> Self {
        Context { registers: HashMap::new() }
    }

    fn expand_dag(&mut self, dag: Dag) -> Dag {
        self.calculate_register_info(&dag);

        let nodes = dag.nodes.into_iter().map(|node| {
            self.expand_node(node)
        }).collect();

        self.eliminate_dead_nodes(Dag {
            nodes: nodes,
            ..dag
        })
    }

    fn expand_node(&mut self, node: Node) -> Node {
        match node {
            Node::Branch(mut branch) => {
                let operands = match branch.opcode {
                    OpCode::Set => {
                        // Only expand the value we're setting.
                        branch.operands[1] = self.expand_node(branch.operands[1].clone());
                        branch.operands
                    },
                    _ => {
                        branch.operands.into_iter().
                            map(|operand| self.expand_node(operand)).
                            collect()
                    },
                };

                Node::Branch(Branch {
                    operands: operands,
                    ..branch
                })
            },
            Node::Leaf(value) => {
                match value {
                    Value::RegisterRef(register_ref) => {
                        if let Some(initial_value) = self.possible_register_substitution(register_ref.register_id) {
                            initial_value
                        } else {
                            Node::Leaf(Value::RegisterRef(register_ref))
                        }
                    },
                    value => Node::Leaf(value),
                }
            },
        }
    }

    fn calculate_register_info(&mut self, dag: &Dag) {
        for node in dag.nodes.iter() {
            self.calculate_node_register_info(node)
        }
    }

    fn calculate_node_register_info(&mut self, node: &Node) {
        match *node {
            Node::Leaf(ref val) => self.calculate_value_register_info(val),
            Node::Branch(ref b) => self.calculate_branch_register_info(b),
        }
    }

    fn calculate_value_register_info(&mut self, val: &Value) {
        if let Value::RegisterRef(ref register_ref) = *val {
            self.register_used(register_ref.register_id);
        }
    }

    fn calculate_branch_register_info(&mut self, branch: &Branch) {
        if branch.opcode == OpCode::Set {
            let register_ref = branch.operands[0].expect_leaf().expect_register_ref();
            let value = branch.operands[1].clone();

            self.calculate_node_register_info(&value);
            self.register_defined(register_ref.register_id, value);
        } else {
            for operand in branch.operands.iter() {
                self.calculate_node_register_info(operand);
            }
        }
    }

    fn register_defined(&mut self, id: util::Id, value: Node) {
        let info = self.registers.entry(id).
            or_insert_with(|| RegisterInfo::new(value));

        info.set_count += 1;
    }

    fn register_used(&mut self, id: util::Id) {
        self.registers.get_mut(&id).expect("register used before it was defined").
            usage_count += 1;
    }

    fn possible_register_substitution(&self, id: util::Id) -> Option<Node> {
        let reg_info = self.registers.get(&id).unwrap();

        if reg_info.set_count == 1 && reg_info.usage_count == 1 {
            Some(reg_info.initial_value.clone())
        } else {
            None
        }
    }

    fn was_register_deleted(&self, id: util::Id) -> bool {
        self.possible_register_substitution(id).is_some()
    }

    /// Eliminates all the junk created by the expansion.
    ///
    /// When we get
    ///
    /// ```
    /// (set %foo, <val>)
    /// (set %bar, %foo)
    /// ```
    ///
    /// We recognize the %foo can be inlined. We then replace all references to
    /// `%foo` with `<val>`.
    ///
    /// i.e. after expansion, this becomes.
    ///
    /// ```
    /// (set %foo, <val>)
    /// (set %bar, <val>)
    /// ```
    ///
    /// The original set is now dead and we discard it here.
    fn eliminate_dead_nodes(&self, dag: Dag) -> Dag {
        dag.filter_nodes(|node| {
            if let Node::Branch(ref branch) = *node {
                if branch.opcode == OpCode::Set {
                    match branch.operands[0] {
                        Node::Leaf(Value::RegisterRef(ref reg_ref)) => {
                            !self.was_register_deleted(reg_ref.register_id)
                        },
                        _ => true,
                    }
                } else {
                    true
                }
            } else {
                true
            }
        })
    }
}


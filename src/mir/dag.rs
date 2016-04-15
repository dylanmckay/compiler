use Node;
use ir;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Dag
{
    nodes: Vec<Node>,
}

impl Dag
{
    pub fn new<I>(nodes: I) -> Self
        where I: IntoIterator<Item=Node> {
        Dag {
            nodes: nodes.into_iter().collect(),
        }
    }

    pub fn from_block(block: &ir::Block) -> Self {
        let nodes = block.values().map(|value| {

            if let ir::Expression::Instruction(ref i) = value.expression {
                Node::from_instruction(i)
            } else {
                panic!("all block-level values should be instructions: \
                       expected instruction but got: {:?}", value.expression);
            }
        }).collect();

        Dag {
            nodes: nodes,
        }
    }
}


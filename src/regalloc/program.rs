use Instruction;
use util;

pub struct Item<I: Instruction>
{
    pub id: util::Id,
    pub instruction: Box<I>,
}

pub struct Program<I: Instruction>
{
    pub items: Vec<Item<I>>,
}

impl<I: Instruction> Item<I>
{
    pub fn new(instruction: Box<I>) -> Self {
        Item {
            id: util::Id::next(),
            instruction: instruction,
        }
    }
}

impl<I: Instruction> Program<I>
{
    pub fn build<It>(instructions: It) -> Self
        where It: IntoIterator<Item=Box<I>> {
        Program {
            items: instructions.into_iter().map(Item::new).collect(),
        }
    }

    pub fn allocate(self) -> Self {
        unimplemented!();
    }
}


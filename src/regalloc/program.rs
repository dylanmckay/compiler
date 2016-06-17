use Target;
use util;

pub struct Item<T: Target>
{
    pub id: util::Id,
    pub instruction: T::Instruction,
}

pub struct Program<T: Target>
{
    pub items: Vec<Item<T>>,
}

impl<T: Target> Item<T>
{
    pub fn new(instruction: T::Instruction) -> Self {
        Item {
            id: util::Id::next(),
            instruction: instruction,
        }
    }
}

impl<T: Target> Program<T>
{
    pub fn build<It>(instructions: It) -> Self
        where It: IntoIterator<Item=T::Instruction> {
        Program {
            items: instructions.into_iter().map(Item::new).collect(),
        }
    }

    pub fn into_instructions(self) -> Vec<T::Instruction> {
        self.items.into_iter().map(|item| item.instruction).collect()
    }
}


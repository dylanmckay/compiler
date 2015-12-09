use {Type,Block};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BlockRef
{
    id: util::Id,
}

impl BlockRef
{
    pub fn reference(block: &Block) -> Self {
        BlockRef {
            id: block.id(),
        }
    }

    pub fn block_id(&self) -> util::Id {
        self.id
    }

    pub fn ty(&self) -> Type {
        Type::block()
    }
}

impl_expression!(BlockRef);

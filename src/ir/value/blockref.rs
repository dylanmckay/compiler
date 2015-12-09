use {Expression,Type,Block,ExpressionTrait};
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BlockRef
{
    block_id: util::Id,
}

impl BlockRef
{
    pub fn reference(block: &Block) -> Self {
        BlockRef {
            block_id: block.id(),
        }
    }

    pub fn block_id(&self) -> util::Id {
        self.block_id
    }

    pub fn ty(&self) -> Type {
        Type::block()
    }
}

impl ExpressionTrait for BlockRef { }

impl Into<Expression> for BlockRef
{
    fn into(self) -> Expression {
        Expression::BlockRef(self)
    }
}

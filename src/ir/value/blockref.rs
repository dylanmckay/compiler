
use ir;
use util;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BlockRef
{
    block_id: util::Id,
}

impl BlockRef
{
    pub fn reference(block: &ir::Block) -> Self {
        BlockRef {
            block_id: block.id(),
        }
    }

    pub fn block_id(&self) -> util::Id {
        self.block_id
    }

    pub fn ty(&self) -> ir::Type {
        ir::Type::block()
    }
}

impl ir::ValueTrait for BlockRef { }

impl Into<ir::Value> for BlockRef
{
    fn into(self) -> ir::Value {
        ir::Value::BlockRef(self)
    }
}


use ir;
use util;
use std;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct BlockRef
{
    id: util::Id,
}

impl BlockRef
{
    pub fn reference(block: &ir::Block) -> Self {
        BlockRef {
            id: block.id(),
        }
    }

    pub fn ty(&self) -> ir::Type {
        ir::Type::block()
    }
}

impl ir::ValueTrait for BlockRef { }

impl std::fmt::Display for BlockRef
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.id.fmt(fmt)
    }
}

impl Into<ir::Value> for BlockRef
{
    fn into(self) -> ir::Value {
        ir::Value::BlockRef(self)
    }
}

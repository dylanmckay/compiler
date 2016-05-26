use {Instruction,Value,Expression,Type};

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Copy
{
    dst: Box<Value>,
    src: Box<Value>,
}

impl Copy
{
    pub fn new(dst: Value, src: Value) -> Self {
        assert!(dst.node.ty() == src.node.ty());

        Copy {
            dst: Box::new(dst),
            src: Box::new(src),
        }
    }

    pub fn ty(&self) -> Type {
        self.dst.node.ty()
    }
}

impl_instruction!(Copy: dst, src);
impl_instruction_binary!(Copy: dst, src);


use target::mc::Encodable;

pub trait Instruction : Encodable
{
    fn mnemonic() -> &'static str;
}

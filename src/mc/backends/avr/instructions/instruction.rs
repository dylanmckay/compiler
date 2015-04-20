
use mc::Encodable;

pub trait Instruction : Encodable
{
    fn mnemonic() -> &'static str;
}

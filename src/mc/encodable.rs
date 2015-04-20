
// TODO: Make output an associated type
pub trait Encodable
{
    /// Encodes the object.
    /// Encoded data is placed in the first tuple element, and
    /// the number of bytes used is placed in the second.
    fn encode(&self) -> (u64,u8);
}

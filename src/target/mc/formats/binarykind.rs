
/// Specifies the kind of a binary.
#[derive(Copy,Clone,Debug)]
pub enum BinaryKind
{
    /// The executable is directly executable.
    Executable,
    /// The file needs is an unlinked executable.
    Object,
    /// The file is a linked shared library.
    SharedObject,
}

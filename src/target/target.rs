/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;
}

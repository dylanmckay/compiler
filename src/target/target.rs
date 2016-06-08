use select;

/// A target.
pub trait Target
{
    fn name(&self) -> &'static str;

    fn create_legalizer(&self) -> select::Legalizer;
}


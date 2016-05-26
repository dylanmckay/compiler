use std::cmp;
use std::fmt;

pub struct Spanned<T>
{
    pub node: T,
}

impl<T: Clone> Clone for Spanned<T>
{
    fn clone(&self) -> Self {
        Spanned {
            node: self.node.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Spanned<T>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.node, fmt)
    }
}

impl<T: cmp::PartialEq> cmp::PartialEq for Spanned<T>
{
    fn eq(&self, rhs: &Self) -> bool { self.node == rhs.node }
}

impl<T: cmp::Eq> cmp::Eq for Spanned<T> { }


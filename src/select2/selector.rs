use {Context, Pattern, Selectable, PatternValue};
use mir;

/// Selects instructions.
pub struct Selector<S: Selectable + 'static, V: PatternValue>
{
    pub patterns: Vec<Pattern<S, V>>,
}

impl<S, V> Selector<S, V>
    where S: Selectable+'static, V: PatternValue
{
    /// Creates a new instruction selector.
    pub fn new(patterns: Vec<Pattern<S, V>>) -> Self {
        Selector {
            patterns: patterns,
        }
    }

    pub fn select(nodes: Vec<mir::Node>) -> Vec<S> {
        let context = Context::<S,V>::new(nodes);
        context.select()
    }
}


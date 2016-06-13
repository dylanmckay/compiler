use {Pattern, PatternNode, PatternValue};
use mir;

/// Selects instructions.
pub struct Selector<V: PatternValue>
{
    pub patterns: Vec<Pattern<V>>,
}

impl<V> Selector<V>
    where V: PatternValue
{
    /// Creates a new instruction selector.
    pub fn new(patterns: Vec<Pattern<V>>) -> Self {
        Selector {
            patterns: patterns,
        }
    }

    pub fn select(&mut self, dag: mir::Dag) -> Vec<Pattern<V>> {
        dag.registers.iter().map(|register| {
            let matches = self.find_matches(&register.value);

            match self::find_optimal_match(&matches) {
                Some(pattern) => pattern.clone(),
                None => panic!("no patterns matching for this node: {:#?}", register.value),
            }
        }).collect()
    }

    fn find_matches(&mut self, node: &mir::Node) -> Vec<Pattern<V>> {
        self.patterns.iter().filter(|pattern| pattern.matches(node)).cloned().collect()
    }
}

fn find_optimal_match<V>(patterns: &[Pattern<V>]) -> Option<&Pattern<V>>
    where V: PatternValue {
    // In most cases, the pattern with the least amount of nodes will
    // be the most optimal.
    patterns.iter().min_by_key(|pattern| pattern.root.area())
}


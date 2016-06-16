use {Pattern, PatternNode, PatternValue, MatchResult, Adjustment};
use mir;

/// Selects instructions.
pub struct Selector<V: PatternValue>
{
    pub patterns: Vec<Pattern<V>>,
}

/// A pattern that matched with a node.
pub struct MatchedPattern<V: PatternValue>
{
    pub node: mir::Node,
    pub pattern: Pattern<V>,
    pub result: MatchResult<V>,
}

/// A permutation of a node is a permuted node which has a perfect match.
///
/// When building matches, we see what adjustments we need to make to
/// the node in order to have a perfect match on a pattern. This can
/// include creating new root nodes (for example when demoting subnodes
/// to registers).
///
/// We can see all the matching permutations for nodes and decide which
/// is the most optimal.
#[derive(Debug,Clone)]
pub struct Permutation<V: PatternValue>
{
    pub nodes: Vec<mir::Node>,
    pub pattern: Pattern<V>,
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

    pub fn select(&mut self, dag: mir::Dag) -> Vec<mir::Node> {
        println!("initial dag: {:#?}", dag);
        let dag = dag.expand();
        println!("expanded dag: {:#?}", dag);

        dag.nodes.iter().flat_map(|node| self.select_node(node)).collect()
    }

    pub fn select_node(&mut self, node: &mir::Node) -> Vec<mir::Node> {
        let permutations = self.find_matching_permutations(node);

        match self::find_optimal_permutation(&permutations) {
            Some(permutation) => permutation.nodes.clone(),
            None => panic!("no patterns matching for this node: {:#?}", node),
        }
    }

    fn find_matching_permutations(&mut self, node: &mir::Node) -> Vec<Permutation<V>> {
        let similar_matches = self.find_similar_matches(node);

        similar_matches.into_iter().filter_map(|pat_match| {
            match pat_match.result {
                MatchResult::Perfect => {
                    Some(Permutation { nodes: vec![node.clone()], pattern: pat_match.pattern.clone() })
                },
                MatchResult::Partial(ref adjustments) => {
                    let nodes = Adjustment::apply_several_to(node.clone(), adjustments);

                    if pat_match.pattern.matches(nodes.last().unwrap()).is_perfect() {
                        Some(Permutation { nodes: nodes, pattern: pat_match.pattern.clone() })
                    } else {
                        None
                    }
                },
                MatchResult::None => unreachable!(),
            }
        }).collect()
    }

    fn find_similar_matches(&mut self, node: &mir::Node) -> Vec<MatchedPattern<V>> {
        self.patterns.iter().cloned().filter_map(|pattern| {
            let pat_match = MatchedPattern { node: node.clone(), result: pattern.matches(node), pattern: pattern };

            if pat_match.result.is_similar() { Some(pat_match) } else { None }
        }).collect()
    }
}

fn find_optimal_permutation<V>(permutations: &[Permutation<V>]) -> Option<&Permutation<V>>
    where V: PatternValue {
    permutations.iter().min_by_key(|permutation| permutation.pattern.root.area())
}


use {Pattern, PatternValue, MatchResult, Adjustment, AdjustmentApplication};
use mir;
use adjustment;

pub trait Selectable : ::std::fmt::Debug
{
}

/// Selects instructions.
pub struct Selector<S: Selectable + 'static, V: PatternValue>
{
    pub patterns: Vec<Pattern<S, V>>,
}

/// A pattern that matched with a node.
#[derive(Debug)]
pub struct MatchedPattern<S: Selectable + 'static, V: PatternValue>
{
    pub node: mir::Node,
    pub pattern: Pattern<S, V>,
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
pub struct Permutation<S: Selectable + 'static, V: PatternValue>
{
    /// The nodes of the permutation.
    pub nodes: Vec<mir::Node>,
    /// The pattern that was matched,
    pub pattern: Pattern<S, V>,
    /// Adjustments that need to be applied to the entire function.
    pub function_adjustments: Vec<Adjustment<V>>,
}

pub struct SelectedPermutation<V: PatternValue>
{
    /// The nodes of the permutation.
    pub nodes: Vec<mir::Node>,
    /// Adjustments that need to be applied to the entire function.
    pub function_adjustments: Vec<Adjustment<V>>,
}

impl<S: Selectable, V> Selector<S, V>
    where V: PatternValue
{
    /// Creates a new instruction selector.
    pub fn new(patterns: Vec<Pattern<S, V>>) -> Self {
        Selector {
            patterns: patterns,
        }
    }

    pub fn select(&mut self, dag: mir::Dag) -> Vec<S> {
        dag.expect_valid();

        let mut function_adjustments = Vec::new();

        let dag = dag.expand();
        let mut nodes: Vec<_> = dag.nodes.iter().flat_map(|node| {
            let permutation = self.select_node(node);
            function_adjustments.extend(permutation.function_adjustments);
            permutation.nodes
        }).collect();

        for adjustment in function_adjustments {
            match adjustment {
                Adjustment::CoerceValue { ref from, ref to } => {
                    nodes = nodes.into_iter().map(|node| adjustment::coerce_value(node, from, to)).collect();
                },
                _ => panic!("invalid function-level adjustment"),
            }
        }

        nodes.iter().map(|node| self.select_legal_node(&node)).collect()
    }

    pub fn select_node(&mut self, node: &mir::Node) -> SelectedPermutation<V> {
        let permutations = self.find_matching_permutations(node);

        match self::find_optimal_permutation(&permutations) {
            Some(permutation) => SelectedPermutation {
                nodes: permutation.nodes.clone(),
                function_adjustments: permutation.function_adjustments.clone(),
            },
            None => panic!("no patterns matching for this node: {:#?}", node),
        }
    }

    fn find_matching_permutations(&mut self, node: &mir::Node) -> Vec<Permutation<S, V>> {
        let similar_matches = self.find_similar_matches(node);

        similar_matches.into_iter().filter_map(|pat_match| {
            match pat_match.result {
                MatchResult::Perfect => {
                    Some(Permutation {
                        nodes: vec![node.clone()],
                        pattern: pat_match.pattern.clone(),
                        function_adjustments: Vec::new(),
                    })
                },
                MatchResult::Partial(mut adjustments) => {
                    let mut current_application = AdjustmentApplication::unadjusted(node.clone());

                    // Try to legalize the permutation.
                    for _ in 0..8 {
                        let new_application = Adjustment::apply_several_to(current_application.adjusted_node.clone(), &adjustments);

                        current_application = current_application.merge(new_application);

                        match pat_match.pattern.matches(&current_application.adjusted_node) {
                            MatchResult::Perfect => {
                                let mut new_function_adjustments = Vec::new();

                                current_application.preceding_nodes = current_application.preceding_nodes.into_iter().flat_map(|preceding_node| {
                                    let permutation = self.select_node(&preceding_node);
                                    new_function_adjustments.extend(permutation.function_adjustments);
                                    permutation.nodes
                                }).collect();

                                current_application.function_adjustments.extend(new_function_adjustments);

                                return Some(Permutation {
                                    nodes: current_application.nodes(),
                                    pattern: pat_match.pattern.clone(),
                                    function_adjustments: current_application.function_adjustments,
                                });
                            }
                            MatchResult::Partial(new_adjustments) => {
                                adjustments = new_adjustments;
                            },
                            MatchResult::None => return None,
                        }
                    }

                    None
                },
                MatchResult::None => unreachable!(),
            }
        }).collect()
    }

    fn find_similar_matches(&mut self, node: &mir::Node) -> Vec<MatchedPattern<S, V>> {
        // FIXME: check if there are register operands with the same id referenced twice
        // in this case, we need to add 'merge virt regs' adjustments

        self.patterns.iter().cloned().filter_map(|pattern| {
            let pat_match = MatchedPattern { node: node.clone(), result: pattern.matches(node), pattern: pattern };

            if pat_match.result.is_similar() { Some(pat_match) } else { None }
        }).collect()
    }

    /// Selects a node, under the guarantee that the node is already legal.
    fn select_legal_node(&mut self, node: &mir::Node) -> S {
        let matches: Vec<_> = self.find_similar_matches(node).into_iter().filter(|pat_match| {
            pat_match.result.is_perfect()
        }).collect();

        assert_eq!(matches.len(), 1);

        let ref pat_match = matches[0];
        (pat_match.pattern.factory)(node)
    }
}

fn find_optimal_permutation<S, V>(permutations: &[Permutation<S, V>]) -> Option<&Permutation<S, V>>
    where S: Selectable, V: PatternValue {
    permutations.iter().min_by_key(|permutation| permutation.pattern.root.area())
}


//! Graph traversal module providing stack-based walking of AST graphs.
//!
//! This module implements a depth-first traversal mechanism for graph structures
//! using the visitor pattern. The walker maintains a stack to process nodes
//! iteratively and delegates result accumulation to visitor callbacks that work
//! with a generic accumulator type.
//!
//! # Features
//!
//! * Stack-based traversal to avoid recursion and potential stack overflow
//! * Generic accumulator support for flexible result collection
//! * Visitor pattern implementation for extensible node processing
//! * Support for all AST node types including vertices, edges, rules, and contexts
//!
//! # Architecture
//!
//! The module consists of two main components:
//! - [`Walker`] - The main traversal engine that manages the iteration process
//! - [`WalkingStep`] - Internal representation of work items during traversal
//!
//! The walker processes nodes in depth-first order, pushing child nodes onto a stack
//! for later processing. This ensures that deeply nested graphs can be traversed
//! without hitting recursion limits.

use crate::ast::{
    Binding,
    GContext,
    GRuleAnon,
    GRuleNamed,
    GTensor,
    GVar,
    GVertex,
    Graph,
    GraphBinding,
};
use crate::visitor::Visitor;

/// Internal enumeration representing the different types of steps during graph traversal.
///
/// This enum is used internally by the walker to maintain a stack of work items,
/// allowing the traversal to handle both graph nodes and binding nodes uniformly.
/// The enum provides a unified interface for processing different node types
/// while maintaining type safety and avoiding dynamic dispatch overhead.
///
/// # Variants
///
/// * `Graph` - Contains a reference to a graph node that needs to be processed
/// * `Binding` - Contains a reference to a binding node (variable nominations)
pub enum WalkingStep<'a> {
    /// A graph node to be processed
    Graph(&'a Graph),
    /// A binding node to be processed
    Binding(&'a Binding),
}

/// A graph walker that traverses AST nodes using the visitor pattern.
///
/// The walker performs a depth-first traversal of the graph structure,
/// starting from a root graph node. It uses a stack-based approach to avoid
/// recursion and potential stack overflow issues with deeply nested graphs.
///
/// The walker is generic over the accumulator type, allowing different
/// visitors to collect results in whatever format they need. This design
/// enables flexible processing patterns such as code generation, analysis,
/// transformation, or validation.
///
/// # Type Parameters
///
/// * `'graph` - Lifetime of the graph being traversed
///
/// # Thread Safety
///
/// Walker is not thread-safe as it maintains mutable state during traversal.
/// However, multiple walkers can safely operate on the same graph concurrently
/// as long as the graph itself is not being modified.
///
/// # Performance
///
/// The stack-based approach provides consistent performance characteristics
/// regardless of graph nesting depth, making it suitable for processing
/// large or deeply nested graph structures.
///
/// # Examples
///
/// ```rust,ignore
/// use crate::walker::Walker;
/// use crate::visitor::Visitor;
///
/// let accumulator = MyAccumulator::new();
/// let visitor = MyVisitor::new();
/// let walker = Walker::new(&graph);
/// let result = walker.visit(visitor, accumulator);
/// // result now contains the traversal results
/// ```
pub struct Walker<'graph> {
    graph: &'graph Graph,
}

impl<'graph> Walker<'graph> {
    /// Performs the graph traversal, visiting each node with the provided visitor.
    ///
    /// This method processes nodes from the stack in LIFO order, calling the
    /// appropriate visitor method for each node type and updating the accumulator
    /// with the results. Child nodes are pushed onto the stack for later processing.
    ///
    /// The traversal is guaranteed to visit every reachable node exactly once,
    /// following a deterministic depth-first order. The visitor methods are called
    /// with the current accumulator state and must return an updated accumulator.
    ///
    /// # Type Parameters
    ///
    /// * `A` - The accumulator type that will be threaded through the traversal
    ///
    /// # Parameters
    ///
    /// * `visitor` - The visitor implementation that handles each node type
    /// * `initial_accumulator` - The initial accumulator value that will be threaded through the traversal
    ///
    /// # Returns
    ///
    /// The final accumulator value after all nodes have been visited
    ///
    /// # Node Processing Order
    ///
    /// The traversal follows these rules:
    /// - Nodes are processed from the stack in LIFO order (depth-first)
    /// - Child graphs are pushed to the stack for later processing
    /// - For composite nodes (edges, rules, etc.), children are processed in reverse order
    ///   to ensure left-to-right traversal when popped from the stack
    /// - Each node type delegates to the appropriate visitor method
    /// - Binding nodes are treated uniformly with graph nodes for consistent processing
    ///
    /// # Visitor Method Mapping
    ///
    /// Each graph node type maps to a specific visitor method:
    /// - `Graph::Nil` → `visit_nil`
    /// - `Graph::Vertex` → `visit_vertex`
    /// - `Graph::Var` → `visit_var`
    /// - `Graph::Nominate` → `visit_nominate`
    /// - `Graph::EdgeAnon` → `visit_edge_anon`
    /// - `Graph::EdgeNamed` → `visit_edge_named`
    /// - `Graph::RuleAnon` → `visit_rule_anon`
    /// - `Graph::RuleNamed` → `visit_rule_named`
    /// - `Graph::Subgraph` → `visit_subgraph`
    /// - `Graph::Tensor` → `visit_tensor`
    /// - `Graph::Context` → `visit_context`
    pub fn visit<A>(&self, visitor: impl Visitor<A>, initial_accumulator: A) -> A {
        let mut stack = vec![WalkingStep::Graph(self.graph)];

        let mut accumulator = initial_accumulator;

        while let Some(el) = stack.pop() {
            accumulator = match el {
                WalkingStep::Graph(Graph::Nil) => visitor.visit_nil(accumulator),
                WalkingStep::Graph(Graph::Vertex(GVertex { graph, vertex })) => {
                    stack.push(WalkingStep::Graph(graph));
                    visitor.visit_vertex(accumulator, vertex)
                }
                WalkingStep::Graph(Graph::Var(GVar { graph, var })) => {
                    stack.push(WalkingStep::Graph(graph));
                    visitor.visit_var(accumulator, var)
                }
                WalkingStep::Graph(Graph::Nominate(Binding { graph, var, vertex })) => {
                    stack.push(WalkingStep::Graph(graph));
                    visitor.visit_nominate(accumulator, var, vertex)
                }
                WalkingStep::Graph(Graph::EdgeAnon(edge)) => {
                    stack.push(WalkingStep::Binding(&edge.binding_2));
                    stack.push(WalkingStep::Binding(&edge.binding_1));
                    visitor.visit_edge_anon(accumulator, edge)
                }
                WalkingStep::Graph(Graph::EdgeNamed(gedge)) => {
                    stack.push(WalkingStep::Binding(&gedge.binding_2));
                    stack.push(WalkingStep::Binding(&gedge.binding_1));
                    visitor.visit_edge_named(accumulator, gedge)
                }
                WalkingStep::Graph(Graph::RuleAnon(GRuleAnon { graph_1, graph_2 })) => {
                    stack.push(WalkingStep::Graph(graph_2));
                    stack.push(WalkingStep::Graph(graph_1));
                    visitor.visit_rule_anon(accumulator, graph_1, graph_2)
                }
                WalkingStep::Graph(Graph::RuleNamed(GRuleNamed {
                    name,
                    graph_1,
                    graph_2,
                })) => {
                    stack.push(WalkingStep::Graph(graph_2));
                    stack.push(WalkingStep::Graph(graph_1));
                    visitor.visit_rule_named(accumulator, name, graph_1, graph_2)
                }
                WalkingStep::Graph(Graph::Subgraph(GraphBinding {
                    graph_1,
                    graph_2,
                    var,
                })) => {
                    stack.push(WalkingStep::Graph(graph_2));
                    stack.push(WalkingStep::Graph(graph_1));
                    visitor.visit_subgraph(accumulator, graph_1, graph_2, var)
                }
                WalkingStep::Graph(Graph::Tensor(GTensor { graph_1, graph_2 })) => {
                    stack.push(WalkingStep::Graph(graph_2));
                    stack.push(WalkingStep::Graph(graph_1));
                    visitor.visit_tensor(accumulator, graph_1, graph_2)
                }
                WalkingStep::Graph(Graph::Context(GContext {
                    graph,
                    name,
                    string,
                })) => {
                    stack.push(WalkingStep::Graph(graph));
                    visitor.visit_context(accumulator, name, string)
                }
                WalkingStep::Binding(Binding { graph, var, vertex }) => {
                    stack.push(WalkingStep::Graph(graph));
                    visitor.visit_nominate(accumulator, var, vertex)
                }
            };
        }

        accumulator
    }
}

impl<'a> Walker<'a> {
    /// Creates a new walker instance for traversing the given graph.
    ///
    /// The walker stores a reference to the root graph node and is ready
    /// to begin traversal when the `visit` method is called. This constructor
    /// is lightweight and performs no validation on the input graph.
    ///
    /// # Parameters
    ///
    /// * `graph` - Reference to the root graph node to traverse
    ///
    /// # Returns
    ///
    /// A new `Walker` instance ready to begin traversal
    ///
    /// # Lifetime
    ///
    /// The returned walker is bound to the lifetime of the input graph,
    /// ensuring memory safety during traversal operations.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let walker = Walker::new(&my_graph);
    /// let result = walker.visit(my_visitor, initial_accumulator);
    /// ```
    ///
    /// # Performance
    ///
    /// This constructor has O(1) time complexity as it only stores a reference
    /// to the graph without performing any preprocessing or validation.
    pub fn new(graph: &'a Graph) -> Self {
        Self { graph }
    }
}

#[cfg(test)]
mod test {

    use std::fmt::Display;

    use crate::ast::{GEdgeAnon, GEdgeNamed, Graph, Name, Vertex};
    use crate::bindings::psGraph;
    use crate::parse_to_ast;
    use crate::visitor::Visitor;
    use crate::walker::Walker;

    /// Test visitor implementation that generates XML-like output for graph nodes.
    ///
    /// This visitor is used in tests to verify that the walker correctly traverses
    /// the graph structure by producing a predictable string representation.
    /// The XML format makes it easy to verify nesting and ordering of node visits.
    ///
    /// # Output Format
    ///
    /// The visitor generates opening and closing XML tags for each node type,
    /// creating a hierarchical representation that mirrors the graph structure.
    /// Self-closing tags are used for leaf nodes like `nil`.
    struct TestVisitor {}

    /// Test accumulator that collects opening and closing XML-like tags.
    ///
    /// The accumulator maintains separate vectors for opening tags (processed in order)
    /// and closing tags (processed in reverse order) to create properly nested output.
    /// This design allows the walker to build the output incrementally while maintaining
    /// correct XML structure.
    ///
    /// # Fields
    ///
    /// * `left` - Opening tags collected during traversal
    /// * `right` - Closing tags collected during traversal (displayed in reverse)
    ///
    /// # Display Behavior
    ///
    /// When displayed, the accumulator outputs all opening tags followed by
    /// all closing tags in reverse order, creating properly nested XML.
    #[derive(Debug, Clone, Default)]
    struct TestAccumulator {
        left: Vec<String>,
        right: Vec<String>,
    }

    impl TestAccumulator {
        /// Creates a new accumulator with an additional opening tag.
        ///
        /// This method is used by visitor methods to add opening XML tags
        /// to the accumulator during traversal. The method preserves immutability
        /// by returning a new accumulator instance.
        ///
        /// # Parameters
        ///
        /// * `left` - The opening tag string to add
        ///
        /// # Returns
        ///
        /// A new TestAccumulator with the tag added to the left (opening) side
        fn with_left(&self, left: &str) -> Self {
            let mut left_temp = self.left.clone();
            left_temp.push(left.to_string());

            Self {
                left: left_temp,
                ..self.clone()
            }
        }

        /// Creates a new accumulator with an additional closing tag.
        ///
        /// This method is used by visitor methods to add closing XML tags
        /// to the accumulator during traversal. The method preserves immutability
        /// by returning a new accumulator instance.
        ///
        /// # Parameters
        ///
        /// * `right` - The closing tag string to add
        ///
        /// # Returns
        ///
        /// A new TestAccumulator with the tag added to the right (closing) side
        fn with_right(&self, right: &str) -> Self {
            let mut right_temp = self.right.clone();
            right_temp.push(right.to_string());

            Self {
                right: right_temp,
                ..self.clone()
            }
        }
    }

    impl Visitor<TestAccumulator> for TestVisitor {
        fn visit_nil(&self, acc: TestAccumulator) -> TestAccumulator {
            acc.with_left("<nil/>\n").with_right("")
        }

        fn visit_vertex(&self, acc: TestAccumulator, vertex: &Vertex) -> TestAccumulator {
            acc.with_left(&format!(
                "<vertex {}>\n",
                match &vertex.name {
                    Name::VVar { value } => value,
                    _ => unreachable!(),
                }
            ))
            .with_right("</vertex>\n")
        }

        fn visit_var(&self, acc: TestAccumulator, var: &str) -> TestAccumulator {
            acc.with_left(&format!("<var {}>\n", var))
                .with_right("</var>\n")
        }

        fn visit_nominate(
            &self,
            acc: TestAccumulator,
            name: &str,
            vertex: &Vertex,
        ) -> TestAccumulator {
            acc.with_left(&format!(
                "<nominate {name} for vertex {vertex_name}>\n",
                vertex_name = match &vertex.name {
                    Name::VVar { value } => value,
                    _ => unreachable!(),
                }
            ))
            .with_right("</nominate>\n")
        }

        fn visit_edge_named(&self, _acc: TestAccumulator, _edge: &GEdgeNamed) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_rule_anon(
            &self,
            _acc: TestAccumulator,
            _graph: &Graph,
            _graph2: &Graph,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_rule_named(
            &self,
            _acc: TestAccumulator,
            _name: &Name,
            _graph: &Graph,
            _graph2: &Graph,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_subgraph(
            &self,
            _acc: TestAccumulator,
            _graph: &Graph,
            _graph2: &Graph,
            _identifier: &str,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_tensor(
            &self,
            _acc: TestAccumulator,
            _graph: &Graph,
            _graph2: &Graph,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_context(
            &self,
            acc: TestAccumulator,
            name: &Name,
            context: &str,
        ) -> TestAccumulator {
            acc.with_left(&format!(
                "<context for {name} with {context}>\n",
                name = match name {
                    Name::VVar { value } => value,
                    _ => unreachable!(),
                }
            ))
            .with_right("</context>\n")
        }

        fn visit_edge_anon(&self, acc: TestAccumulator, _edge: &GEdgeAnon) -> TestAccumulator {
            acc.with_left("<edge>\n").with_right("</edge>\n")
        }
    }

    /// Creates a new test visitor instance.
    ///
    /// This factory function provides a consistent way to create visitor
    /// instances for testing purposes.
    fn create_visitor() -> TestVisitor {
        TestVisitor {}
    }

    impl Display for TestAccumulator {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // Write opening tags
            for open in &self.left {
                write!(f, "{}", open)?;
            }
            // Write closing tags in reverse order
            for close in self.right.iter().rev() {
                write!(f, "{}", close)?;
            }
            Ok(())
        }
    }

    /// Creates a new test accumulator instance.
    ///
    /// This factory function provides a consistent way to create accumulator
    /// instances for testing purposes.
    fn create_accumulator() -> TestAccumulator {
        TestAccumulator::default()
    }

    /// Tests walker behavior with a nil graph node.
    ///
    /// Verifies that the walker correctly calls the visit_nil callback
    /// and returns the expected string result. This test validates the
    /// most basic traversal case.
    #[test]
    fn test_gnil_visitor() {
        let graph: Graph = unsafe { psGraph(c"{0}".as_ptr()) }.try_into().unwrap();
        let visitor = create_visitor();
        let walker = Walker::new(&graph);
        let accumulator = walker.visit(visitor, create_accumulator());

        assert_eq!(&accumulator.to_string(), "<nil/>\n");
    }

    /// Tests walker behavior with a nomination (let binding) structure.
    ///
    /// Verifies that the walker correctly processes a let binding that nominates
    /// a variable for a vertex, followed by the vertex usage and nil termination.
    /// This test validates variable binding and reference handling.
    #[test]
    fn test_nomination_visitor() {
        let graph = parse_to_ast("let a = <a> in <a> | 0".into()).unwrap();
        let visitor = create_visitor();

        let walker = Walker::new(&graph);
        let accumulator = walker.visit(visitor, create_accumulator());

        assert_eq!(
            &accumulator.to_string(),
            "<nominate a for vertex a>\n<vertex a>\n<nil/>\n</vertex>\n</nominate>\n"
        );
    }

    /// Tests walker behavior with an edge containing two bindings.
    ///
    /// Verifies that the walker correctly processes an edge structure with
    /// two let bindings, each containing a vertex and nil continuation.
    /// This test validates composite node traversal and child ordering.
    #[test]
    fn test_edge_visitor() {
        let graph: Graph =
            parse_to_ast("(let a = <a> in <a> | 0, let b = <b> in <b> | 0)".into()).unwrap();
        let visitor = create_visitor();
        let walker = Walker::new(&graph);
        let accumulator = walker.visit(visitor, create_accumulator());

        assert_eq!(
            &accumulator.to_string(),
            r#"<edge>
<nominate a for vertex a>
<vertex a>
<nil/>
<nominate b for vertex b>
<vertex b>
<nil/>
</vertex>
</nominate>
</vertex>
</nominate>
</edge>
"#
        );
    }

    /// Tests walker behavior with a vertex graph node.
    ///
    /// Verifies that the walker processes both the vertex and its continuation (nil),
    /// calling the appropriate visitor methods in the correct order.
    /// This test validates basic vertex node processing.
    #[test]
    fn test_vertex_visitor() {
        let graph = parse_to_ast("<a> | 0".into()).unwrap();
        let visitor = create_visitor();

        let walker = Walker::new(&graph);
        let accumulator = walker.visit(visitor, create_accumulator());

        assert_eq!(&accumulator.to_string(), "<vertex a>\n<nil/>\n</vertex>\n");
    }

    /// Tests walker behavior with an anonymous edge containing two bindings.
    ///
    /// Verifies that the walker correctly processes the edge structure,
    /// visiting both bindings and their associated vertices and continuations.
    /// This test validates anonymous edge processing and nested structures.
    #[test]
    fn test_annonim_edge_visitor() {
        let graph =
            parse_to_ast("{ (let va = <a> in <a> | 0, let vb = <b> in <b> | 0) }".into()).unwrap();
        let visitor = create_visitor();

        let walker = Walker::new(&graph);
        let accumulator = walker.visit(visitor, create_accumulator());

        assert_eq!(
            &accumulator.to_string(),
            r#"<edge>
<nominate va for vertex a>
<vertex a>
<nil/>
<nominate vb for vertex b>
<vertex b>
<nil/>
</vertex>
</nominate>
</vertex>
</nominate>
</edge>
"#
        );
    }

    /// Tests walker behavior with a complex nested graph structure.
    ///
    /// This test uses a linear graph with three edges and various node types
    /// including vertices, variables, and nested bindings. It verifies that
    /// the walker processes all nodes in the correct depth-first order.
    /// This comprehensive test validates complex traversal scenarios.
    #[test]
    fn test_linear_graph_with_3_edges() {
        let graph: Graph = parse_to_ast(
            "{
                    (
                      let n2 = <notification> in {
                        (
                          let e2 = <encryption> in {
                            (
                              let e1 = <encryption> in <encryption> | 0,
                              let s = <store> in <store> | 0
                            )
                          } ,
                          let n1 = <notification> in <notification> | 0
                        )
                      },
                      let e3 = <encryption> in e1 | 0
                    )
                  }"
            .into(),
        )
        .unwrap();
        let visitor = create_visitor();
        let walker = Walker::new(&graph);
        let accumulator = walker.visit(visitor, create_accumulator());

        assert_eq!(
            &accumulator.to_string(),
            r#"<edge>
<nominate n2 for vertex notification>
<edge>
<nominate e2 for vertex encryption>
<edge>
<nominate e1 for vertex encryption>
<vertex encryption>
<nil/>
<nominate s for vertex store>
<vertex store>
<nil/>
<nominate n1 for vertex notification>
<vertex notification>
<nil/>
<nominate e3 for vertex encryption>
<var e1>
<nil/>
</var>
</nominate>
</vertex>
</nominate>
</vertex>
</nominate>
</vertex>
</nominate>
</edge>
</nominate>
</edge>
</nominate>
</edge>
"#
        );
    }

    /// Tests walker behavior with a context node.
    ///
    /// Verifies that the walker correctly processes a context node that provides
    /// additional information for a named vertex. This test validates context
    /// node processing and metadata handling.
    #[test]
    fn test_vertext_context() {
        let graph = parse_to_ast("context \"foo=bar\" for a in <a> | {0}".into()).unwrap();
        let visitor = create_visitor();

        let walker = Walker::new(&graph);
        let accumulator = walker.visit(visitor, create_accumulator());

        assert_eq!(
            &accumulator.to_string(),
            r#"<context for a with foo=bar>
<vertex a>
<nil/>
</vertex>
</context>
"#
        );
    }
}

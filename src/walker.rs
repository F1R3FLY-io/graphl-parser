//! Graph traversal module providing iterator-based walking of AST graphs.
//!
//! This module implements a depth-first traversal mechanism for graph structures
//! using the visitor pattern. The walker maintains a stack to process nodes
//! iteratively and accumulates results from visitor callbacks.

#![allow(dead_code)]
use crate::ast::{Binding, GVertex, Graph};
use crate::visitor::Visitor;

/// A graph walker that traverses AST nodes using the visitor pattern.
///
/// The walker performs a depth-first traversal of the graph structure,
/// maintaining a stack of nodes to visit and accumulating string results
/// from visitor method calls.
///
/// # Type Parameters
///
/// * `'graph` - Lifetime of the graph being traversed
/// * `'visitor` - Lifetime of the visitor instance
///
/// # Examples
///
/// ```
/// let mut visitor = Visitor::default();
/// let mut walker = Walker::new(&graph, &mut visitor);
/// let result = walker.visit();
/// ```
pub struct Walker<'graph, 'visitor, A>
where
    A: Clone,
{
    /// Mutable reference to the visitor handling node callbacks
    visitor: &'visitor dyn Visitor<A>,
    /// Stack of graph nodes to be processed (LIFO order)
    stack: Vec<Graph>,
    /// String accumulator for collecting visitor results
    accumulator: &'graph mut A,
}

impl<'graph, 'visitor, A> Walker<'graph, 'visitor, A>
where
    A: Clone,
{
    /// Performs the graph traversal, visiting each node with the configured visitor.
    ///
    /// This method processes nodes from the stack in LIFO order, calling the
    /// appropriate visitor method for each node type and accumulating the results.
    /// Child nodes are pushed onto the stack for later processing.
    ///
    /// # Returns
    ///
    /// A `String` containing the accumulated results from all visitor method calls.
    ///
    /// # Node Processing Order
    ///
    /// The traversal follows these rules:
    /// - Nodes are processed from the stack in LIFO order
    /// - Child graphs are pushed to the stack for later processing
    /// - For composite nodes (edges, rules, etc.), children are processed in reverse order
    pub fn visit(&mut self) {
        while let Some(el) = self.stack.pop() {
            *self.accumulator = match el {
                Graph::Nil => self.visitor.visit_nil(self.accumulator),
                Graph::Vertex(GVertex { graph, vertex }) => {
                    self.stack.push(*graph);
                    self.visitor.visit_vertex(self.accumulator, &vertex)
                }
                Graph::Var(gvar) => {
                    self.stack.push(*gvar.graph);
                    self.visitor.visit_var(self.accumulator, &gvar.var)
                }
                Graph::Nominate(binding) => {
                    self.stack.push(*binding.graph);
                    self.visitor
                        .visit_nominate(self.accumulator, &binding.var, &binding.vertex)
                }
                Graph::EdgeAnon(gedge_anon) => {
                    let gedged_clone = gedge_anon.clone();
                    self.stack.push(Graph::Nominate(Binding {
                        var: gedge_anon.binding_2.var,
                        vertex: gedge_anon.binding_2.vertex,
                        graph: gedge_anon.binding_2.graph,
                    }));
                    self.stack.push(Graph::Nominate(Binding {
                        var: gedge_anon.binding_1.var,
                        vertex: gedge_anon.binding_1.vertex,
                        graph: gedge_anon.binding_1.graph,
                    }));

                    self.visitor
                        .visit_edge_anon(self.accumulator, &gedged_clone)
                }
                Graph::EdgeNamed(gedge_named) => {
                    let gedged_clone = gedge_named.clone();
                    self.stack.push(Graph::Nominate(Binding {
                        var: gedge_named.binding_2.var,
                        vertex: gedge_named.binding_2.vertex,
                        graph: gedge_named.binding_2.graph,
                    }));
                    self.stack.push(Graph::Nominate(Binding {
                        var: gedge_named.binding_1.var,
                        vertex: gedge_named.binding_1.vertex,
                        graph: gedge_named.binding_1.graph,
                    }));

                    self.visitor
                        .visit_edge_named(self.accumulator, &gedged_clone)
                }
                Graph::RuleAnon(grule_anon) => {
                    self.stack.push(*grule_anon.graph_2.clone());
                    self.stack.push(*grule_anon.graph_1.clone());
                    self.visitor.visit_rule_anon(
                        self.accumulator,
                        &grule_anon.graph_1,
                        &grule_anon.graph_2,
                    )
                }
                Graph::RuleNamed(grule_named) => {
                    self.stack.push(*grule_named.graph_2.clone());
                    self.stack.push(*grule_named.graph_1.clone());
                    self.visitor.visit_rule_named(
                        self.accumulator,
                        &grule_named.name,
                        &grule_named.graph_1,
                        &grule_named.graph_2,
                    )
                }
                Graph::Subgraph(graph_binding) => {
                    self.stack.push(*graph_binding.graph_2.clone());
                    self.stack.push(*graph_binding.graph_1.clone());
                    self.visitor.visit_subgraph(
                        self.accumulator,
                        &graph_binding.graph_1,
                        &graph_binding.graph_2,
                        &graph_binding.var,
                    )
                }
                Graph::Tensor(gtensor) => {
                    self.stack.push(*gtensor.graph_2.clone());
                    self.stack.push(*gtensor.graph_1.clone());
                    self.visitor
                        .visit_tensor(self.accumulator, &gtensor.graph_1, &gtensor.graph_2)
                }
                Graph::Context(gcontext) => {
                    self.stack.push(*gcontext.graph);
                    self.visitor
                        .visit_context(self.accumulator, &gcontext.name, &gcontext.string)
                }
            };
        }
    }
}

impl<'graph, 'visitor, A> Walker<'graph, 'visitor, A>
where
    'visitor: 'graph,
    A: Clone,
{
    /// Creates a new walker instance for traversing the given graph.
    ///
    /// The walker is initialized with the root graph node placed on the stack
    /// and an empty string accumulator.
    ///
    /// # Parameters
    ///
    /// * `graph` - Reference to the root graph node to traverse
    /// * `visitor` - Mutable reference to the visitor for handling node callbacks
    ///
    /// # Returns
    ///
    /// A new `Walker` instance ready to begin traversal.
    pub fn new(
        graph: &'graph Graph,
        visitor: &'visitor impl Visitor<A>,
        accumulator: &'graph mut A,
    ) -> Self {
        Self {
            visitor,
            stack: vec![graph.clone()],
            accumulator,
        }
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

    type OpenClosePair = (String, String);
    struct TestVisitor {}

    #[derive(Debug, Clone, Default)]
    struct TestAccumulator {
        left: Vec<String>,
        right: Vec<String>,
    }

    impl TestAccumulator {
        fn with_left(&self, left: &str) -> Self {
            let mut left_temp = self.left.clone();
            left_temp.push(left.to_string());

            Self {
                left: left_temp,
                ..self.clone()
            }
        }

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
        fn visit_nil(&self, acc: &TestAccumulator) -> TestAccumulator {
            acc.with_left("<nil/>\n").with_right("")
        }

        fn visit_vertex(&self, acc: &TestAccumulator, vertex: &Vertex) -> TestAccumulator {
            acc.with_left(&format!(
                "<vertex {}>\n",
                match &vertex.name {
                    Name::VVar { value } => value,
                    _ => unreachable!(),
                }
            ))
            .with_right("</vertex>\n")
        }

        fn visit_var(&self, acc: &TestAccumulator, var: &str) -> TestAccumulator {
            acc.with_left(&format!("<var {}>\n", var))
                .with_right("</var>\n")
        }

        fn visit_nominate(
            &self,
            acc: &TestAccumulator,
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

        fn visit_edge_named(&self, _acc: &TestAccumulator, _edge: &GEdgeNamed) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_rule_anon(
            &self,
            _acc: &TestAccumulator,
            _graph: &Graph,
            _graph2: &Graph,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_rule_named(
            &self,
            acc: &TestAccumulator,
            _name: &Name,
            _graph: &Graph,
            _graph2: &Graph,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_subgraph(
            &self,
            acc: &TestAccumulator,
            _graph: &Graph,
            _graph2: &Graph,
            _identifier: &str,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_tensor(
            &self,
            acc: &TestAccumulator,
            _graph: &Graph,
            _graph2: &Graph,
        ) -> TestAccumulator {
            unimplemented!()
        }

        fn visit_context(
            &self,
            acc: &TestAccumulator,
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

        fn visit_edge_anon(&self, acc: &TestAccumulator, _edge: &GEdgeAnon) -> TestAccumulator {
            acc.with_left(&format!("<edge>\n"))
                .with_right(&format!("</edge>\n"))
        }
    }

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

    fn create_accumulator() -> TestAccumulator {
        TestAccumulator::default()
    }

    /// Tests walker behavior with a nil graph node.
    ///
    /// Verifies that the walker correctly calls the visit_nil callback
    /// and returns the expected string result.
    #[test]
    fn test_gnil_visitor() {
        let graph: Graph = unsafe { psGraph(c"{0}".as_ptr()) }.try_into().unwrap();
        let visitor = create_visitor();
        let mut accumulator = create_accumulator();
        let mut walker = Walker::new(&graph, &visitor, &mut accumulator);
        walker.visit();

        assert_eq!(&accumulator.to_string(), "<nil/>\n");
    }

    #[test]
    fn test_nomination_visitor() {
        let graph = parse_to_ast("let a = <a> in <a> | 0".into()).unwrap();
        let visitor = create_visitor();
        let mut accumulator = create_accumulator();

        let mut walker = Walker::new(&graph, &visitor, &mut accumulator);
        walker.visit();

        assert_eq!(
            &accumulator.to_string(),
            "<nominate a for vertex a>\n<vertex a>\n<nil/>\n</vertex>\n</nominate>\n"
        );
    }

    #[test]
    fn test_edge_visitor() {
        let graph: Graph =
            parse_to_ast("(let a = <a> in <a> | 0, let b = <b> in <b> | 0)".into()).unwrap();
        let visitor = create_visitor();
        let mut accumulator = create_accumulator();

        let mut walker = Walker::new(&graph, &visitor, &mut accumulator);
        walker.visit();

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
    #[test]
    fn test_vertex_visitor() {
        let graph = parse_to_ast("<a> | 0".into()).unwrap();
        let visitor = create_visitor();
        let mut accumulator = create_accumulator();

        let mut walker = Walker::new(&graph, &visitor, &mut accumulator);
        walker.visit();

        assert_eq!(&accumulator.to_string(), "<vertex a>\n<nil/>\n</vertex>\n");
    }

    /// Tests walker behavior with an anonymous edge containing two bindings.
    ///
    /// Verifies that the walker correctly processes the edge structure,
    /// visiting both bindings and their associated vertices and continuations.
    #[test]
    fn test_annonim_edge_visitor() {
        let graph =
            parse_to_ast("{ (let va = <a> in <a> | 0, let vb = <b> in <b> | 0) }".into()).unwrap();
        let visitor = create_visitor();
        let mut accumulator = create_accumulator();

        let mut walker = Walker::new(&graph, &visitor, &mut accumulator);
        walker.visit();

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
        let mut accumulator = create_accumulator();

        let mut walker = Walker::new(&graph, &visitor, &mut accumulator);
        walker.visit();

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

    #[test]
    fn test_vertext_context() {
        let graph = parse_to_ast("context \"foo=bar\" for a in <a> | {0}".into()).unwrap();
        let visitor = create_visitor();
        let mut accumulator = create_accumulator();

        let mut walker = Walker::new(&graph, &visitor, &mut accumulator);
        walker.visit();

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

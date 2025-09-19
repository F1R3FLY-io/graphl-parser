//! Graph traversal module providing iterator-based walking of AST graphs.
//!
//! This module implements a depth-first traversal mechanism for graph structures
//! using the visitor pattern. The walker maintains a stack to process nodes
//! iteratively and accumulates results from visitor callbacks.

#![allow(dead_code)]
use crate::ast::{GVertex, Graph};
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
pub struct Walker<'graph, 'visitor, T> {
    /// Reference to the graph being traversed
    graph: &'graph Graph,
    /// Mutable reference to the visitor handling node callbacks
    visitor: &'visitor dyn Visitor<T>,
    /// Stack of graph nodes to be processed (LIFO order)
    stack: Vec<&'graph Graph>,
    /// String accumulator for collecting visitor results
    accumulator: Vec<String>,
    accumulator_2: Vec<String>,
}

impl<'graph, 'visitor> Walker<'graph, 'visitor, (String, String)> {
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
    pub fn visit(&mut self) -> String {
        while let Some(el) = self.stack.pop() {
            let (open, close) = match el {
                Graph::Nil => self.visitor.visit_nil(),
                Graph::Vertex(GVertex { graph, vertex }) => {
                    self.stack.push(graph);
                    self.visitor.visit_vertex(vertex)
                }
                Graph::Var(gvar) => {
                    self.stack.push(&gvar.graph);
                    self.visitor.visit_var(&gvar.var)
                }
                Graph::Nominate(binding) => {
                    self.stack.push(&binding.graph);
                    self.visitor.visit_nominate(&binding.var, &binding.vertex)
                }
                Graph::EdgeAnon(gedge_anon) => {
                    self.stack.push(&gedge_anon.binding_2.graph);
                    self.stack.push(&gedge_anon.binding_1.graph);
                    let nomination_1 = self
                        .visitor
                        .visit_nominate(&gedge_anon.binding_1.var, &gedge_anon.binding_1.vertex);
                    let nomination_2 = self
                        .visitor
                        .visit_nominate(&gedge_anon.binding_2.var, &gedge_anon.binding_2.vertex);
                    self.visitor
                        .visit_edge_anon(gedge_anon, nomination_1, nomination_2)
                }
                Graph::EdgeNamed(gedge_named) => {
                    self.stack.push(&gedge_named.binding_2.graph);
                    self.stack.push(&gedge_named.binding_1.graph);
                    let nomination_1 = self
                        .visitor
                        .visit_nominate(&gedge_named.binding_1.var, &gedge_named.binding_1.vertex);
                    let nomination_2 = self
                        .visitor
                        .visit_nominate(&gedge_named.binding_2.var, &gedge_named.binding_2.vertex);
                    self.visitor
                        .visit_edge_named(gedge_named, nomination_1, nomination_2)
                }
                Graph::RuleAnon(grule_anon) => {
                    self.stack.push(&grule_anon.graph_2);
                    self.stack.push(&grule_anon.graph_1);
                    self.visitor
                        .visit_rule_anon(&grule_anon.graph_1, &grule_anon.graph_2)
                }
                Graph::RuleNamed(grule_named) => {
                    self.stack.push(&grule_named.graph_2);
                    self.stack.push(&grule_named.graph_1);
                    self.visitor.visit_rule_named(
                        &grule_named.name,
                        &grule_named.graph_1,
                        &grule_named.graph_2,
                    )
                }
                Graph::Subgraph(graph_binding) => {
                    self.stack.push(&graph_binding.graph_2);
                    self.stack.push(&graph_binding.graph_1);
                    self.visitor.visit_subgraph(
                        &graph_binding.graph_1,
                        &graph_binding.graph_2,
                        &graph_binding.var,
                    )
                }
                Graph::Tensor(gtensor) => {
                    self.stack.push(&gtensor.graph_2);
                    self.stack.push(&gtensor.graph_1);
                    self.visitor
                        .visit_tensor(&gtensor.graph_1, &gtensor.graph_2)
                }
                Graph::Context(gcontext) => {
                    self.stack.push(&gcontext.graph);
                    self.visitor.visit_context(&gcontext.name, &gcontext.string)
                }
            };

            self.accumulator.push(open.to_owned());
            self.accumulator_2.push(close.to_owned());
        }

        self.accumulator_2.reverse();

        format!(
            "{}{}",
            self.accumulator.join(""),
            self.accumulator_2.join("")
        )
    }
}

impl<'graph, 'visitor> Walker<'graph, 'visitor, (String, String)> {
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
    pub fn new(graph: &'graph Graph, visitor: &'visitor impl Visitor<(String, String)>) -> Self {
        Self {
            graph,
            visitor,
            stack: vec![graph],
            accumulator: vec![],
            accumulator_2: vec![],
        }
    }
}

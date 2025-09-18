use crate::ast::{GEdgeAnon, GEdgeNamed, Graph, Name, Vertex};

pub type OpenClosePair = (String, String);

/// A visitor pattern implementation for traversing and transforming graph AST nodes.
///
/// The `Visitor` struct contains function pointers for handling different types of AST nodes,
/// allowing for flexible traversal and transformation of graph structures. Each visitor function
/// returns a `String` representation of the processed node.
pub struct Visitor {
    /// Handles nil/empty nodes
    pub(crate) visit_nil: Box<dyn FnMut() -> OpenClosePair>,
    /// Handles vertex nodes
    pub(crate) visit_vertex: Box<dyn FnMut(&Vertex) -> OpenClosePair>,
    /// Handles variable references
    pub(crate) visit_var: Box<dyn FnMut(&str) -> OpenClosePair>,
    /// Handles nomination of vertices with a name
    pub(crate) visit_nominate: Box<dyn FnMut(&str, &Vertex) -> OpenClosePair>,
    /// Handles anonymous graph edges
    pub(crate) visit_edge_anon: Box<dyn FnMut(&GEdgeAnon) -> OpenClosePair>,
    /// Handles named graph edges
    pub(crate) visit_edge_named: Box<dyn FnMut(&GEdgeNamed) -> OpenClosePair>,
    /// Handles anonymous graph rewrite rules (left-hand side, right-hand side)
    pub(crate) visit_rule_anon: Box<dyn FnMut(&Graph, &Graph) -> OpenClosePair>,
    /// Handles named graph rewrite rules with a name and left/right-hand sides
    pub(crate) visit_rule_named: Box<dyn FnMut(&Name, &Graph, &Graph) -> OpenClosePair>,
    /// Handles subgraphs with parent graph, subgraph, and identifier
    pub(crate) visit_subgraph: Box<dyn FnMut(&Graph, &Graph, &str) -> OpenClosePair>,
    /// Handles tensor products of two graphs
    pub(crate) visit_tensor: Box<dyn FnMut(&Graph, &Graph) -> OpenClosePair>,
    /// Handles context nodes with name and context string
    pub(crate) visit_context: Box<dyn FnMut(&Name, &str) -> OpenClosePair>,
}

impl Default for Visitor {
    /// Creates a new `Visitor` with all visitor functions set to unimplemented defaults.
    ///
    /// This allows for selective implementation of only the visitor functions that are needed
    /// for a particular use case, while providing clear error messages for unimplemented visitors.
    fn default() -> Self {
        Self {
            visit_nil: Box::new(|| unimplemented!("unimplemented visit_nil")),
            visit_vertex: Box::new(|_| unimplemented!("unimplemented visit_vertex")),
            visit_var: Box::new(|_| unimplemented!("unimplemented visit_var")),
            visit_nominate: Box::new(|_, _| unimplemented!("unimplemented visit_nominate")),
            visit_edge_anon: Box::new(|_| unimplemented!("unimplemented visit_edge_anon")),
            visit_edge_named: Box::new(|_| unimplemented!("unimplemented visit_edge_named")),
            visit_rule_anon: Box::new(|_, _| unimplemented!("unimplemented visit_rule_anon")),
            visit_rule_named: Box::new(|_, _, _| unimplemented!("unimplemented visit_rule_named")),
            visit_subgraph: Box::new(|_, _, _| unimplemented!("unimplemented visit_subgraph")),
            visit_tensor: Box::new(|_, _| unimplemented!("unimplemented visit_tensor")),
            visit_context: Box::new(|_, _| unimplemented!("unimplemented visit_context")),
        }
    }
}

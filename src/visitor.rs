use crate::ast::{GEdgeAnon, GEdgeNamed, Graph, Name, Vertex};

pub type OpenClosePair = (String, String);

// Type aliases for complex visitor function types
type VisitNilFn = Box<dyn Fn() -> OpenClosePair>;
type VisitVertexFn = Box<dyn Fn(&Vertex) -> OpenClosePair>;
type VisitVarFn = Box<dyn Fn(&str) -> OpenClosePair>;
type VisitNominateFn = Box<dyn Fn(&str, &Vertex) -> OpenClosePair>;
type VisitEdgeAnonFn = Box<dyn Fn(&GEdgeAnon, OpenClosePair, OpenClosePair) -> OpenClosePair>;
type VisitEdgeNamedFn = Box<dyn Fn(&GEdgeNamed, OpenClosePair, OpenClosePair) -> OpenClosePair>;
type VisitRuleAnonFn = Box<dyn Fn(&Graph, &Graph) -> OpenClosePair>;
type VisitRuleNamedFn = Box<dyn Fn(&Name, &Graph, &Graph) -> OpenClosePair>;
type VisitSubgraphFn = Box<dyn Fn(&Graph, &Graph, &str) -> OpenClosePair>;
type VisitTensorFn = Box<dyn Fn(&Graph, &Graph) -> OpenClosePair>;
type VisitContextFn = Box<dyn Fn(&Name, &str) -> OpenClosePair>;

/// A visitor pattern implementation for traversing and transforming graph AST nodes.
///
/// The `Visitor` struct contains function pointers for handling different types of AST nodes,
/// allowing for flexible traversal and transformation of graph structures. Each visitor function
/// returns a `String` representation of the processed node.
pub struct Visitor {
    /// Handles nil/empty nodes
    pub(crate) visit_nil: VisitNilFn,
    /// Handles vertex nodes
    pub(crate) visit_vertex: VisitVertexFn,
    /// Handles variable references
    pub(crate) visit_var: VisitVarFn,
    /// Handles nomination of vertices with a name
    pub(crate) visit_nominate: VisitNominateFn,
    /// Handles anonymous graph edges
    pub(crate) visit_edge_anon: VisitEdgeAnonFn,
    /// Handles named graph edges
    pub(crate) visit_edge_named: VisitEdgeNamedFn,
    /// Handles anonymous graph rewrite rules (left-hand side, right-hand side)
    pub(crate) visit_rule_anon: VisitRuleAnonFn,
    /// Handles named graph rewrite rules with a name and left/right-hand sides
    pub(crate) visit_rule_named: VisitRuleNamedFn,
    /// Handles subgraphs with parent graph, subgraph, and identifier
    pub(crate) visit_subgraph: VisitSubgraphFn,
    /// Handles tensor products of two graphs
    pub(crate) visit_tensor: VisitTensorFn,
    /// Handles context nodes with name and context string
    pub(crate) visit_context: VisitContextFn,
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
            visit_edge_anon: Box::new(|_, _, _| unimplemented!("unimplemented visit_edge_anon")),
            visit_edge_named: Box::new(|_, _, _| unimplemented!("unimplemented visit_edge_named")),
            visit_rule_anon: Box::new(|_, _| unimplemented!("unimplemented visit_rule_anon")),
            visit_rule_named: Box::new(|_, _, _| unimplemented!("unimplemented visit_rule_named")),
            visit_subgraph: Box::new(|_, _, _| unimplemented!("unimplemented visit_subgraph")),
            visit_tensor: Box::new(|_, _| unimplemented!("unimplemented visit_tensor")),
            visit_context: Box::new(|_, _| unimplemented!("unimplemented visit_context")),
        }
    }
}

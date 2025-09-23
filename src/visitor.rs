use crate::ast::{GEdgeAnon, GEdgeNamed, Graph, Name, Vertex};

pub trait Visitor<A> {
    fn visit_nil(&self, acc: A) -> A;
    fn visit_vertex(&self, acc: A, vertex: &Vertex) -> A;
    fn visit_var(&self, acc: A, var: &str) -> A;
    fn visit_nominate(&self, acc: A, name: &str, vertex: &Vertex) -> A;
    fn visit_edge_anon(&self, acc: A, edge: &GEdgeAnon) -> A;
    fn visit_edge_named(&self, acc: A, edge: &GEdgeNamed) -> A;
    fn visit_rule_anon(&self, acc: A, graph: &Graph, graph2: &Graph) -> A;
    fn visit_rule_named(&self, acc: A, name: &Name, graph: &Graph, graph2: &Graph) -> A;
    fn visit_subgraph(&self, acc: A, graph: &Graph, graph2: &Graph, identifier: &str) -> A;
    fn visit_tensor(&self, acc: A, graph: &Graph, graph2: &Graph) -> A;
    fn visit_context(&self, acc: A, name: &Name, context: &str) -> A;
}

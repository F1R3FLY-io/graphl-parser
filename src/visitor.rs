use crate::ast::{GEdgeAnon, GEdgeNamed, Graph, Name, Vertex};

pub trait Visitor<R> {
    fn visit_nil(&self) -> R;
    fn visit_vertex(&self, vertex: &Vertex) -> R;
    fn visit_var(&self, var: &str) -> R;
    fn visit_nominate(&self, name: &str, vertex: &Vertex) -> R;
    fn visit_edge_anon(&self, edge: &GEdgeAnon, nominate_a: R, nominate_b: R) -> R;
    fn visit_edge_named(&self, edge: &GEdgeNamed, nominate_a: R, nominate_b: R) -> R;
    fn visit_rule_anon(&self, graph: &Graph, graph2: &Graph) -> R;
    fn visit_rule_named(&self, name: &Name, graph: &Graph, graph2: &Graph) -> R;
    fn visit_subgraph(&self, graph: &Graph, graph2: &Graph, identifier: &str) -> R;
    fn visit_tensor(&self, graph: &Graph, graph2: &Graph) -> R;
    fn visit_context(&self, name: &Name, context: &str) -> R;
}

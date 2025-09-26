use crate::ast::{
    Binding,
    GContext,
    GEdgeAnon,
    GEdgeNamed,
    GRuleAnon,
    GRuleNamed,
    GTensor,
    GVar,
    GVertex,
    GraphBinding,
};

#[allow(unused_variables)]
pub trait Visitor<'a, A, E> {
    fn visit_nil(&self, acc: A) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_vertex(&self, acc: A, vertex: &'a GVertex) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_var(&self, acc: A, var: &'a GVar) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_nominate(&self, acc: A, binding: &'a Binding) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_edge_anon(&self, acc: A, edge: &'a GEdgeAnon) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_edge_named(&self, acc: A, edge: &'a GEdgeNamed) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_rule_anon(&self, acc: A, rule: &'a GRuleAnon) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_rule_named(&self, acc: A, rule: &'a GRuleNamed) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_subgraph(&self, acc: A, subgraph: &'a GraphBinding) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_tensor(&self, acc: A, tensor: &'a GTensor) -> Result<A, E> {
        Ok(acc)
    }

    fn visit_context(&self, acc: A, context: &'a GContext) -> Result<A, E> {
        Ok(acc)
    }
}

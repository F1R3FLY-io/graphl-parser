#![allow(clippy::not_unsafe_ptr_arg_deref)]

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use tsify::Tsify;

use crate::bindings;
use crate::guard::{Guard, Guarded, ResourceConsumer};

#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
#[serde(tag = "type")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Error {
    #[error("invalid c string at position: {position}")]
    InvalidCString { position: usize },
    #[error("invalid utf-8 string")]
    InvalidUtf8String,
    #[error("got nullpointer at: {context}")]
    NullPointer { context: String },
    #[error("invalid enum variant at: {context}")]
    InvalidVariant { context: String },
    #[error("invalid graphl")]
    InvalidGraphL,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Binding {
    pub graph: Box<Graph>,
    pub var: String,
    pub vertex: Vertex,
}

impl TryFrom<bindings::Binding> for Binding {
    type Error = Error;

    fn try_from(value: bindings::Binding) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(Self::Error::NullPointer {
                context: "Binding".into(),
            });
        }

        unsafe {
            match (*value).kind {
                bindings::Binding__is_VBind => {
                    let v_bind = (*value).u.vBind_;
                    let graph = v_bind.graph_.try_into().map(Box::new)?;
                    let var = to_string(v_bind.lvar_)?;
                    let vertex = v_bind.vertex_.try_into()?;
                    Ok(Self { graph, var, vertex })
                }
                _ => Err(Self::Error::InvalidVariant {
                    context: "Binding".into(),
                }),
            }
        }
    }
}

impl TryFrom<Binding> for Guard<bindings::Binding> {
    type Error = Error;

    fn try_from(value: Binding) -> Result<Self, Self::Error> {
        let graph = (*value.graph).try_into()?;
        let var = to_c_string(value.var)?;
        let vertex = value.vertex.try_into()?;
        (var, vertex, graph)
            .consume(|(var, vertex, graph)| unsafe { bindings::make_VBind(var, vertex, graph) })
            .ok_or_else(|| Self::Error::NullPointer {
                context: "make_VBind returned null".into(),
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GraphBinding {
    pub graph_1: Box<Graph>,
    pub graph_2: Box<Graph>,
    pub var: String,
}

impl TryFrom<bindings::GraphBinding> for GraphBinding {
    type Error = Error;

    fn try_from(value: bindings::GraphBinding) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(Self::Error::NullPointer {
                context: "GraphBinding".into(),
            });
        }

        unsafe {
            match (*value).kind {
                bindings::GraphBinding__is_GBind => {
                    let g_bind = (*value).u.gBind_;
                    let graph_1 = g_bind.graph_1.try_into().map(Box::new)?;
                    let graph_2 = g_bind.graph_2.try_into().map(Box::new)?;
                    let var = to_string(g_bind.uvar_)?;
                    Ok(Self {
                        graph_1,
                        graph_2,
                        var,
                    })
                }
                _ => Err(Self::Error::InvalidVariant {
                    context: "GraphBinding".into(),
                }),
            }
        }
    }
}

impl TryFrom<GraphBinding> for Guard<bindings::GraphBinding> {
    type Error = Error;

    fn try_from(value: GraphBinding) -> Result<Self, Self::Error> {
        let graph_1 = (*value.graph_1).try_into()?;
        let graph_2 = (*value.graph_2).try_into()?;
        let var = to_c_string(value.var)?;
        (var, graph_1, graph_2)
            .consume(|(var, graph_1, graph_2)| unsafe {
                bindings::make_GBind(var, graph_1, graph_2)
            })
            .ok_or_else(|| Self::Error::NullPointer {
                context: "make_GBind returned null".into(),
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Vertex {
    pub name: Name,
}

impl TryFrom<bindings::Vertex> for Vertex {
    type Error = Error;

    fn try_from(value: bindings::Vertex) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(Self::Error::NullPointer {
                context: "Vertex".into(),
            });
        }

        unsafe {
            match (*value).kind {
                bindings::Vertex__is_VName => {
                    (*value).u.vName_.name_.try_into().map(|name| Self { name })
                }
                _ => Err(Self::Error::InvalidVariant {
                    context: "Vertex".into(),
                }),
            }
        }
    }
}

impl TryFrom<Vertex> for Guard<bindings::Vertex> {
    type Error = Error;

    fn try_from(value: Vertex) -> Result<Self, Self::Error> {
        let name = value.name.try_into()?;
        (name,)
            .consume(|(name,)| unsafe { bindings::make_VName(name) })
            .ok_or_else(|| Self::Error::NullPointer {
                context: "make_VName returned null".into(),
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Name {
    Wildcard,
    VVar { value: String },
    GVar { value: String },
    QuoteGraph { value: Box<Graph> },
    QuoteVertex { value: Box<Vertex> },
}

impl TryFrom<bindings::Name> for Name {
    type Error = Error;

    fn try_from(value: bindings::Name) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(Self::Error::NullPointer {
                context: "Name".into(),
            });
        }

        unsafe {
            match (*value).kind {
                bindings::Name__is_NameWildcard => Ok(Self::Wildcard),
                bindings::Name__is_NameVVar => {
                    to_string((*value).u.nameVVar_.lvar_).map(|value| Self::VVar { value })
                }
                bindings::Name__is_NameGVar => {
                    to_string((*value).u.nameGVar_.uvar_).map(|value| Self::GVar { value })
                }
                bindings::Name__is_NameQuoteGraph => (*value)
                    .u
                    .nameQuoteGraph_
                    .graph_
                    .try_into()
                    .map(|g| Self::QuoteGraph { value: Box::new(g) }),
                bindings::Name__is_NameQuoteVertex => (*value)
                    .u
                    .nameQuoteVertex_
                    .vertex_
                    .try_into()
                    .map(|v| Self::QuoteVertex { value: Box::new(v) }),
                _ => Err(Self::Error::InvalidVariant {
                    context: "Name".into(),
                }),
            }
        }
    }
}

impl TryFrom<Name> for Guard<bindings::Name> {
    type Error = Error;

    fn try_from(value: Name) -> Result<Self, Self::Error> {
        match value {
            Name::Wildcard => {
                let var = unsafe { bindings::make_NameWildcard() };

                if var.is_null() {
                    return Err(Error::NullPointer {
                        context: "make_NameWildcard returned null".into(),
                    });
                }

                Ok(var.guarded())
            }
            Name::VVar { value } => {
                let value = to_c_string(value)?;
                (value,)
                    .consume(|(value,)| unsafe { bindings::make_NameVVar(value) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_NameVVar returned null".into(),
                    })
            }
            Name::GVar { value } => {
                let value = to_c_string(value)?;
                (value,)
                    .consume(|(value,)| unsafe { bindings::make_NameGVar(value) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_NameGVar returned null".into(),
                    })
            }
            Name::QuoteGraph { value } => {
                let graph = (*value).try_into()?;
                (graph,)
                    .consume(|(graph,)| unsafe { bindings::make_NameQuoteGraph(graph) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_NameQuoteGraph returned null".into(),
                    })
            }
            Name::QuoteVertex { value } => {
                let vertex = (*value).try_into()?;
                (vertex,)
                    .consume(|(vertex,)| unsafe { bindings::make_NameQuoteVertex(vertex) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_NameQuoteVertex returned null".into(),
                    })
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GVertex {
    pub graph: Box<Graph>,
    pub vertex: Vertex,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GVar {
    pub graph: Box<Graph>,
    pub var: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GEdgeAnon {
    pub binding_1: Binding,
    pub binding_2: Binding,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GEdgeNamed {
    pub binding_1: Binding,
    pub binding_2: Binding,
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GRuleAnon {
    pub graph_1: Box<Graph>,
    pub graph_2: Box<Graph>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GRuleNamed {
    pub graph_1: Box<Graph>,
    pub graph_2: Box<Graph>,
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GTensor {
    pub graph_1: Box<Graph>,
    pub graph_2: Box<Graph>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GContext {
    pub graph: Box<Graph>,
    pub name: Name,
    pub string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Graph {
    Nil,
    Vertex(GVertex),
    Var(GVar),
    Nominate(Binding),
    EdgeAnon(GEdgeAnon),
    EdgeNamed(GEdgeNamed),
    RuleAnon(GRuleAnon),
    RuleNamed(GRuleNamed),
    Subgraph(GraphBinding),
    Tensor(GTensor),
    Context(GContext),
}

impl TryFrom<bindings::Graph> for Graph {
    type Error = Error;

    fn try_from(value: bindings::Graph) -> Result<Self, Self::Error> {
        if value.is_null() {
            return Err(Self::Error::NullPointer {
                context: "Graph".into(),
            });
        }

        unsafe {
            match (*value).kind {
                bindings::Graph__is_GNil => Ok(Self::Nil),
                bindings::Graph__is_GVertex => {
                    let g_vertex = (*value).u.gVertex_;
                    let graph = g_vertex.graph_.try_into().map(Box::new)?;
                    let vertex = g_vertex.vertex_.try_into()?;
                    Ok(Self::Vertex(GVertex { graph, vertex }))
                }
                bindings::Graph__is_GVar => {
                    let g_var = (*value).u.gVar_;
                    let graph = g_var.graph_.try_into().map(Box::new)?;
                    let var = to_string(g_var.lvar_)?;
                    Ok(Self::Var(GVar { graph, var }))
                }
                bindings::Graph__is_GNominate => {
                    let g_nominate = (*value).u.gNominate_;
                    let binding = g_nominate.binding_.try_into()?;
                    Ok(Self::Nominate(binding))
                }
                bindings::Graph__is_GEdgeAnon => {
                    let g_edge_anon = (*value).u.gEdgeAnon_;
                    let binding_1 = g_edge_anon.binding_1.try_into()?;
                    let binding_2 = g_edge_anon.binding_2.try_into()?;
                    Ok(Self::EdgeAnon(GEdgeAnon {
                        binding_1,
                        binding_2,
                    }))
                }
                bindings::Graph__is_GEdgeNamed => {
                    let g_edge_named = (*value).u.gEdgeNamed_;
                    let name = g_edge_named.name_.try_into()?;
                    let binding_1 = g_edge_named.binding_1.try_into()?;
                    let binding_2 = g_edge_named.binding_2.try_into()?;
                    Ok(Self::EdgeNamed(GEdgeNamed {
                        name,
                        binding_1,
                        binding_2,
                    }))
                }
                bindings::Graph__is_GRuleAnon => {
                    let g_rule_anon = (*value).u.gRuleAnon_;
                    let graph_1 = g_rule_anon.graph_1.try_into().map(Box::new)?;
                    let graph_2 = g_rule_anon.graph_2.try_into().map(Box::new)?;
                    Ok(Self::RuleAnon(GRuleAnon { graph_1, graph_2 }))
                }
                bindings::Graph__is_GRuleNamed => {
                    let g_rule_named = (*value).u.gRuleNamed_;
                    let name = g_rule_named.name_.try_into()?;
                    let graph_1 = g_rule_named.graph_1.try_into().map(Box::new)?;
                    let graph_2 = g_rule_named.graph_2.try_into().map(Box::new)?;
                    Ok(Self::RuleNamed(GRuleNamed {
                        graph_1,
                        graph_2,
                        name,
                    }))
                }
                bindings::Graph__is_GSubgraph => {
                    let g_subgraph = (*value).u.gSubgraph_;
                    let subgraph = g_subgraph.graphbinding_.try_into()?;
                    Ok(Self::Subgraph(subgraph))
                }
                bindings::Graph__is_GTensor => {
                    let g_tensor = (*value).u.gTensor_;
                    let graph_1 = g_tensor.graph_1.try_into().map(Box::new)?;
                    let graph_2 = g_tensor.graph_2.try_into().map(Box::new)?;
                    Ok(Self::Tensor(GTensor { graph_1, graph_2 }))
                }
                bindings::Graph__is_GContext => {
                    let g_context = (*value).u.gContext_;
                    let name = g_context.name_.try_into()?;
                    let graph = g_context.graph_.try_into().map(Box::new)?;
                    let string = to_string(g_context.string_)?;
                    Ok(Self::Context(GContext {
                        graph,
                        name,
                        string,
                    }))
                }
                _ => Err(Self::Error::InvalidVariant {
                    context: "Graph".into(),
                }),
            }
        }
    }
}

impl TryFrom<Graph> for Guard<bindings::Graph> {
    type Error = Error;

    fn try_from(value: Graph) -> Result<Self, Self::Error> {
        match value {
            Graph::Nil => {
                let var = unsafe { bindings::make_GNil() };

                if var.is_null() {
                    return Err(Error::NullPointer {
                        context: "make_GNil returned null".into(),
                    });
                }

                Ok(var.guarded())
            }
            Graph::Vertex(gvertex) => {
                let graph = (*gvertex.graph).try_into()?;
                let vertex = gvertex.vertex.try_into()?;
                (vertex, graph)
                    .consume(|(vertex, graph)| unsafe { bindings::make_GVertex(vertex, graph) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GVertex returned null".into(),
                    })
            }
            Graph::Var(gvar) => {
                let graph = (*gvar.graph).try_into()?;
                let var = to_c_string(gvar.var)?;
                (var, graph)
                    .consume(|(var, graph)| unsafe { bindings::make_GVar(var, graph) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GVar returned null".into(),
                    })
            }
            Graph::Nominate(binding) => {
                let binding = binding.try_into()?;
                (binding,)
                    .consume(|(binding,)| unsafe { bindings::make_GNominate(binding) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GNominate returned null".into(),
                    })
            }
            Graph::EdgeAnon(gedge_anon) => {
                let binding_1 = gedge_anon.binding_1.try_into()?;
                let binding_2 = gedge_anon.binding_2.try_into()?;
                (binding_1, binding_2)
                    .consume(|(binding_1, binding_2)| unsafe {
                        bindings::make_GEdgeAnon(binding_1, binding_2)
                    })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GEdgeAnon returned null".into(),
                    })
            }
            Graph::EdgeNamed(gedge_named) => {
                let binding_1 = gedge_named.binding_1.try_into()?;
                let binding_2 = gedge_named.binding_2.try_into()?;
                let name = gedge_named.name.try_into()?;
                (name, binding_1, binding_2)
                    .consume(|(name, binding_1, binding_2)| unsafe {
                        bindings::make_GEdgeNamed(name, binding_1, binding_2)
                    })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GEdgeNamed returned null".into(),
                    })
            }
            Graph::RuleAnon(grule_anon) => {
                let graph_1 = (*grule_anon.graph_1).try_into()?;
                let graph_2 = (*grule_anon.graph_2).try_into()?;
                (graph_1, graph_2)
                    .consume(|(graph_1, graph_2)| unsafe {
                        bindings::make_GRuleAnon(graph_1, graph_2)
                    })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GRuleAnon returned null".into(),
                    })
            }
            Graph::RuleNamed(grule_named) => {
                let graph_1 = (*grule_named.graph_1).try_into()?;
                let graph_2 = (*grule_named.graph_2).try_into()?;
                let name = grule_named.name.try_into()?;
                (name, graph_1, graph_2)
                    .consume(|(name, graph_1, graph_2)| unsafe {
                        bindings::make_GRuleNamed(name, graph_1, graph_2)
                    })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GRuleNamed returned null".into(),
                    })
            }
            Graph::Subgraph(graph_binding) => {
                let graph_binding = graph_binding.try_into()?;
                (graph_binding,)
                    .consume(|(graph_binding,)| unsafe { bindings::make_GSubgraph(graph_binding) })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GSubgraph returned null".into(),
                    })
            }
            Graph::Tensor(gtensor) => {
                let graph_1 = (*gtensor.graph_1).try_into()?;
                let graph_2 = (*gtensor.graph_2).try_into()?;
                (graph_1, graph_2)
                    .consume(|(graph_1, graph_2)| unsafe {
                        bindings::make_GTensor(graph_1, graph_2)
                    })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GTensor returned null".into(),
                    })
            }
            Graph::Context(gcontext) => {
                let graph = (*gcontext.graph).try_into()?;
                let name = gcontext.name.try_into()?;
                let string = to_c_string(gcontext.string)?;
                (string, name, graph)
                    .consume(|(string, name, graph)| unsafe {
                        bindings::make_GContext(string, name, graph)
                    })
                    .ok_or_else(|| Self::Error::NullPointer {
                        context: "make_GContext returned null".into(),
                    })
            }
        }
    }
}

fn to_string(chars: *mut std::os::raw::c_char) -> Result<String, Error> {
    unsafe { std::ffi::CStr::from_ptr(chars) }
        .to_str()
        .map_err(|_| Error::InvalidUtf8String)
        .map(ToOwned::to_owned)
}

fn to_c_string(str: String) -> Result<Guard<*mut std::os::raw::c_char>, Error> {
    let c_str = std::ffi::CString::new(str).map_err(|err| Error::InvalidCString {
        position: err.nul_position(),
    })?;

    // we need to reallocate with malloc
    let var = unsafe { bindings::make_LVar(c_str.as_ptr() as _) };

    if var.is_null() {
        return Err(Error::NullPointer {
            context: "make_LVar returned null".into(),
        });
    }

    Ok(var.guarded())
}

#[test]
fn test_curly_braces_are_correctly_inserted() {
    let graphl = r#"< a > | { context "foo" for f in 0 }"#;
    let ast = crate::parse_to_ast(graphl.to_owned()).unwrap();

    let printed_graphl = crate::ast_to_graphl(ast.clone()).unwrap();
    let printed_ast = crate::parse_to_ast(printed_graphl).unwrap();

    assert_eq!(ast, printed_ast)
}

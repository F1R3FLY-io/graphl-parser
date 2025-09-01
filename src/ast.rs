#![allow(clippy::not_unsafe_ptr_arg_deref)]

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use tsify::Tsify;

use crate::bindings;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Error {
    InvalidCString { position: usize },
    InvalidUtf8String,
    NullPointer { context: String },
    InvalidVariant { context: String },
    InvalidGraphL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub enum Name {
    Wildcard,
    VVar { value: String },
    GVar { value: String },
    QuoteGraph(Box<Graph>),
    QuoteVertex(Box<Vertex>),
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
                    .map(|g| Self::QuoteGraph(Box::new(g))),
                bindings::Name__is_NameQuoteVertex => (*value)
                    .u
                    .nameQuoteVertex_
                    .vertex_
                    .try_into()
                    .map(|v| Self::QuoteVertex(Box::new(v))),
                _ => Err(Self::Error::InvalidVariant {
                    context: "Name".into(),
                }),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GVertex {
    pub graph: Box<Graph>,
    pub vertex: Vertex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GVar {
    pub graph: Box<Graph>,
    pub var: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GEdgeAnon {
    pub binding_1: Binding,
    pub binding_2: Binding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GEdgeNamed {
    pub binding_1: Binding,
    pub binding_2: Binding,
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GRuleAnon {
    pub graph_1: Box<Graph>,
    pub graph_2: Box<Graph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GRuleNamed {
    pub graph_1: Box<Graph>,
    pub graph_2: Box<Graph>,
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GTensor {
    pub graph_1: Box<Graph>,
    pub graph_2: Box<Graph>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                _ => Err(Self::Error::InvalidVariant {
                    context: "Graph".into(),
                }),
            }
        }
    }
}

fn to_string(chars: *mut ::std::os::raw::c_char) -> Result<String, Error> {
    unsafe { std::ffi::CStr::from_ptr(chars) }
        .to_str()
        .map_err(|_| Error::InvalidUtf8String)
        .map(ToOwned::to_owned)
}

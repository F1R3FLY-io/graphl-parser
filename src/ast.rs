#![allow(clippy::not_unsafe_ptr_arg_deref)]

use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use tsify::Tsify;

use crate::bindings;
use crate::guard::{Guard, Guarded};

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

impl TryFrom<Binding> for Guard<bindings::Binding> {
    type Error = Error;

    fn try_from(value: Binding) -> Result<Self, Self::Error> {
        let graph: Guard<_> = (*value.graph).try_into()?;
        let var = to_c_string(value.var)?;
        let vertex: Guard<_> = value.vertex.try_into()?;
        Ok(unsafe {
            bindings::make_VBind(var.into_inner(), vertex.into_inner(), graph.into_inner())
        }
        .guarded())
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

impl TryFrom<GraphBinding> for Guard<bindings::GraphBinding> {
    type Error = Error;

    fn try_from(value: GraphBinding) -> Result<Self, Self::Error> {
        let graph_1: Guard<_> = (*value.graph_1).try_into()?;
        let graph_2: Guard<_> = (*value.graph_2).try_into()?;
        let var = to_c_string(value.var)?;
        Ok(unsafe {
            bindings::make_GBind(var.into_inner(), graph_1.into_inner(), graph_2.into_inner())
        }
        .guarded())
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

impl TryFrom<Vertex> for Guard<bindings::Vertex> {
    type Error = Error;

    fn try_from(value: Vertex) -> Result<Self, Self::Error> {
        let name: Guard<_> = value.name.try_into()?;
        Ok(unsafe { bindings::make_VName(name.into_inner()) }.guarded())
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

impl TryFrom<Name> for Guard<bindings::Name> {
    type Error = Error;

    fn try_from(value: Name) -> Result<Self, Self::Error> {
        match value {
            Name::Wildcard => Ok(unsafe { bindings::make_NameWildcard() }.guarded()),
            Name::VVar { value } => {
                let value = to_c_string(value)?;
                Ok(unsafe { bindings::make_NameVVar(value.into_inner()) }.guarded())
            }
            Name::GVar { value } => {
                let value = to_c_string(value)?;
                Ok(unsafe { bindings::make_NameGVar(value.into_inner()) }.guarded())
            }
            Name::QuoteGraph(graph) => {
                let graph: Guard<_> = (*graph).try_into()?;
                Ok(unsafe { bindings::make_NameQuoteGraph(graph.into_inner()) }.guarded())
            }
            Name::QuoteVertex(vertex) => {
                let vertex: Guard<_> = (*vertex).try_into()?;
                Ok(unsafe { bindings::make_NameQuoteVertex(vertex.into_inner()) }.guarded())
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

impl TryFrom<Graph> for Guard<bindings::Graph> {
    type Error = Error;

    fn try_from(value: Graph) -> Result<Self, Self::Error> {
        match value {
            Graph::Nil => Ok(unsafe { bindings::make_GNil() }.guarded()),
            Graph::Vertex(gvertex) => {
                let graph: Guard<_> = (*gvertex.graph).try_into()?;
                let vertex: Guard<_> = gvertex.vertex.try_into()?;
                Ok(
                    unsafe { bindings::make_GVertex(vertex.into_inner(), graph.into_inner()) }
                        .guarded(),
                )
            }
            Graph::Var(gvar) => {
                let graph: Guard<_> = (*gvar.graph).try_into()?;
                let var = to_c_string(gvar.var)?;
                Ok(unsafe { bindings::make_GVar(var.into_inner(), graph.into_inner()) }.guarded())
            }
            Graph::Nominate(binding) => {
                let binding: Guard<_> = binding.try_into()?;
                Ok(unsafe { bindings::make_GNominate(binding.into_inner()) }.guarded())
            }
            Graph::EdgeAnon(gedge_anon) => {
                let binding_1: Guard<_> = gedge_anon.binding_1.try_into()?;
                let binding_2: Guard<_> = gedge_anon.binding_2.try_into()?;
                Ok(unsafe {
                    bindings::make_GEdgeAnon(binding_1.into_inner(), binding_2.into_inner())
                }
                .guarded())
            }
            Graph::EdgeNamed(gedge_named) => {
                let binding_1: Guard<_> = gedge_named.binding_1.try_into()?;
                let binding_2: Guard<_> = gedge_named.binding_2.try_into()?;
                let name: Guard<_> = gedge_named.name.try_into()?;
                Ok(unsafe {
                    bindings::make_GEdgeNamed(
                        name.into_inner(),
                        binding_1.into_inner(),
                        binding_2.into_inner(),
                    )
                }
                .guarded())
            }
            Graph::RuleAnon(grule_anon) => {
                let graph_1: Guard<_> = (*grule_anon.graph_1).try_into()?;
                let graph_2: Guard<_> = (*grule_anon.graph_2).try_into()?;
                Ok(
                    unsafe { bindings::make_GRuleAnon(graph_1.into_inner(), graph_2.into_inner()) }
                        .guarded(),
                )
            }
            Graph::RuleNamed(grule_named) => {
                let graph_1: Guard<_> = (*grule_named.graph_1).try_into()?;
                let graph_2: Guard<_> = (*grule_named.graph_2).try_into()?;
                let name: Guard<_> = grule_named.name.try_into()?;
                Ok(unsafe {
                    bindings::make_GRuleNamed(
                        name.into_inner(),
                        graph_1.into_inner(),
                        graph_2.into_inner(),
                    )
                }
                .guarded())
            }
            Graph::Subgraph(graph_binding) => {
                let graph_binding: Guard<_> = graph_binding.try_into()?;
                Ok(unsafe { bindings::make_GSubgraph(graph_binding.into_inner()) }.guarded())
            }
            Graph::Tensor(gtensor) => {
                let graph_1: Guard<_> = (*gtensor.graph_1).try_into()?;
                let graph_2: Guard<_> = (*gtensor.graph_2).try_into()?;
                Ok(
                    unsafe { bindings::make_GTensor(graph_1.into_inner(), graph_2.into_inner()) }
                        .guarded(),
                )
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
    Ok(unsafe { bindings::make_LVar(c_str.as_ptr()) }.guarded())
}

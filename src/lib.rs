mod utils;
pub mod edge;
mod node;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use edge::Edge;
use node::Node;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

#[wasm_bindgen]
impl Graph {
    pub fn new() -> Graph { Graph { nodes: vec![], edges: vec![] } }

    pub fn add_node(&mut self, node: Node) { self.nodes.push(node) }
    pub fn get_nodes(&self) -> JsValue { serde_wasm_bindgen::to_value(&self.nodes).unwrap() }

    pub fn add_edge(&mut self, edge: Edge) { self.edges.push(edge)}
    pub fn get_edges(&self) -> JsValue { serde_wasm_bindgen::to_value(&self.edges).unwrap() }
}

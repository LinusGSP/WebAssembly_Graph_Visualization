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

    pub fn get_html(&self) -> JsValue {
        let radius = 50;
        let html = self.nodes.iter()
            .map(|node| format!("<div style='width: {}px; height: {}px; border-radius: {}px; background-color: red; position: absolute; left: {}px; top: {}px; margin-top: -{}px; margin-left: -{}px;'></div>",
            radius, radius, radius, node.pos.0, node.pos.1, radius/2, radius/2)).collect::<String>();
        serde_wasm_bindgen::to_value(&html).unwrap()
    }
}

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Edge {
    source: u32,
    target: u32,
    weight: f64
}

#[wasm_bindgen]
impl Edge {
    pub fn new(source: u32, target: u32, weight: f64) -> Edge {
        Edge { source, target, weight }
    }
}
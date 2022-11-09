use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Node {
    pos: (f64, f64),
    id: u16,
    label: String,
}

#[wasm_bindgen]
impl Node {
    pub fn new(x: f64, y: f64, id: u16, label: Option<String>) -> Node {
        let label = label.unwrap_or(id.to_string());
        Node {  pos: (x, y), id, label }
    }

    pub fn get_pos(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.pos).unwrap()
    }
}

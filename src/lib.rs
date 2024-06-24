use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[wasm_bindgen]
pub fn process_json(input: &str) -> Result<String, JsValue> {
    match serde_json::from_str::<Value>(input) {
        Ok(parsed) => {
            let mut obj = parsed.as_object().unwrap().clone();
            obj.insert("processed".to_string(), json!(true));
            Ok(serde_json::to_string(&obj).unwrap())
        },
        Err(e) => Err(JsValue::from_str(&format!("Failed to parse JSON: {}", e))),
    }
}

#[wasm_bindgen]
pub struct JsonProcessor {
    data: Value,
}

#[wasm_bindgen]
impl JsonProcessor {

    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<JsonProcessor, JsValue> {
        let data: Value = serde_json::from_str(input).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(JsonProcessor { data })
    }

    pub fn process(&mut self) -> Result<String, JsValue> {
        // Example processing: add a new field to the JSON
        let obj = self.data.as_object_mut()
            .ok_or_else(|| JsValue::from_str("Expected a JSON object"))?;
        obj.insert("processed".to_string(), json!(true));
        serde_json::to_string(&self.data).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn get_data(&self) -> String {
        serde_json::to_string(&self.data).unwrap()
    }
}

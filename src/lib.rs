use wasm_bindgen::prelude::*;
use serde_json::{Value, json};

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen]
pub fn set_hook() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn process_json(input: &str) -> Result<Vec<String>, JsValue> {
    match serde_json::from_str::<Value>(input) {
        Ok(parsed) => {
            let url = parsed["url"].as_str().unwrap();
            let client = reqwest::Client::new();
            let response: serde_json::Value = client.get(url)
                .send()
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?
                .json()
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let mut obj = parsed.as_object().unwrap().clone();
            obj.insert("processed".to_string(), json!(true));
            obj.insert("answer".to_string(), json!(42));
            obj.insert("response".to_string(), response);
            let ret = serde_json::to_string(&obj).unwrap();
            Ok(vec![ret])
        },
        Err(e) => Err(JsValue::from_str(&format!("Failed to parse JSON: {}", e))),
    }
}

#[wasm_bindgen]
pub fn process_json_blocking(input: &str) -> Result<Vec<String>, JsValue> {
    match serde_json::from_str::<Value>(input) {
        Ok(parsed) => {
            let url = parsed["url"].as_str().unwrap();
            // TODO: blocking IO is not supported by WebAssembly runtime!
            let response = format!("TODO: blocking HTTP call to {url}");
            let mut obj = parsed.as_object().unwrap().clone();
            obj.insert("processed".to_string(), json!(true));
            obj.insert("answer".to_string(), json!(42));
            obj.insert("response".to_string(), json!(response));
            let ret = serde_json::to_string(&obj).unwrap();
            Ok(vec![ret])
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

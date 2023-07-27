use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ethereum, js_name = isConnected)]
    pub fn is_connected() -> bool;

    // this actually returns a promise. Need to figure out how to handle this without async rust
    #[wasm_bindgen(js_namespace = ethereum, js_name = request)]
    async fn request_raw(args: JsValue) -> JsValue;
}

#[derive(serde::Serialize)]
pub struct RequestArgs {
    pub method: String,
    pub params: Option<Vec<String>>,
}
pub async fn request(method: &str, params: &[&str]) -> JsValue {
    let args = RequestArgs {
        method: method.to_string(),
        params: Some(params.iter().map(|s| s.to_string()).collect()),
    };
    request_raw(serde_wasm_bindgen::to_value(&args).unwrap()).await
}

pub async fn request_accounts() -> JsValue {
    request("eth_requestAccounts", &[]).await
}

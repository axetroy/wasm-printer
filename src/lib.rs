use wasm_bindgen::prelude::*;
use wasmprinter::print_bytes;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse_wat(bytes: Vec<u8>) -> Result<String, JsValue> {
    let output = print_bytes(bytes);

    match output {
        Ok(output) => Ok(output),
        Err(e) => {
            let s = format!("parse wasm file error: {}", e);
            Err(JsValue::from(s))
        }
    }
}

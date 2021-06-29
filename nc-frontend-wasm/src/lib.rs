use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() -> Result<(), JsValue> {
    // Sets the panic hook.
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"Hello from rust!".into());
    Ok(())
}

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // Sets the panic hook.
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"Hello from rust!".into());

    Ok(())
}


use js_sys;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    using_web_sys();
}

// First up let's take a look of binding `console.log` manually, without the
// help of `web_sys`. Here we're writing the `#[wasm_bindgen]` annotations
// manually ourselves, and the correctness of our program relies on the
// correctness of these annotations!

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageRawData {
    data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

#[wasm_bindgen]
impl ImageRawData {
    #[wasm_bindgen]
    pub fn get_data(&self) -> js_sys::Uint8Array {
        unsafe { js_sys::Uint8Array::view(&self.data[..]) }
    }
}

#[wasm_bindgen(js_name = "getImageData")]
pub fn get_image_data() -> ImageRawData {
    let w = 150;
    let h = 150;
    let mut data = Vec::with_capacity(w * h);
    for i in 0..w {
        for j in 0..h {
            data.push((i * 255 / w) as u8);
            data.push((j * 255 / w) as u8);
            data.push((i * 255 / w) as u8);
            data.push(255 as u8);
        }
    }
    ImageRawData {
        data,
        width: w,
        height: w,
    }
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// And finally, we don't even have to define the `log` function ourselves! The
// `web_sys` crate already has it defined for us.

fn using_web_sys() {
    use web_sys::console;

    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
}

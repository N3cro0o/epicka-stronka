// TODO
// Add window size injector

use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f32;
}

pub fn get_pixel_ratio() -> Result<f64, String> {
    let w = window().ok_or(String::from("Cannot get window"))?;
    Ok(w.device_pixel_ratio())
}

pub fn get_page_size() -> Result<(u32, u32), String> {
    let w = window().ok_or(String::from("Cannot get window"))?;
    let height = w.inner_height().unwrap_or(wasm_bindgen::JsValue::from_f64(1080.0));
    let width = w.inner_width().unwrap_or(wasm_bindgen::JsValue::from_f64(1920.0));
    let h = height.as_f64().unwrap_or(1080.0);
    let w = width.as_f64().unwrap_or(1920.0);
    Ok((w as u32, h as u32))
}

pub fn request_animation_frame(closure: &Closure<dyn FnMut() -> ()>) {
    window().expect("Cannot get window")
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("register should be a OK");
}

pub fn random_i32() -> i32 {
    let f = random();
    let u = (f * 10000.0) as i32;
    u
}

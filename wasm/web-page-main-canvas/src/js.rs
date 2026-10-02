// TODO
// Add window size injector

use wasm_bindgen::prelude::*;
use web_sys::window;
use super::{HTML_SENDER, Input};

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn check_page() {
    let mut s = HTML_SENDER.get().unwrap().clone();
    s.try_send(Input::ReturnCurrentPage).unwrap();
}

#[wasm_bindgen]
pub fn change_page_num(id: usize) {
    let mut s = HTML_SENDER.get().unwrap().clone();
    s.try_send(Input::ChangeCurrentPage(id)).unwrap();
}

#[wasm_bindgen]
pub fn change_app_size(width: usize, height: usize) {
    let mut s = HTML_SENDER.get().unwrap().clone();
    s.try_send(Input::ChangeAppSize((width, height))).unwrap();
}

pub fn get_pixel_ratio() -> Result<f64, String> {
    let w = window().ok_or(String::from("Cannot get window"))?;
    Ok(w.device_pixel_ratio())
}

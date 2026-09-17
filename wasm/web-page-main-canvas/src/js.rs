use wasm_bindgen::prelude::*;
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


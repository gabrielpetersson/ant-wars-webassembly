use crate::draw::draw_box;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod draw;

#[wasm_bindgen]
extern {
    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);

    #[wasm_bindgen(method, js_name = querySelector)]
    fn query_selector(this: &HTMLDocument, selector: &str) -> Option<Element>;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

#[wasm_bindgen]
pub fn create_elementt() {
    log("UEEEEEEEE");
    let canvas = document.query_selector("#game-canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>();
    
    let a = match canvas {
        Err(_) => return (),
        Ok(f) =>  f,
    };
    draw_box(a, 5.0, 5.0);
}
use crate::log;
use wasm_bindgen::JsCast;


pub fn draw_box(canvas: web_sys::HtmlCanvasElement, x: f64, y: f64) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    let a = wasm_bindgen::JsValue::from("black");
    log("UEEEEEEEE");
    context.set_stroke_style(&a);
    context.rect(x, y, 50.0, 50.0);
    println!("YESSSSSSSSSSSSSSSSSS");
    context.stroke();
}
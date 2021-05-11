use wasm_bindgen::JsCast;


pub fn draw_box(canvas: &web_sys::HtmlCanvasElement, x: f64, y: f64) {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    let js_color = wasm_bindgen::JsValue::from("black");
    context.set_stroke_style(&js_color);
    context.fill_rect(x, y, 50.0, 50.0);
    context.stroke();
}

pub fn clear_canvas(canvas: &web_sys::HtmlCanvasElement) {
    canvas.set_width(canvas.width());
}
use crate::map::GameMap;
use crate::draw::clear_canvas;
use crate::ant::Ant;
use crate::ant::Point;
use crate::draw::draw_box;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;

mod draw;
mod ant;
mod map;

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

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen]
pub fn create_elementt() {
    log("UEEEEEEEE");
    let maybe_canvas = document.query_selector("#game-canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>();
    let canvas = match maybe_canvas {
        Err(_) => return (),
        Ok(f) =>  f,
    };

    let map = GameMap { width: 500.0, height: 500.0} ;
    let mut ant = Ant { pos: Point { x: 1.0, y: 6.0 }, direction_angle: 100.0} ;

    
    canvas.set_width(map.width as u32);
    canvas.set_height(map.height as u32);
    
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        ant.next();
        clear_canvas(&canvas);
        draw_box(&canvas, ant.pos.x, ant.pos.y);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    
}
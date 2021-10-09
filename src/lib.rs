mod ant;
mod anthill;
mod draw;
mod map;

use crate::ant::Ant;
use crate::ant::Team;
use crate::anthill::Anthill;
use crate::draw::Painter;
use crate::map::GameMap;

use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

#[wasm_bindgen]
extern "C" {
    type HTMLDocument;
    type Element;

    // static document: HTMLDocument;

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

// console_log!("1 + 3 = {}", 1 + 3);
// macro_rules! log {
//     ($($t:tt)*) => (console_log(&format_args!($($t)*).to_string()))
// }

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

pub fn get_random_i32(from: i32, to: i32) -> i32 {
    rand::thread_rng().gen_range(from..to)
}

#[wasm_bindgen]
pub fn create_element() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    let maybe_canvas = document
        .query_selector("#game-canvas")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>();
    let map_canvas = match maybe_canvas {
        Err(_) => return (),
        Ok(f) => f,
    };

    let map = GameMap {
        width: 3000.0,
        height: 3000.0,
    };
    map_canvas.set_width(map.width as u32);
    map_canvas.set_height(map.height as u32);
    let mut map_painter = Arc::new(Mutex::new(Painter::new(map_canvas)));
    let map_painter_clone = Arc::clone(&map_painter);
    let cb = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut map_painter = map_painter_clone.lock().unwrap();
        web_sys::console::log_1(&event);
        if event.key_code() == 68 {
            map_painter.increment_x_offset(10.0);
        } else if event.key_code() == 87 {
            map_painter.increment_y_offset(-10.0);
        } else if event.key_code() == 65 {
            map_painter.increment_x_offset(-10.0);
        } else if event.key_code() == 83 {
            map_painter.increment_y_offset(10.0);
        }
    }) as Box<dyn FnMut(_)>);
    document
        .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
        .expect("error");
    cb.forget();

    let mut anthills = vec![
        Anthill::new(Team::TOP, 400.0, 150.0),
        Anthill::new(Team::BOTTOM, 360.0, 870.0),
    ];

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut map_painter = map_painter.lock().unwrap();
        map_painter.clear_map();

        let mut ants: Vec<&mut Ant> = vec![];
        let mut ants_clone: Vec<Ant> = vec![];

        let num = get_random_i32(0, 30);
        for anthill in anthills.iter_mut() {
            anthill.remove_dead_ants();
            map_painter.draw_anthill(&anthill);

            if (num == 0) & (anthill.team == Team::BOTTOM) {
                anthill.spawn_ant()
            }
            if (num == 1) & (anthill.team == Team::TOP) {
                anthill.spawn_ant()
            }

            for ant in anthill.ants.iter_mut() {
                ants_clone.push(ant.clone());
                ants.push(ant);
            }
        }

        let mut attacked_ant_ids: Vec<String> = vec![];
        for ant in ants.iter_mut() {
            ant.update_intention(&ants_clone);
            let attacked_ant_id = ant.update_position(&ants_clone);
            match attacked_ant_id {
                Option::Some(ant_id) => attacked_ant_ids.push(ant_id),
                Option::None => (),
            }
            map_painter.draw_ant(ant);
        }

        for ant in ants.iter_mut() {
            for ant_id in attacked_ant_ids.iter() {
                if ant.id == ant_id.clone() {
                    ant.take_damage(20.0);
                }
            }
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
}

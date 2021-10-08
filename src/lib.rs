mod anthill;
mod draw;
mod ant;
mod map;

use crate::draw::Painter;
use crate::map::GameMap;
use crate::ant::Ant;
use crate::ant::Team;
use crate::anthill::Anthill;

use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
extern {
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
    
    let maybe_canvas = document.query_selector("#game-canvas").unwrap().unwrap().dyn_into::<web_sys::HtmlCanvasElement>();
    let canvas = match maybe_canvas {
        Err(_) => return (),
        Ok(f) =>  f,
    };

    let map = GameMap { width: 3000.0, height: 3000.0} ;

    // let mut ants: Vec<Ant> = [Ant::new(Team::TOP, &map), Ant::new(Team::BOTTOM, &map)].to_vec();
    let anthills =  vec![Anthill::new(Team::TOP, 30.0, 30.0), Anthill::new(Team::BOTTOM, 300.0, 500.0)];
    
    canvas.set_width(map.width as u32);
    canvas.set_height(map.height as u32);
    let mut map_painter = Painter::new(canvas); 

    // let mut test  = || {
    //     map_painter.increment_x_offset(10.0);
    // };

    // let cb = Closure::wrap(Box::new(|event: web_sys::KeyboardEvent | { 
    //     // log(&"he".to_string());
    //     web_sys::console::log_1(&event);
    //     if event.key_code() == 68 {
    //         // test()
    //         map_painter.increment_x_offset(10.0);
    //     }
    //     else if event.key_code() == 87 {
    //         // map_painter.increment_y_offset(-10.0);
    //     }
    //     else if event.key_code() == 65 {
    //         // map_painter.increment_x_offset(-10.0);
    //     }
    //     else if event.key_code() == 83 {
    //         // map_painter.increment_y_offset(10.0);
    //     }
    // }) as Box<dyn FnMut(_)>);

    // document.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()).expect("error");
    // cb.forget();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        map_painter.clear_map();

        let ants: Vec<Ant> = vec![];
        for anthill in anthills {
            for ant in anthill.ants {
                ants.push(ant);
            }
        }

        let ants_clone: Vec<Ant> = (*ants.clone()).to_vec();
        let num = get_random_i32(0, 30);
        // let mut new_ant: Option<Ant> = Option::None;

        for anthill in anthills {
            if (num == 0) & (anthill.team == Team::BOTTOM) {
                anthill.spawn_ant()
            }
            if (num == 1) & (anthill.team == Team::TOP) {
                anthill.spawn_ant()
            }
        }
        
        // if num == 0 {
            
        // }
        // if num == 1 {

        // }
        // match new_ant {
        //     Option::None => (),
        //     Option::Some(new_ant) =>{
        //         ants.insert(0, new_ant); 
        //     }
        // }
        
        let mut attacked_ant_ids: Vec<String> = vec![];
        for ant in ants.iter_mut() {
            ant.update_intention(&ants_clone);
            let attacked_ant_id = ant.update_position(&ants_clone);
            match attacked_ant_id {
                Option::Some(ant_id) => attacked_ant_ids.push(ant_id),
                Option::None => ()
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

        let mut alive_ants: Vec<Ant> = [].to_vec();
        for ant in ants.iter() {
            if ant.health > 0.0 {
                alive_ants.insert(0, ant.clone());
            }
        } 
        ants = alive_ants;


        for anthill in anthills.iter() {
            map_painter.draw_anthill(anthill);
        }       
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    
}

use crate::Anthill;
use crate::Ant;
use wasm_bindgen::JsCast;

// #[derive(Clone)]
pub struct Painter {
     canvas:  web_sys::HtmlCanvasElement,  
     x_offset: f64,
     y_offset: f64,
}

impl Painter {
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Painter{
        Painter { canvas, y_offset: 0.0, x_offset: 0.0 }    
    }

    pub fn set_y_offset(&mut self, y: f64) {
        self.y_offset = y;
    }

    pub fn set_x_offset(&mut self, x: f64) {
        self.x_offset = x;
    }

    pub fn increment_y_offset(&mut self, y: f64) {
        self.set_y_offset(self.y_offset + y);
    }

    pub fn increment_x_offset(&mut self, x: f64) {
        self.set_x_offset(self.y_offset + x);
    }
    
                                                                                                                                                pub fn draw_box(&mut self, x: f64, y: f64, width: f64, height: f64, color: String) {
        let context = self.canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
    
        
        let js_color = wasm_bindgen::JsValue::from_str(&color);
        context.set_fill_style(&js_color);
        context.fill_rect(x + self.x_offset, y + self.y_offset, width, height);
    }
    
    pub fn draw_ant(&mut self, ant: &Ant) {
        // ant
        self.draw_box(ant.pos.x, ant.pos.y, 12.0, 28.0, "orange".to_string());
        // hp
        self.draw_box(ant.pos.x - 7.0, ant.pos.y - 12.0, 26.0  * (ant.health / 100.0) , 5.0, "black".to_string());
    }

    pub fn draw_anthill(&mut self, anthill: &Anthill) {
        self.draw_box(anthill.pos.x, anthill.pos.y, 30.0, 30.0, "brown".to_string());
    }

    pub fn clear_map(&mut self) {
        self.canvas.set_width(self.canvas.width());
    }
}
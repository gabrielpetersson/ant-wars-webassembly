pub struct Point {
    pub x: f64,
    pub y: f64
}

pub struct Ant {
    pub pos: Point,
    pub direction_angle: f64
}

impl Ant {
    pub fn next(&mut self) {
        self.pos = Point {x: self.pos.x, y: self.pos.y + 1.0}
    }
}

pub struct Point {
    pub x: f64,
    pub y: f64
}

pub enum Team {
    TOP,
    BOTTOM
}
pub struct Ant {
    pub team: Team, 
    pub pos: Point,
    pub direction_angle: f64
}

impl Ant {
    pub fn new(team: Team) -> Ant {
        Ant { pos: Point { x: 1.0, y: 6.0 }, direction_angle: 100.0, team }
    }

    pub fn update_intention(&mut self) {
        self.pos = Point {x: self.pos.x, y: self.pos.y + 1.0}
    }

    pub fn update_position(&mut self) {
        self.pos = Point {x: self.pos.x, y: self.pos.y + 1.0}
    }
}

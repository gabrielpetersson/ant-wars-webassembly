use crate::ant::Point;
use crate::Team;
use nanoid::nanoid;

// #[derive(Clone)]
pub struct Anthill {
    pub team: Team, 
    pub pos: Point,
    pub id: String  
}

impl Anthill {
    pub fn new(team: Team, x: f64, y: f64) -> Anthill {
        Anthill { team, pos: Point { x, y }, id: nanoid!() }
    }
}
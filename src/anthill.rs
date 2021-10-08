use crate::Ant;
use crate::ant::Point;
use crate::Team;
use nanoid::nanoid;

// #[derive(Clone)]
pub struct Anthill {
    pub team: Team, 
    pub pos: Point,
    pub id: String,
    pub ants: Vec<Ant>,  
}

impl Anthill {
    pub fn new(team: Team, x: f64, y: f64) -> Anthill {
        Anthill { team, pos: Point { x, y }, id: nanoid!(), ants: vec![] }
    }

    pub fn spawn_ant(&mut self) {
        let ant = Ant::new(Team::TOP, &self);
        self.ants.push(ant);
    }
}
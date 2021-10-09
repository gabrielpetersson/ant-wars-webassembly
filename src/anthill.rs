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
        let ant = Ant::new(self.team.clone(), &self);
        self.ants.push(ant);
    }

    pub fn remove_dead_ants(&mut self) {
        let mut alive_ants: Vec<Ant> = [].to_vec();
        for ant in self.ants.iter_mut() {
            if ant.health > 0.0 {
                alive_ants.insert(0, ant.clone());
            }
        }
        self.ants = alive_ants;
    }

    
}
use crate::get_random_i32;
use crate::GameMap;
use crate::log;
use std::fmt;
use nanoid::nanoid;

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

#[derive(Clone, PartialEq)]
pub enum Team {
    TOP,
    BOTTOM
}

#[derive(Clone, Debug)]
pub enum Intention {
    ATTACK,
    WALK_TOWARDS_target_id,
    STROLL
}

impl fmt::Display for Intention {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub struct Ant {
    pub team: Team, 
    pub pos: Point,
    pub direction_angle: f64,
    pub intention: Intention, 
    pub health: f64,
    pub cooldown: f64,
    pub damage: f64,
    pub id: String,
    pub target_id: String,
}

fn get_starting_point(team: &Team, map_width: i32) -> Point {
    let x = get_random_i32(0, map_width);
    if matches!(team, Team::TOP) {
        return Point { x: x as f64, y: 40.0 }
    }

    return Point { x: x as f64, y: 500.0 }
}

impl Ant {
    fn get_delta(&mut self) -> f64 {
        if matches!(self.team, Team::TOP) {
            return 1.0
        }

        return -1.0
    }

    pub fn new(team: Team, map: &GameMap) -> Ant {
        let t = team.clone();
        Ant { team, pos: get_starting_point(&t, map.width as i32), direction_angle: 100.0, intention: Intention::STROLL, damage: 10.0, health: 100.0, cooldown: 0.0, id: nanoid!(), target_id: nanoid!() }
    }

    pub fn get_distance_to_ant(&mut self, ant: &Ant) -> f64{
        f64::abs(self.pos.x - ant.pos.x) + f64::abs(self.pos.y - ant.pos.y)
    }

    pub fn take_damage(&mut self, damage: f64) {
        self.health -= damage;
    }

    pub fn update_intention(&mut self, ants: &Vec<Ant>) {
        let closest_enemy = self.get_closest_enemy(ants);
        match closest_enemy {
            Option::Some(closest_enemy) => {
                let x_distance = f64::abs(self.pos.x - closest_enemy.pos.x);
                let y_distance = f64::abs(self.pos.y - closest_enemy.pos.y);
                
                if  (x_distance < 24.0) & (y_distance < 24.0) {
                    self.intention = Intention::ATTACK;
                    return
                } 
                else if  (x_distance < 75.0) & (y_distance < 75.0) {
                    self.intention = Intention::WALK_TOWARDS_target_id;
                    return
                }   
                else {
                    self.intention = Intention::STROLL;
                }  
            },
            _ => {
                // Option::None => {
                    self.intention = Intention::STROLL;
                     log("is none");
                // },
            }
        }
    }
    
    pub fn get_closest_enemy(&mut self, ants: &Vec<Ant>) -> Option<Ant> {
        let mut closest_ant = Option::None; 
        let mut closest_ant_dist = f64::INFINITY;

        // let iter_ants = ants.clone();
        for ant in ants {
            if self.team == ant.team {
                continue;
            }

            let dist = self.get_distance_to_ant(ant);

            if dist < closest_ant_dist {
                closest_ant = Some(ant.clone());
                closest_ant_dist = dist
            }
        }

        return closest_ant;
    }

    pub fn update_position(&mut self, ants: &Vec<Ant>) -> Option<String> {
        if matches!(self.intention, Intention::STROLL) {
            self.pos = Point {x: self.pos.x, y: self.pos.y + self.get_delta()};
            return Option::None
        }

        if matches!(self.intention, Intention::WALK_TOWARDS_target_id) {
            let closest_ant = self.get_closest_enemy(ants); 
            
            match closest_ant {
                Option::None => (),
                Option::Some(closest_ant) => {
                    self.target_id = closest_ant.id.clone();

                    let delta_x: f64 = closest_ant.pos.x - self.pos.x;
                    let delta_y: f64 = closest_ant.pos.y - self.pos.y;
                    let delta_vector_length: f64 = ((delta_x.powf(2.0) + delta_y.powf(2.0)) as f64).sqrt();
        
                    let normalized_x = match delta_vector_length {
                        0.0 => 0.0,
                        _ => (delta_x / delta_vector_length) as f64
                    }; 
                    let normalized_y = match delta_vector_length {
                        0.0 => 0.0,
                        _ => (delta_y / delta_vector_length) as f64
                    }; 
        
                    self.pos = Point {x: self.pos.x + normalized_x, y: self.pos.y + normalized_y};
                }
            }
            return Option::None
        }
        
        if matches!(self.intention, Intention::ATTACK) {
            // fix this later, brings back entity after attack since all cds are the same atm
            if self.cooldown == 90.0 {
                if self.team == Team::TOP {
                    self.pos.y -= 10.0;
                } else {
                    self.pos.y += 10.0;
                } 
            }
            
            if self.cooldown >= 0.0 {
                self.cooldown -= 1.0;
                return Option::None
            }            
            
            if self.team == Team::TOP {
                self.pos.y += 10.0;
            } else {
                self.pos.y -= 10.0;
            }


            // let mut target_ant =  &mut self.clone();
            // for ant in ants.iter_mut() {
            //     if ant.id == self.target_id {
            //         target_ant = ant;
            //     }
            // }
            self.cooldown = 100.0;
            // self.health -= 20.0;
            // cant dmg for some borrow reason
            // match ants.get(&mut self.target_id) {
            //     Some(ant) => {
            //         ant.health -= 10.0;
            //     },
            //     None => {}
            // }
            return Option::Some(self.target_id.clone());

        }   
        return Option::None;
    }
}


use crate::snake::{get_max_x, get_max_y, set_exit};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone)]
pub struct Snake {
    direction: Direction,
    positions: Vec<(u32, u32)>
}


impl<'a> Snake {
    pub fn new(position: (u32, u32)) -> Snake {
        Snake {
            direction: Direction::RIGHT,
            positions: vec![position],
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_by(&mut self) {

        let next;
        match self.direction {
            Direction::RIGHT if self.positions.first().unwrap().0 < get_max_x() - 1 => {
                next = (self.positions.first().unwrap().0 + 1, self.positions.first().unwrap().1);
            }
            Direction::UP if self.positions.first().unwrap().1 > 1 => {
                next = (self.positions.first().unwrap().0, self.positions.first().unwrap().1 - 1);
            }
            Direction::DOWN if self.positions.first().unwrap().1 < get_max_y() - 1 => {
                next = (self.positions.first().unwrap().0, self.positions.first().unwrap().1 + 1);
            }
            Direction::LEFT if self.positions.first().unwrap().0 > 1 => {
                next = (self.positions.first().unwrap().0 - 1, self.positions.first().unwrap().1);
            }
            _ => {
                set_exit(true);
                next = (0, 0);
            }
        }
        self.positions.insert(0, next);
        self.positions.pop();
    }

    pub fn add_part(&mut self){
        self.positions.push(self.positions.last().unwrap().clone());
    }

    pub fn check_internal_collision(&self) -> bool{
        let cabeza = *self.positions.first().unwrap();
        self.positions.iter().filter(|x| cabeza == **x).count() > 1
    }

    pub fn get_position(&self) -> (u32, u32) {self.positions[0]}

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_positions(&self) -> &Vec<(u32, u32)>{&self.positions}
}

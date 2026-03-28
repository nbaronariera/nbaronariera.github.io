//! Estructura Snake y su lógica de movimiento

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
pub struct Snake {
    direction: Direction,
    positions: Vec<(u32, u32)>,
    max_x: u32,
    max_y: u32,
    hit_wall: bool,
}

impl Snake {
    pub fn new(position: (u32, u32), max_x: u32, max_y: u32) -> Snake {
        Snake {
            direction: Direction::Right,
            positions: vec![position],
            max_x,
            max_y,
            hit_wall: false,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_by(&mut self) {
        let head = *self.positions.first().unwrap();
        let next = match self.direction {
            Direction::Right if head.0 < self.max_x - 1 => (head.0 + 1, head.1),
            Direction::Left if head.0 > 1 => (head.0 - 1, head.1),
            Direction::Up if head.1 > 1 => (head.0, head.1 - 1),
            Direction::Down if head.1 < self.max_y - 1 => (head.0, head.1 + 1),
            _ => {
                self.hit_wall = true;
                (0, 0)
            }
        };

        if !self.hit_wall {
            self.positions.insert(0, next);
            self.positions.pop();
        }
    }

    pub fn add_part(&mut self) {
        if let Some(&last) = self.positions.last() {
            self.positions.push(last);
        }
    }

    pub fn check_internal_collision(&self) -> bool {
        let head = *self.positions.first().unwrap();
        self.positions.iter().skip(1).any(|&pos| pos == head)
    }

    pub fn get_position(&self) -> (u32, u32) {
        self.positions[0]
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn get_positions(&self) -> &[(u32, u32)] {
        &self.positions
    }

    pub fn hit_wall(&self) -> bool {
        self.hit_wall
    }
}

use crate::app::maze::MazeGrid;

pub struct Player {
    pub avatar: String,
    pub x: usize,
    pub y: usize,
    pub speed: f32,
}

impl Movable for Player {
    fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

pub struct PacPerson {
    pub avatar: String,
    pub x: usize,
    pub y: usize,
    pub speed: f32,
}

impl Movable for PacPerson {
    fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

pub trait Movable {
    fn position(&self) -> (usize, usize);

    fn set_position(&mut self, x: usize, y: usize);

    fn try_move(&mut self, dx: isize, dy: isize, maze: &MazeGrid) {
        let (x, y) = self.position();
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x >= 0 && new_y >= 0 {
            let (nx, ny) = (new_x as usize, new_y as usize);
            if maze.in_bounds(nx, ny) && !maze.get_cell(nx, ny).is_wall {
                self.set_position(nx, ny);
            }
        }
    }
}

pub struct Veggie {
    pub avatar: String,
    pub is_eaten: bool,
    pub x: usize,
    pub y: usize,
}

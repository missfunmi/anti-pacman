use crate::app::player::Player;
use rand::rng;
use rand::seq::SliceRandom;

pub struct Veggie {
    pub avatar: String,
    pub is_eaten: bool,
    pub x: usize,
    pub y: usize,
}

pub struct MazeCell {
    pub x: usize,
    pub y: usize,
    pub is_wall: bool,
    pub has_player: bool,
    pub has_pacperson: bool,
    pub has_veggie: bool,
}

pub struct MazeGrid {
    pub cells: Vec<Vec<MazeCell>>,
    pub veggies: Vec<Veggie>,
}

const BOARD_SIZE: usize = 640;
const CELL_COUNT: usize = 8;

impl MazeGrid {
    pub fn get_cell(&self, x: usize, y: usize) -> &MazeCell {
        self.cells.get(y).and_then(|row| row.get(x)).unwrap()
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut MazeCell {
        self.cells
            .get_mut(y)
            .and_then(|row| row.get_mut(x))
            .unwrap()
    }

    /*pub fn move_player(&mut self, from: (usize, usize), to: (usize, usize)) {
            log!(Level::Info, "moving player");
            let to_cell = self.get_cell_mut(to.0, to.1);

            if !to_cell.is_wall {
                to_cell.has_player = true;

                let from_cell = self.get_cell_mut(from.0, from.1);
                from_cell.has_player = false;
            }
        }
    */

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.cells[0].len() && y < self.cells.len()
    }

    pub fn move_player(&mut self, from: (usize, usize), to: (usize, usize)) {
        let (fx, fy) = from;
        let (tx, ty) = to;

        if !self.in_bounds(fx, fy) || !self.in_bounds(tx, ty) {
            return;
        }

        if from == to {
            return;
        }

        // Case 1: moving in the same row
        if fy == ty {
            let row = &mut self.cells[fy];
            if fx != tx {
                // Split so we can get two mutable, non-overlapping references
                let (first, second) = if fx < tx {
                    let (first, second) = row.split_at_mut(tx);
                    (&mut first[fx], &mut second[0])
                } else {
                    let (first, second) = row.split_at_mut(fx);
                    (&mut second[0], &mut first[tx])
                };

                if !second.is_wall {
                    first.has_player = false;
                    second.has_player = true;
                }
            }
        } else {
            // Case 2: moving across rows (safe because they're in different rows)
            let (from_row, to_row) = if fy < ty {
                let (top, bottom) = self.cells.split_at_mut(ty);
                (&mut top[fy], &mut bottom[0])
            } else {
                let (top, bottom) = self.cells.split_at_mut(fy);
                (&mut bottom[0], &mut top[ty])
            };

            let from_cell = &mut from_row[fx];
            let to_cell = &mut to_row[tx];

            if !to_cell.is_wall {
                from_cell.has_player = false;
                to_cell.has_player = true;
            }
        }
    }

    pub fn init() -> Self {
        let cells = (0..CELL_COUNT)
            .map(|y| {
                (0..CELL_COUNT)
                    .map(|x| MazeCell {
                        x,
                        y,
                        is_wall: false,
                        has_player: false,
                        has_pacperson: false,
                        has_veggie: false,
                    })
                    .collect()
            })
            .collect();
        Self {
            cells,
            veggies: Vec::new(),
        }
    }

    pub fn add_walls(&mut self) -> &mut Self {
        // TODO: make this modifiable by level
        let grid = self;
        grid.cells[0][0].is_wall = true;
        grid.cells[0][1].is_wall = true;
        grid.cells[0][2].is_wall = true;
        grid.cells[0][3].is_wall = true;
        grid.cells[0][4].is_wall = true;
        grid.cells[0][5].is_wall = true;
        grid.cells[1][0].is_wall = true;
        grid.cells[1][1].is_wall = true;
        grid.cells[1][2].is_wall = true;
        grid.cells[1][3].is_wall = true;
        grid.cells[1][4].is_wall = true;
        grid.cells[1][5].is_wall = true;
        grid.cells[1][7].is_wall = true;
        grid.cells[2][0].is_wall = true;
        grid.cells[2][1].is_wall = true;
        grid.cells[2][2].is_wall = true;
        grid.cells[2][3].is_wall = true;
        grid.cells[2][4].is_wall = true;
        grid.cells[2][5].is_wall = true;
        grid.cells[2][7].is_wall = true;
        grid.cells[3][0].is_wall = true;
        grid.cells[3][1].is_wall = true;
        grid.cells[3][2].is_wall = true;
        grid.cells[3][3].is_wall = true;
        grid.cells[3][4].is_wall = true;
        grid.cells[3][5].is_wall = true;
        grid.cells[3][7].is_wall = true;
        grid.cells[4][0].is_wall = true;
        grid.cells[4][7].is_wall = true;
        grid.cells[5][0].is_wall = true;
        grid.cells[5][2].is_wall = true;
        grid.cells[5][3].is_wall = true;
        grid.cells[5][4].is_wall = true;
        grid.cells[5][5].is_wall = true;
        grid.cells[5][6].is_wall = true;
        grid.cells[5][7].is_wall = true;
        grid.cells[6][0].is_wall = true;
        grid.cells[6][2].is_wall = true;
        grid.cells[6][3].is_wall = true;
        grid.cells[6][4].is_wall = true;
        grid.cells[6][5].is_wall = true;
        grid.cells[6][6].is_wall = true;
        grid.cells[6][7].is_wall = true;
        grid.cells[7][2].is_wall = true;
        grid.cells[7][3].is_wall = true;
        grid.cells[7][4].is_wall = true;
        grid.cells[7][5].is_wall = true;
        grid.cells[7][6].is_wall = true;
        grid.cells[7][7].is_wall = true;
        grid
    }

    pub fn add_player(&mut self, player: &Player) -> &mut Self {
        let cell = self.get_cell_mut(player.x, player.y);
        cell.has_player = true;
        self
    }

    pub fn walkable_cells(&self) -> Vec<(usize, usize)> {
        self.cells
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(
                    move |(x, cell)| {
                        if !cell.is_wall { Some((x, y)) } else { None }
                    },
                )
            })
            .collect()
    }

    pub fn generate_veggies(&mut self, count: usize) {
        let mut rng = rng();
        let mut candidates = self.walkable_cells();
        candidates.shuffle(&mut rng);

        self.veggies = candidates
            .into_iter()
            .take(count)
            .map(|(x, y)| Veggie {
                avatar: "🥬".to_string(),
                x,
                y,
                is_eaten: false,
            })
            .collect();
    }
}

// fn shuffle<T>(arr: &mut [T]) {
//     let len = arr.len();
//     for i in (1..len).rev() {
//         // Get a random index from 0..=i
//         let j = (Math::random() * ((i + 1) as f64)).floor() as usize;
//         arr.swap(i, j);
//     }
// }

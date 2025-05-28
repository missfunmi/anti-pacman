use crate::app::player::Veggie;
use rand::rng;
use rand::seq::SliceRandom;

pub struct MazeCell {
    pub x: usize,
    pub y: usize,
    pub is_wall: bool,
    pub has_veggie: bool,
}

pub struct MazeGrid {
    pub cells: Vec<Vec<MazeCell>>,
    pub veggies: Vec<Veggie>, // TODO refactor?
}

pub const BOARD_SIZE: usize = 640;
pub const CELL_COUNT: usize = 8;

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

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.cells[0].len() && y < self.cells.len()
    }

    pub fn init() -> Self {
        let cells = (0..CELL_COUNT)
            .map(|y| {
                (0..CELL_COUNT)
                    .map(|x| MazeCell {
                        x,
                        y,
                        is_wall: false,
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

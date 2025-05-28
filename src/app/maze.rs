use crate::app::player::Veggie;
use rand::rng;
use rand::seq::SliceRandom;

pub struct MazeCell {
    pub x: usize,
    pub y: usize,
    pub is_wall: bool,
}

pub struct MazeGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<MazeCell>>,
    pub veggies: Vec<Veggie>, // TODO refactor — probably should not be an attribute of the Maze?
}

pub const BOARD_SIZE: usize = 640;
pub const CELL_COUNT: usize = 8;

impl MazeGrid {
    pub fn from_str(map: &str) -> Self {
        let lines: Vec<&str> = map.trim().lines().collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut cells = vec![];
        for (y, line) in lines.iter().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                row.push(MazeCell {
                    x,
                    y,
                    is_wall: c == '#',
                });
            }
            cells.push(row);
        }

        MazeGrid {
            width,
            height,
            cells,
            veggies: Vec::new(),
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &MazeCell {
        &self.cells[y][x]
    }

    pub fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
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

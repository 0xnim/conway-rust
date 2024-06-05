// Conways game of life

// display using ASCII art in the terminal
// use the crate termion to do this

use termion::terminal_size;

fn main() {
    // ------
    let (width, height) = terminal_size().unwrap();
    let width: usize = width as usize;
    let height: usize = (height - 1) as usize;
    let mut world = World::new(width, height);
    // randomise the initial state of the cells
    for row in &mut world.cells {
        for cell in row {
            cell.state = if rand::random::<bool>() {
                CellState::Alive
            } else {
                CellState::Dead
            };
        }
    }
    loop {
        print!("\x1b[2J\x1b[1;1H");
        world = world.next_state();
        print!("{}", world);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellState {
    Dead,
    Alive,
}
#[derive(Clone)]
struct Cell {
    state: CellState,
}

struct World {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl World {
    fn new(width: usize, height: usize) -> World {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Cell {
                    state: CellState::Dead,
                });
            }
            cells.push(row);
        }
        World {
            width,
            height,
            cells,
        }
    }

    // implement the rules of the game of life
    fn next_state(&self) -> World {
        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_alive_neighbors(x, y);
                let curr_state = self.cells[y][x].state;

                new_cells[y][x].state = match curr_state {
                    CellState::Alive => {
                        if neighbors < 2 || neighbors > 3 {
                            CellState::Dead
                        } else {
                            CellState::Alive
                        }
                    }
                    CellState::Dead => {
                        if neighbors == 3 {
                            CellState::Alive
                        } else {
                            CellState::Dead
                        }
                    }
                };
            }
        }

        World {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let neighbor_state = self.cells[ny as usize][nx as usize].state;
                    if neighbor_state == CellState::Alive {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{}", cell.state)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl CellState {
    fn next_state(&self) -> CellState {
        match self {
            CellState::Dead => CellState::Alive,
            CellState::Alive => CellState::Dead,
        }
    }
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Dead => write!(f, "░"),
            CellState::Alive => write!(f, "█"),
        }
    }
}

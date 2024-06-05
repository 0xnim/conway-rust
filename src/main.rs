// Conways game of life

extern crate libc;

use libc::{fcntl, F_GETFL, F_SETFL, O_NONBLOCK};
use std::io::Stdout;
use std::io::{stdout, Read, Write};
use std::os::unix::io::AsRawFd;
use termion::clear;
use termion::cursor;
use termion::cursor::{Goto, Hide, Show};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::style;
use termion::terminal_size;

fn main() {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    let (width, height) = terminal_size().unwrap();
    let width = width as usize;
    let height = height as usize;
    let mut world = World::new(width, height);

    // Set stdin to non-blocking
    unsafe {
        let fd = std::io::stdin().as_raw_fd();
        let flags = fcntl(fd, F_GETFL, 0);
        if flags != -1 {
            fcntl(fd, F_SETFL, flags | O_NONBLOCK);
        }
    }

    let mut buffer = [0u8; 1];

    let mut on = false;

    world.randomize();
    home(width, height, &mut stdout);
    loop {
        match std::io::stdin().read(&mut buffer) {
            Ok(0) => {
                // std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Ok(_) => {
                match buffer[0] {
                    b'q' => break,              // Exit the loop when 'q' is pressed.
                    b'r' => world.randomize(), // Randomize the world when 'r' is pressed
                    b' ' => on = !on,          // Start/stop the world when 's' is pressed
                    b'h' => {
                        on = false;
                        home(width, height, &mut stdout)}
                    , // Home screen when 'h' is pressed
                    _ => {}                    // Ignore other inputs
                }
            }
            Err(e) => {
                // eprintln!("Error reading from stdin: {}", e);
                // std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        // when user presses q, exit the loop
        if on {
            world.next_generation();
            write!(stdout, "{}{}", clear::All, world).unwrap();
            stdout.flush().unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    quit(stdout);
}

fn home(width: usize, height: usize, stdout: &mut Stdout) {
    let home_text = "q: quit, r: randomize, space: start/stop, h: home";
    let mut home: String; /* "q: quit, r: randomize, s: stop, h: start";*/
    home = String::new();
    // make a screen that is blank and {home_text} in the bottom line
    for _ in 0..height {
        home.push_str("\n");
    }
    home.push_str(home_text);
    let more = width - home_text.len();
    for _ in 0..more {
        home.push_str(" ");
    }

    write!(stdout, "{}", home).unwrap();
    stdout.flush().unwrap();
}

fn quit(mut stdout: RawTerminal<Stdout>) {
    // Reset terminal styles
    write!(stdout, "{}", style::Reset).unwrap();

    // Move cursor to top-left corner
    write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();

    // Clear the screen
    write!(stdout, "{}", clear::All).unwrap();

    // Flush the terminal buffer
    stdout.flush().unwrap();
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", Hide, Goto(1, 1)).unwrap(); // Hide cursor and move to top-left

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y][x];
                let ch = match cell.state {
                    CellState::Dead => "░",
                    CellState::Alive => "█",
                };
                write!(stdout, "{}", ch).unwrap();
            }
            write!(stdout, "{}", Goto(1, y as u16 + 2)).unwrap(); // Move cursor to start of next line
        }

        write!(stdout, "{}", Show).unwrap(); // Show cursor
        stdout.flush().unwrap();
        Ok(())
    }
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

    fn randomize(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y][x].state = if rand::random::<bool>() {
                    CellState::Alive
                } else {
                    CellState::Dead
                };
            }
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
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

    fn next_generation(&mut self) {
        let mut new_cells = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
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

        self.cells = new_cells;
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

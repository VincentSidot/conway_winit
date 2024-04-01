use rand::Rng;
use std::thread;

pub struct Universe {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    new_cells: Vec<Cell>,
    concurent_threads: usize,
    cells_per_thread: usize,
}

impl Universe {
    pub fn new(width: usize, height: usize, alive_probability: f64, concurent_threads: usize) -> Self {
        let mut _self = Self {
            width,
            height,
            cells: Vec::new(),
            new_cells: Vec::new(),
            concurent_threads,
            cells_per_thread: 0,
        };
        _self.init(alive_probability);
        _self
    }

    fn init(&mut self, alive_probability: f64) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.width * self.height {
            self.cells.push(
                Cell::new(
                    rng.gen_bool(alive_probability)
                )
            );
            self.new_cells.push(
                Cell::new(false)
            );
        }
        self.cells_per_thread = self.cells.len() / self.concurent_threads;
    }

    pub fn update(&mut self) {
        if self.concurent_threads == 1 {
            self.update_sync()
        } else {
            self.update_parallel()
        }
    }

    fn update_sync(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = x + y * self.width;
                let count = {
                    let mut count = 0;
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let nx = ((self.width as i32 + x as i32 + dx) % self.width as i32) as usize;
                            let ny = ((self.height as i32 + y as i32 + dy) % self.height as i32) as usize;
                            let idx = nx + ny * self.width;
                            if self.cells[idx].alive {
                                count += 1;
                            }
                        }
                    }
                    count
                };
                self.new_cells[idx] = self.cells[idx].update_neibs(count);
            }
        }
        std::mem::swap(&mut self.cells, &mut self.new_cells);
    }

    fn update_parallel(&mut self) {

        let concurrent_threads = self.concurent_threads;
        // Compute the number of rows each thread will be responsible for
        let cells_per_thread = self.cells_per_thread;

        // Split the cells into slices for each thread to process
        let mut cells = {
            let mut answer = Vec::new();
            let mut cells = self.new_cells.as_mut_slice();
            for _ in 0..concurrent_threads {
                let (start, end) = cells.split_at_mut(cells_per_thread);
                answer.push(start);
                cells = end;
            }
            answer
        };

        let current_cells = self.cells.as_slice();

        let width = self.width;
        let height = self.height;

        // Create a scope for the threads to run in
        thread::scope(|s| {
            
            // Spawn a thread for each slice of cells
            for (i, cells) in cells.iter_mut().enumerate() {
                let base_index = i * cells_per_thread;
                s.spawn(move || {
                    for (i, cell) in cells.iter_mut().enumerate() {
                        let x = (base_index + i) % width;
                        let y = (base_index + i) / width;
                        let count = {
                            let mut count = 0;
                            for dx in -1..=1 {
                                for dy in -1..=1 {
                                    if dx == 0 && dy == 0 {
                                        continue;
                                    }
                                    let nx = ((width as i32 + x as i32 + dx) % width as i32) as usize;
                                    let ny = ((height as i32 + y as i32 + dy) % height as i32) as usize;
                                    let idx = nx + ny * width;
                                    if current_cells[idx].alive {
                                        count += 1;
                                    }
                                }
                            }
                            count
                        };
                        *cell = current_cells[base_index + i].update_neibs(count);
                    }
                });
            }
        });
        std::mem::swap(&mut self.cells, &mut self.new_cells);
    }

    pub fn render(&self, frame: &mut [u8]) {
        // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        //     let x = (i % self.width) as usize;
        //     let y = (i / self.width) as usize;
        //     let idx = self.get_index(x, y);
        //     let color = if self.cells[idx].alive {
        //         ALIVE_COLOR
        //     } else {
        //         DEAD_COLOR
        //     };
        //     pixel.copy_from_slice(&color);
        // }
        debug_assert_eq!(4 * self.cells.len(), frame.len());
        for (cell, pixel) in self.cells.iter().zip(frame.chunks_exact_mut(4)) {
            pixel.copy_from_slice(&cell.get_color());
        }
    }
}

const BIRTH_RULE: [bool; 9] = [false, false, false, true, false, false, false, false, false];
const SURVIVE_RULE: [bool; 9] = [false, false, true, true, false, false, false, false, false];
const DECAY_RATE: f32 = 0.01;

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
    heat: u8
}

impl Cell {
    fn new(alive: bool) -> Self {
        Self {
            alive,
            heat: 0
        }
    }

    fn update_neibs(self, n: usize) -> Self {
        let next_alive = if self.alive {
            SURVIVE_RULE[n]
        } else {
            BIRTH_RULE[n]
        };
        self.next_state(next_alive)
    }

    fn next_state(mut self, alive: bool) -> Self {
        self.alive = alive;
        if self.alive {
            self.heat = 0xff;
        } else {
            let heat = (self.heat as f32 * (1.0 - DECAY_RATE)).clamp(0.0, 255.0);
            self.heat = heat as u8;
        }
        self
    }

    fn get_color(&self) -> [u8; 4] {
        if self.alive {
            [0x0, 0xff, 0xff, 0xff]
        } else {
            [0x0, 0x0, self.heat, 0xff]
        }
    }
}
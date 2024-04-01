# Test Winit

A simple Rust application to demonstrate the Game of Life using the `winit` windowing library and parallel computing.

## Overview

This application simulates Conway's Game of Life, a cellular automaton devised by the British mathematician John Horton Conway. The universe is represented by a grid of cells, where each cell can be either alive or dead. The application uses the `winit` windowing library for the graphical interface and parallel computing for efficient simulation.

## Features

- **Concurrent Simulation**: The universe can be updated either synchronously or in parallel based on the number of concurrent threads specified.
- **Graphics Rendering**: The state of the universe is rendered on the screen using pixels.
- **Random Initialization**: The initial state of the universe is randomly generated based on a given probability.

## Dependencies

- [env_logger](https://crates.io/crates/env_logger) - Logging implementation for Rust.
- [error-iter](https://crates.io/crates/error-iter) - Helper for iterating over error sources.
- [ittapi](https://crates.io/crates/ittapi) - Support for Intel ITT API.
- [log](https://crates.io/crates/log) - Lightweight logging facade for Rust.
- [pixels](https://crates.io/crates/pixels) - A tiny hardware-accelerated pixel frame buffer.
- [rand](https://crates.io/crates/rand) - Random number generation.
- [winit](https://crates.io/crates/winit) - Cross-platform window creation and management in Rust.

## Installation

To build and run the application, follow these steps:

1. Install Rust and Cargo from [here](https://www.rust-lang.org/tools/install).
2. Clone this repository:

    ```bash
    git clone https://github.com/VincentSidot/conway_winit.git
    ```
3. Navigate to the project directory:

    ```bash
    cd conway_winit
    ```
4. Build and run the application:

    ```bash
    cargo run
    ```

## Usage

- Press `Escape` to exit the application.
- The FPS and update time will be displayed in the console.

## Configuration

You can modify the following constants in the `main.rs` file to customize the simulation:

- `WIDTH`: Width of the window.
- `HEIGHT`: Height of the window.
- `THREADS`: Number of concurrent threads for parallel computation.
- `ALIVE_PROBABILITY`: Probability of a cell being alive initially.

## Example

Here's a snippet of the `Universe` struct and its methods:

```rust
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
        // Initialization logic...
    }

    pub fn update(&mut self) -> String {
        if self.concurent_threads == 1 {
            self.update_sync()
        } else {
            self.update_parallel()
        }
    }

    fn update_sync(&mut self) -> String {
        // Synchronous update logic...
    }

    fn update_parallel(&mut self) -> String {
        // Parallel update logic...
    }

    pub fn render(&self, frame: &mut [u8]) {
        // Rendering logic...
    }
}
```
License
This project is licensed under the MIT License. See the LICENSE file for details.
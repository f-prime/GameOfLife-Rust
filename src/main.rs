use rand::prelude::*;
use std::{thread, time};

const CELL_SIZE: i32 = 25;
const GRID_WIDTH: i32 = 200;
const GRID_HEIGHT: i32 = 50;

struct Cell {
    x: i32,
    y: i32,
    on: bool,
}

struct AdjacentCells {
    left: bool,
    right: bool,
    top: bool,
    top_left: bool,
    top_right: bool,
    bottom: bool,
    bottom_left: bool,
    bottom_right: bool,
    num_neighbors: i32,
}

fn rand_num() -> f64 {
    let mut rng = rand::thread_rng();
    let value: f64 = rng.gen();
    value
}

fn create_cell(x: i32, y: i32) -> Cell {
    let value = rand_num();
    let on = value > 0.98;

    Cell {
        x: x * CELL_SIZE,
        y: y * CELL_SIZE,
        on,
    }
}

fn create_grid(x: i32, y: i32) -> Vec<Cell> {
    let mut grid: Vec<Cell>  = Vec::new();

    for i in 0..y {
        for j in 0..x {
            grid.push(
                create_cell(j, i),
            )
        }
    }

    grid
}

fn draw_grid(grid: &Vec<Cell>) {
    let mut last_x = 0;

    for c in grid {
        if c.x < last_x {
            println!("");
        }

        last_x = c.x;

        if c.on {
            print!("#");
        } else {
            print!(" ");
        }
    }
}

fn get_adjacent_cells(grid: &mut Vec<Cell>, index: usize) -> AdjacentCells {
    let mut num_neighbors = 0;

    let is_far_left = index as i32 % GRID_WIDTH == 0;
    let is_far_right = index as i32 % GRID_WIDTH == GRID_WIDTH - 1;
    let is_top = index as i32 - GRID_WIDTH < 0;
    let is_bottom = index as i32 + GRID_WIDTH > grid.len() as i32 - 1;

    let mut left = false;
    let mut right = false;
    let mut top = false;
    let mut bottom = false;
    let mut bottom_right = false;
    let mut bottom_left = false;
    let mut top_left = false;
    let mut top_right = false;

    if !is_far_left {
        let cell = &grid[index - 1];
        left = cell.on;
    }

    if !is_far_right {
        let cell = &grid[index + 1];
        right = cell.on;
    }

    if !is_top {
        let t = &grid[index - GRID_WIDTH as usize];
        top = t.on;

        if !is_far_right {
            let tr = &grid[index - GRID_WIDTH as usize + 1];
            top_right = tr.on;
        }

        if !is_far_left {
            let tl = &grid[index - GRID_WIDTH as usize - 1];
            top_left = tl.on;
        }
    }

    if !is_bottom {
        let b = &grid[index + GRID_WIDTH as usize];
        bottom = b.on;

        if !is_far_left {
            let bl = &grid[index + GRID_WIDTH as usize - 1];
            bottom_left = bl.on;
        }

        if !is_far_right {
            let br = &grid[index + GRID_WIDTH as usize + 1];
            bottom_right = br.on;
        }
    }

    for i in [left, right, top, top_left, top_right, bottom, bottom_left, bottom_right] {
        if i {
            num_neighbors += 1;
        }
    }

    AdjacentCells {
        left,
        right,
        top,
        top_left,
        top_right,
        bottom,
        bottom_left,
        bottom_right,
        num_neighbors,
    }
}

fn step_grid(grid: &mut Vec<Cell>) {
    for i in 0..grid.len() {
        let adjacent = get_adjacent_cells(grid, i);
        let mut cell = &mut grid[i];

        if adjacent.num_neighbors == 3 {
            cell.on = true;    
        } else if adjacent.num_neighbors <= 1 {
            cell.on = false;
        } else if adjacent.num_neighbors >= 5 {
            cell.on = false;
        } 

        /*
        let value = rand_num();
        if value > 0.99 {
            cell.on = !cell.on;
        }
        */
    }
}

fn main() {
    let mut v = create_grid(GRID_WIDTH, GRID_HEIGHT);

    let mut i = 0;

    loop {
        println!("{}", i);
        i += 1;
        draw_grid(&v);
        step_grid(&mut v);
        thread::sleep(time::Duration::from_millis(100));
        println!("\n\n\n\n\n");
    }
}

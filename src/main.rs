use rand::Rng;
use std::{thread, time};
const ROWS: usize = 20;
const COLS: usize = 20;

#[derive(Clone)]
#[derive(Copy)]
struct Cell {
    old_state: u8, //1 for the head, 2 for the trail, 0 for the dead cells
    new_state: u8, // same here
}


enum Directions {
    Left,
    Right,
    Up,
    Down,
    None,
}

fn main() {
    let mut grid = [[Cell {old_state: 0, new_state: 0,}; ROWS]; COLS];
    grid[ROWS/2][COLS/2].old_state = 1; //initial head
    let one_second = time::Duration::from_millis(1000);
    let mut counter = 0;
    loop {
        let direction = match rand::thread_rng().gen_range(1..5) {
            1 => Directions::Left,
            2 => Directions::Right,
            3 => Directions::Up,
            4 => Directions::Down,
            _ => Directions::None,
        };
        print!("\x1b[2J\x1b[0;0H"); //clears the screen on Linux
        print_grid(&grid);
        update_grid(&mut grid, &direction);
        update_states(&mut grid);
        println!("Generations = {}", counter);
        counter = counter + 1;
        thread::sleep(one_second);
    }
}

fn update_grid(arr: &mut [[Cell; ROWS]; COLS], direction: &Directions) {
    for x in 1..ROWS {
        for y in 1..COLS {
            match arr[x][y].old_state {
                1 => match check_neighborhood(arr, x, y) {
                         true => match direction {
                                     Directions::Left => {
                                         arr[x-1][y].new_state = 1;
                                         arr[x][y].new_state = 2;
                                     },
                                     Directions::Right => {
                                         arr[x+1][y].new_state = 1;
                                         arr[x][y].new_state = 2;
                                     },
                                     Directions::Up => {
                                         arr[x][y-1].new_state = 1;
                                         arr[x][y].new_state = 2;
                                     },
                                     Directions::Down => {
                                         arr[x][y+1].new_state = 1;
                                         arr[x][y].new_state = 2;
                                     },
                                     _ => {},
                                 },
                         false => continue,
                     },
                 2 => continue,
                 0 => continue,
                 _ => continue, //unused, it's not possible for a cell to be in this state
            }
        }
    }
}

fn update_states(arr: &mut [[Cell; ROWS];COLS]) {
    for x in 1..ROWS {
        for y in 1..COLS {
            arr[x][y].old_state = arr[x][y].new_state;
        }
    }
}

fn print_grid(arr: &[[Cell; ROWS]; COLS]) {
    for x in 1..ROWS {
        for y in 1..COLS {
            print!("{}", if arr[x][y].old_state == 0 {
                " "
            } else {
                "o"
            })
        }
        print!("\n");
    }
}

fn check_neighborhood(arr: &[[Cell;ROWS];COLS], xindex: usize, yindex: usize) -> bool {
    if xindex > 1 && yindex > 1 && xindex < ROWS - 1 && yindex < COLS - 1 {
        return arr[xindex-1][yindex].old_state != 0 ||
            arr[xindex][yindex-1].old_state != 0 ||
            arr[xindex+1][yindex].old_state != 0 ||
            arr[xindex][yindex+1].old_state != 0 ||
            arr[xindex][yindex].old_state != 0;

    } else {
        return false;
    }
}

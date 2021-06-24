use rand::Rng;
use std::{thread, time};

const ROWS: usize = 50;
const COLS: usize = 50;

enum Directions {
    Left,
    Right,
    Up,
    Down,
    None,
}

fn main() {
    let mut trail: Vec<(usize, usize)> = Vec::new();
    let head = (ROWS/2, COLS/2);
    trail.push(head);
    let interval = time::Duration::from_millis(100);
    let mut generations: usize = 1;
    print!("\x1b[2J");
    loop {
        let direction = match rand::thread_rng().gen_range(1..5) {
            1 => Directions::Left,
            2 => Directions::Right,
            3 => Directions::Up,
            4 => Directions::Down,
            _ => Directions::None,
        };
        //moves the cursor on the top left corner of the screen on linux
        print!("\x1b[0;0H");
        println!("Generations: {}", generations);
        print_grid(&trail);
        trail.push(get_next_head(&trail[trail.len() - 1], &direction));
        thread::sleep(interval);
        generations = generations + 1;
    }
}

fn get_next_head(head: &(usize, usize),
                 direction: &Directions) -> (usize, usize) {
    //function to get the next head of the automaton
    return match direction {
        Directions::Left => (head.0 - 1, head.1),
        Directions::Right => (head.0 + 1, head.1),
        Directions::Up => (head.0, head.1 - 1),
        Directions::Down => (head.0, head.1 + 1),
        _ => (head.0, head.1),
    };
}

fn print_grid(trail: &Vec<(usize, usize)>)
{
    for i in 0..trail.len() {
        print!("\x1b[{};{}Ho", trail[i].1, trail[i].0);
    }
}

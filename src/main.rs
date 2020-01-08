//Brandon Zupan
//Game of Life but in Rust

//Derived from Roger Priebe's 312 assignment #2 in C

use std::fs::File;    //For reading a file
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::convert::TryInto;  //For try_into()


fn main() {
    //edit these two lines and use command line arguments
    let f_name = "world.txt";
    let num_generations = 2; //set to a smaller number for debugging

    //array of strings that will hold the grid
    let mut world: Vec<String> = Vec::new();
    
    let mut num_rows = 0;
    let mut num_cols = 0;

    populate_world(&f_name, &mut world, &mut num_rows, &mut num_cols);

    show_world(&world);

    for _iteration in 0..num_generations {
        
        // code to clear screen goes here
        //print!("\x1B[2J");
        pause();


        iterate_generations(&world, &num_rows, &num_cols);

        show_world(&world);
    }
}

//Funtions

fn populate_world(f_name: &str, world: &mut Vec<String>, num_rows: &mut i32, num_cols: &mut i32) {
    //Fill in the world variable to include the world
    //Open the file and read each line into the world
    //Used https://doc.rust-lang.org/std/io/trait.BufRead.html
    let file = File::open(f_name)
        .expect("Error, could not open file");
    let file = BufReader::new(file);

    //Not sure if this is a good way to do this, error handling is wack
    *num_cols = 0;
    for line in file.lines() {
        let line = line.unwrap();
        //println!("{}", &line);
        let mut column = String::new();
        for space in line.chars() {
            if space == '0' {
                column.push('.');
            } else if space == '1' {
                column.push('*');
            } else {
                panic!("Error: Unrecognized character");
            }
        }
        world.push(String::from(&column));
        *num_cols += 1;
    }

    //Make a usize fit into an i32
    *num_rows = world[0].len().try_into().unwrap();

    //println!("{} rows\n{} cols", &num_rows, &num_cols);


}

fn show_world(world: &Vec<String>) {
    //Prints each line in the world vector
    for line in world {
        println!("{}", line);
    }
}

fn count_neighbors(world: &Vec<String>, row: &i32, col: &i32, num_rows: &i32, num_cols: &i32) -> i32 {
    //Counts the amount of alive neighbors that a cell has and returns it
    //Make sure we are checking a valid spot
    if (col < &0) || (col >= num_cols) || (row < &0) || (row >= num_rows) {
        panic!("Error, tried to count neighbors of invalid spot, col:{} row:{}", col, row);
    }
    let mut neighbors = 0;
    for col_index in -1..2 {
        for row_index in -1..2 {
            if row_index != 0 || col_index != 0 {
                let line = match world.get((col + col_index) as usize) {
                    Some(line) => line,
                    None => continue,
                };
                //Turn it into a vector before working with it
                let mut cells: Vec<char> = Vec::new();
                for character in line.chars() {
                    cells.push(character);
                }
                let cell = match cells.get((row + row_index) as usize) {
                    Some(character) => character,
                    None => continue,
                };
                //println!("Cell {}", cell);
                if *cell == '*' {
                    neighbors += 1;
                }
            }
        }
    }
    return neighbors
}

fn iterate_generations(world: &Vec<String>, num_rows: &i32, num_cols: &i32) {
    //Iterates to the next generation
    let row = 0;
    let col = 0;
    let neighbors = count_neighbors(&world, &row, &col, &num_rows, &num_cols);
    println!("Neighbors: {}", neighbors);
}

fn pause() {
    println!("Press enter to advance...");
    let mut yeet = String::new();   //gets yeeted
    let _ = io::stdin().read_line(&mut yeet).unwrap();
}

//Brandon Zupan
//Game of Life but in Rust

//Derived from Roger Priebe's 312 assignment #2 in C

use std::fs::File;    //For reading a file
use std::io::BufReader;
use std::io::prelude::*;
use std::convert::TryInto;  //For try_into()


fn main() {
    println!("Hello, world!");
    
    //edit these two lines and use command line arguments
    let f_name = "world.txt";
    let num_generations = 2; //set to a smaller number for debugging

    //array of strings that will hold the grid
    let mut world: Vec<String> = Vec::new();
    
    let mut num_rows = 0;
    let mut num_cols = 0;

    populate_world(&f_name, &mut world, &mut num_rows, &mut num_cols);

    //show_world(&world, &num_rows, &num_cols);

    for iteration in 1..num_generations {
        
        // code to clear screen goes here

        //iterate_generations(&world, &num_rows, &num_cols);

        //show_world(&world, &num_rows, &num_cols);
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
        println!("{}", &line);
        world.push(String::from(&line));
        *num_cols += 1;
    }

    //Make a usize fit into an i32
    *num_rows = world[0].len().try_into().unwrap();

    println!("{} rows\n{} cols", &num_rows, &num_cols);


}

//fn show_world(world: &Vec<String>, num_rows: &i32, num_cols: &i32) {
//
//}
//
//fn iterate_generations(world: &Vec<String>, num_rows: &i32, num_cols: &i32) {
//
//}

//Brandon Zupan
//Game of Life but in Rust

//Derived from Roger Priebe's 312 assignment #2 in C




fn main() {
    println!("Hello, world!");
    
    //edit these two lines and use command line arguments
    let f_name = "world.txt";
    let num_generations = 2; //set to a smaller number for debugging

    const MAX_ROWS = 80; //we want the world to fit on one screen
    
    //array of strings that will hold the grid
    let world[&String, MAX_ROWS];
    
    let num_rows = 0;
    let num_cols = 0;

    populate_world(f_name, world, &num_rows, &num_cols);

    show_world(world, num_rows, num_cols);

    for iteration in 1..num_generations {
        
        // code to clear screen goes here

        iterate_generations(world, num_rows, num_cols);

        show_world(world, num_rows, num_cols);
    }
}

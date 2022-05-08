use std::env;

pub fn run (){
    // cli
    // cli is used to create custom commands
    // cli can be created with the cli crate
    let args : Vec<String> = env::args().collect();
    let command = args[1].clone();

    println!("{:?}", command);
}
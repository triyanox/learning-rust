pub fn run (){
    // enums
    // enums are used to create custom types
    // enums can be created with curly braces       
    enum Directions {
        Up,
        Down,
        Left,
        Right
    }
    fn move_direction(dir: Directions) {
        match dir {
            Directions::Up => println!("up"),
            Directions::Down => println!("down"),
            Directions::Left => println!("left"),
            Directions::Right => println!("right")
        }
    }
    move_direction(Directions::Up);
}
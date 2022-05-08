pub fn run (){
    // if statement
    let temp = 10;
    if temp < 20 {
        println!("temp is less than 20");
    } else {
        println!("temp is greater than 20");
    }
    // if else if else
    let temp = 10;
    if temp < 20 {
        println!("temp is less than 20");
    } else if temp == 20 {
        println!("temp is equal to 20");
    } else {
        println!("temp is greater than 20");
    }
    // match
    let temp = 10;
    match temp {
        10 => println!("temp is 10"),
        20 => println!("temp is 20"),
        _ => println!("temp is not 10 or 20")
    }
    // operator precedence
    // AND &&
    // OR ||
    // NOT !
    if temp < 20 && temp >= 10 {
        println!("temp is less than 20 and greater or equal than 10");
    }

}
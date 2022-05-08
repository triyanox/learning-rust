pub fn run (){
    let name = "Peter";
    let mut age = 27;
    println!("My name is {} and I am {} years old", name, age);
    age = 28;
    println!("My name is {} and I am {} years old", name, age);
    // contstants
    const ID: i32 = 1;
    println!("ID is {}", ID);
    // assign multiple variables
    let (x, y) = (1, 2);
    println!("x is {} and y is {}", x, y);
    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is {}", x);
    // shadowing with mutable variables
    let mut x = 5;
    let y = x;
    x = x + 1;
    println!("x is {} and y is {}", x, y);
    // shadowing with immutable variables
    let x = 5;
    let y = x;
    println!("x is {} and y is {}", x, y);
}
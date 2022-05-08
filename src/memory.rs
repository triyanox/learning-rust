// memory management
pub fn run(){
    // heap
    // heap is a memory space where mutable data is stored
    let mut str = String::from("Hello");
    str.push_str(" World");
    println!("{}", str);
    // stack
    // stack is a memory space where immutable data is stored
    let str = String::from("Hello");
    println!("{}", str);
    // ownership
    let str1 = String::from("Hello");
    let str2 = str1;
    println!("{}", str2);
    // ownership with move
    let str1 = String::from("Hello");
    let str2 = String::from("World");
    let str3 = str1 + &str2;
    println!("{}", str3);
    // borrowing
    let str1 = String::from("Hello");
    let str2 = &str1;
    println!("{}", str2);
    // borrowing with move
    let str1 = String::from("Hello");
    let str2 = &str1;
    println!("{}", str2);
}
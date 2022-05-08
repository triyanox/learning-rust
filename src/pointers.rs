pub fn run () {
    // pointers
    // pointers are used to reference data in memory
    // &T is a reference
    // &mut T is a mutable reference
    // *T is a pointer
    // T is a value
    let x = 5;
    let  y = &x;
    println!("{}", y);
    // get the address of x
    println!("{:?}", &x as *const i32);
    // get the value of x
    println!("{}", *y);
    // get the value of y
    println!("{}", *y); 
}
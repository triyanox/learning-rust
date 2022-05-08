pub fn run (){
    // loop
    let mut temp = 10;
    loop {
        println!("temp is {}", temp);
        temp += 1;
        if temp == 20 {
            break;
        }
    }
    // while
    let mut temp = 10;
    while temp < 20 {
        println!("temp is {}", temp);
        temp += 1;
    }
    // for
    for i in 0..10 {
        println!("i is {}", i);
    }
    // for with range
    for i in 0..10 {
        println!("i is {}", i);
    }
    // for with range and step
    for i in (0..10).step_by(2) {
        println!("i is {}", i);
    }
    // for with range and step
    for i in (0..10).rev().step_by(2) {
        println!("i is {}", i);
    }
}
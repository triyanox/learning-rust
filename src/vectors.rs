pub fn run (){
    let my_vector = vec![1, 2, 3, 4, 5];
    println!("{}", my_vector[2]);

    // add to vector
    let mut my_vector = vec![1, 2, 3, 4, 5];
    my_vector.push(6);
    // pop from vector
    my_vector.pop();
    my_vector.push(7);
    println!("{:?}", my_vector);
    // loop through vector
    for i in &my_vector {
        println!("{}", i);
    }
    // loop through vector mutably
    for x in my_vector.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", my_vector);
    
}
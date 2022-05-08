pub fn run (){
    let my_array = [1, 2, 3, 4, 5];
    println!("{}", my_array[2]);
    // fixed size array
    let  my_fixed_array = [0; 5];
    println!("{}", my_fixed_array[2]);
    // array length
    let lenght = my_array.len();
    println!("length is {}", lenght);
    // stack allocated array
    let mut my_stack_array = [0; 5];
    my_stack_array[0] = 1;
    my_stack_array[1] = 2;
    my_stack_array[2] = 3;
    my_stack_array[3] = 4;
    my_stack_array[4] = 5;
    println!("{}", my_stack_array[2]);
    // loop through the array
    for i in 0..my_stack_array.len() {
        println!("{}", my_stack_array[i]);
    }

    // slice
    let slice = &my_stack_array[0..2];
    println!("{:?}", slice);

}
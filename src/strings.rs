pub fn run (){
    let mut s = String::new();
    let data = "initial contents";
    s.push('W');
    println!("{}", s);
    s.pop();
    s.push_str(data);
    println!("{}", s);
    let s = String::from("contents");
    println!("{}", s);
    let length = s.len();
    println!("length is {}", length);
    // loop through the string
    for c in s.chars() {
        println!("{}", c);
    }

    // capacity
    let mut t = String::new();
    let capacity = t.capacity();
    println!("capacity is {}", capacity);
    t.push('a');
    println!("capacity is {}", t.capacity());

    // Assertion testing
    let mut s = String::new();
    assert_eq!(s.capacity(), 0);
    s.push_str("Hello");
    assert_eq!(s.capacity(), 5);
    s.push_str("World");
    assert_eq!(s.capacity(), 10);
  
}

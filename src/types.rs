
pub fn run (){
// integer types
    let x: i8 = 5;
    let y: i16 = 5;
    let z: i32 = 5;
    let i: i64 = 5;
    let j: isize = 5;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
    println!("i is {}", i);
    println!("j is {}", j);
// unsigned integer types
    let x: u8 = 5;
    let y: u16 = 5;
    let z: u32 = 5;
    let i: u64 = 5;
    let j: usize = 5;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
    println!("i is {}", i);
    println!("j is {}", j);
// floating point types
    let x: f32 = 5.0;
    let y: f64 = 5.0;
    println!("x is {}", x);
    println!("y is {}", y);
// boolean types
    let x: bool = true;
    println!("x is {}", x);
// character types
    let x: char = 'a';
    println!("x is {}", x);
// tuple types
    let x: (i32, f64, u8) = (500, 6.4, 1);
    println!("x is {:?}", x);
// array types
    let x: [i32; 5] = [1, 2, 3, 4, 5];
// slice types
    let x: &[i32] = &[1, 2, 3, 4, 5];
// reference types
    let x: &i32 = &5;
    println!("x is {}", x);
// raw pointer types
    let x: *const i32 = &5;
// function types
    let x: fn(i32) -> i32 = |x| x + 1;
    println!("x is {}", x(5));
// tuple struct types
    struct Color(i32, i32, i32);
    let x = Color(0, 255, 255);
// enum types
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
        ChangeFontSize(f32),

    }
    let x = Message::ChangeColor(0, 255, 255);

}
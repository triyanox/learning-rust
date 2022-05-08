pub fn run(){
    // structs
    // structs are used to create custom types
    // struct Color {
    //     red: u8,
    //     green: u8,
    //     blue: u8
    // }
    // structs can be created with curly braces
    // let  c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
    // println!("{}", c.red);

    // structs can be created with parentheses
    // let mut c = color(255, 0, 0);
    // structs can be created with new
    // let mut c = Color::new(255, 0, 0);
    // structs can be created with named fields
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
    // tuple structs
    // tuple structs are used to create custom types that contain multiple values
    // struct color(u8, u8, u8);
    // tuple structs can be created with parentheses
    // let mut c = color(255, 0, 0);
    // tuple structs can be created with new
    // let mut c = Color::new(255, 0, 0);
    // tuple structs can be created with named fields
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0
    // };
//  structs with methods
//  structs can be created with methods
        struct Circle {
            x: f64,
            y: f64,
            radius: f64
        }
        impl Circle {
            // this is a static method
            // static methods are called without using self
            // static methods are called with the struct name
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
            // this is a instance method
            // instance methods are called using self
            // instance methods are called with the struct name
            fn grow(&mut self, increment: f64) {
                self.radius += increment;
            }
            fn position(&self) -> (f64, f64) {
                (self.x, self.y)
            }

        }
        let mut c = Circle {
            x: 0.0,
            y: 0.0,
            radius: 2.0
        };
        c.grow(2.0);
        println!("{}", c.area());
        println!("{}", c.position().0);

}
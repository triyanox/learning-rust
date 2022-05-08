pub fn run (){
    println!("Hello, world!");
    // math
    println!("{}", 1 + 2);
    // formatted output
    println!("This {} {}", "string", "is interpolated");
    // positional arguments
    println!("{0} of {1:b} people know binary, the other half don't", 1, 2);
    // named arguments
    println!("{subject} {verb} {object}", object="binary", subject="One", verb="is");
    // placeholders
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    // debug trait
    println!("{:?}", (12, true, "hello"));
}
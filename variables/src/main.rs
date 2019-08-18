#![allow(unused_variables)]

fn main() {

    let mut temp = 5; // x -> mutable
    println!("The value of temp is: {}", temp);
    temp = 6;
    println!("The value of temp is: {}", temp);
    let temp = temp; // Shadow
    println!("The value of temp is: {}", temp);
    
    let empty = "   ";
    let empty = empty.len();
    println!("Num spaces: {}", spaces);
    
    // String literals are immutable, so they can't be assigned to a mutable typ
    // one can use the format! macro to convert from string literals to heap
    // allocated string buffers.
    // example:
    // greet("Sagnik");
    // This will not work, because "Sagnik" is of type &'static str, where as
    // the greet function expects a String value.
    // To get around this, we can do the following:
    greet(format!("{}", "Sagnik"));
    // format! is similar to fmt.Sprintf() is golang.
}

fn greet(name: String) {
    println!("Hello, {}", name);
}

fn greet(name:&'static  String) {
    primtln!("Hello, {}", name);
}

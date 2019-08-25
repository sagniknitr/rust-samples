enum ColorConversion {
    RGB,
    Gray,
}

struct ColorConversionStruct {
    kind: ColorConversion,
    code: String,
}

enum IpAddr {
    RGB(u8, u8, u8), // We can encode data within the enum itself
    Gray(u8),         // not all variants need to have the same data
                        // each variant can have different types and amounts of associated data.
}

// Rust enums are what are known as Algebraic Data Types (see: Haskell).
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can define methods over enums
impl Message {
    fn call(&self) {
        // method body would be defined here
        // Q: How do we access the values passed in the constructor?
        // Ans: using ref
        // self is borrowed.
        match *self {
            // Message is only accessible by reference, since we have an immutable reference to
            // self.
            // We use 'ref' because we can't move it out of borrowed content.
            // if the function argument was just 'self' instead of '&self', we could just write
            // Message:Write(msg) and move msg out of self.
            Message::Write(ref msg) => {
                println!("Written message: {}", msg);
            }
            Message::Move { x: _, y: _ } => {}
            Message::ChangeColor(_, _, _) => {}
            // The _ pattern will match any value, so the following is not required.
            // Message::Quit => {},
            // Otherwise, checks need to be exhaustive.
            // Using the _ placeholder for match isn't the best idea, because a newly added varian
            // might remain unchecked at different places.
            _ => {}
        }
    }
}

fn main() {
    let conversion_type = ColorConversion::V4; // Variants of an enum are namespaced under its identifier
    println!("Titan!");
    let rgb = ColorConversionStruct {
        kind: ColorConversion::RGB,
        code: String::from("RGB"),
    };

    let gray = ColorConversionStruct {
        kind: ColorConversion::Gray,
        code: String::from("Gray"),
    };
}
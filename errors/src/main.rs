use std::io;
use std::fs::remove_file;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

// Imp Notes:
//
// Two types of error exist in RUST.
//  1. Recoverable (result type)
//  2. Unrecoverable (panic! macro , stops execution)
//
// -> Panic!:
// The program stops execution, unwinds and cleans up the
// stack, and then exits.
// The stacktrace shows the following:
//    0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
//    1: std::panicking::default_hook::{{closure}}
//    2: std::panicking::default_hook
//    3: std::panicking::rust_panic_with_hook
//    4: std::panicking::begin_panic
//
// Question: What is the default hook? How can we insert a custom hook? Is it similar to Golang's
// recover?
//
// In order to directly exit on panic! without the stack unwinding, we have to add the following to
// the config.toml file:
// ```
//  [profile.release]
//  panic = 'abort'
// ```
fn main() {

    // Example: Read a file that is not present.
    // f is of type Result<std::fs::file, std::io::Error>
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file hello.txt, error: {}", error);
        }
    };

    // Example
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        // the if condition after ther Err match specifies a "match guard"
        // ref is need to ensure that error is not moved into this condition, but only referenced
        // by it.
        // In the context of a pattern, & matches a reference and gives us its value, but ref
        // matches a value and gives us a reference to it.
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };


    // Example: 
    // Unwrap returns the contents of Ok if the operation succeeds, otherwise panics.
    let _f = File::open("hello.txt").unwrap();

    // Example:
    // remove_file("hello.txt").unwrap();
    // expect is just like unwrap, except that it panics with the given msg.
    let _f = File::open("hello.txt").expect("Failed to open file");

    // Example
    let name = read_username_from_file().expect("Failed to get username");
    println!("Username is {:?}", name);

}


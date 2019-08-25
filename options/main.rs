    // Options:
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    // Options prevent us from accessing potentially null values.
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // This is not allowed.
    // let sum = x + y;
    // Is option in Rust a monadic type?
    //
    // The if let syntax lets you combine if and let into a less verbose way to handle values that
    // match one pattern and ignore the rest
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        // Note(abhay): It makes more sense to avoid '_' as pattern in match
        _ => (),
    }
    // This is equivalent, though doesn't perform exhaustiveness check at compile time.
    if let Some(3) = some_u8_value {
        println!("three");
    }
struct Info{
    name : String,
    age  : u32,
    company : String
}

impl AddInfo for Info
{
    fn addName(str : String)
    {

    }
}







fn sum(vector: Vec<i32>) -> i32 {
    let mut sum = 0; // mutability

    for item in vector {
        sum = sum + item
    }

    sum
}

fn main() {
    let v = vec![1,2,3];
    let s = sum(v); // check on shallow copy vs move in RUST

    println!("sum: {}", s);
}
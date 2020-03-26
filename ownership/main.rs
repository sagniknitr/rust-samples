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


// No reference , but much more clutter
fn calculate_hike(s: Info) -> (Info, usize) {
    let length = s.len(); 

    (s, length)
}

fn calculate_hike_eff(s: &Info) -> usize {
    let length = s.len(); 
}

fn main() {
    let v = vec![1,2,3];
    let s = sum(v); // check on shallow copy vs move in RUST

    println!("sum: {}", s);
}
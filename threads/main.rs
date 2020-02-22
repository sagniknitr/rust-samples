use std::thread;


fn sample() {
    let handle = thread::spawn(|| for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }
    handle.join();
}


fn cpu_affinity() {

    // Retrieve the IDs of all active CPU cores.
    let core_ids = core_affinity::get_core_ids().unwrap();
 
// Create a thread for each active CPU core.
    let handles = core_ids.into_iter().map(|id| {
    thread::spawn(move || {
        // Pin this thread to a single CPU core.
        core_affinity::set_for_current(id);
        // Do more work after this.
    })
    }).collect::<Vec<_>>();
 
    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

}
fn capture() {
    let v = vec![1, 2, 3];
    let x = 5;
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // drop(v); // This is not valid once v has been moved intot he closure.
    // x can however be used because it is not moved. Why is it not moved?
    println!("{}", x);
    handle.join();
}



fn main() {
}

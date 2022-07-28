// Solution
fn main() {
    let mut vec = Vec::with_capacity(10);

    assert_eq!(vec.len(), 0); // Add 0 because the Vector is emmpty
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10); // Add 10 because the Vector has 10 elements now
    assert_eq!(vec.capacity(), 10); // Add 10 because the capacity is still the same

    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);

    let mut vec = Vec::with_capacity(100); // Add hundred as a capacity so it is enough for the element addition in the for loop
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100); // Add 100 because we have 100 elements now
    assert_eq!(vec.capacity(), 100); // Add 100 because the capacity is the same
    
    println!("Success!")
}

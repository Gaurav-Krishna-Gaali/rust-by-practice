// Solution

fn main() {
    let a = [4, 3, 2, 1];

    // Interate throught the index and a elements
    // Add .iter() to make it iterateable and .enumerate to get index
    for (i,v) in a.iter().enumerate() { 
        println!("The {}th element is {}",i+1,v);
    }
}


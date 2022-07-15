// Solution

// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           // Continue and for for the next iteration
           continue; 
       }
        //Break here
       break;
    }

    assert_eq!(n, 66);

    println!("Success!");
}


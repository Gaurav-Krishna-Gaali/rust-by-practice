// Solution


fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    //  .iter()  make names iterable
    for name in names.iter() { 
        // Can do anything with name
    }

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Can do anything with name
    }
    
    println!("{:?}", numbers);
} 


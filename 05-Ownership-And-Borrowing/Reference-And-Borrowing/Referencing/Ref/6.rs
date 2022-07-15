
// Solution
fn main() {
    let c = '中';

    let r1 = &c;
    // ref - reference to c
    let ref r2 = c; 
    assert_eq!(*r1, *r2);
    
    //  equality of the two address 
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// Solution

// x will be equal to 3 and y will be to 2

fn main() {
    let (x, y);

    (x, ..) = (3, 4); 
    [.., y] = [1, 2]; 

    assert_eq!([x, y], [3, 2]); 

    println!("Success!");
}

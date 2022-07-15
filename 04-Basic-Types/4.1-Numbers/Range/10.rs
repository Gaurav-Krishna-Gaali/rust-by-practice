
//  Solution

fn main() {
    // 5 is not included in the range.
    assert_eq!((1..5), Range{ start: 1, end: 5 }); 
    // =5 to include the range
    assert_eq!((1..=5_), RangeInclusive::new(1, 5)); 

    println!("Success!");
}

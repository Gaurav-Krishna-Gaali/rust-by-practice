// Solution
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];

    // Use v.len() to get the length of the Vec v, we could also leave the end of the range which is v.len() too. so [0..] is also valid, and since we are beginning at the beginning, we can leave the start  of the range too. So [..] is a better solution
    let slice2 = &v[0..v.len()];
    
    assert_eq!(slice1, slice2);
    
    // slice are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    // Make the slice include 4. => make the range till the end of the Vector
    let slice3 = &mut v[0..]; 

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!")
}


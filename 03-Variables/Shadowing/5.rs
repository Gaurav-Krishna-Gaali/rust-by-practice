
// Solution


fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        // Changed value to twelve because x shadowed prev
        assert_eq!(x, 12); 
    }

     // x = 5  in this scope. bracket ends the scope.
    assert_eq!(x, 5);
    let x = 42;
    println!("{x}");
}

// Solution
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    let v = Color(0, 127, 255); 
    // Change struct name to Color as we are checking color,
    //   add values that do not conflict with assert!
    
    check_color(v);

    println!("Success!");
}   

fn check_color(p: Color) {
    let Color(x, _, _) = p; // Add Color struct 
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
 }

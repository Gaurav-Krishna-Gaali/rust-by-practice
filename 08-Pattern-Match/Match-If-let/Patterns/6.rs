
// Solution
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
         // Remove &mut because r is already &mut reference to v
       value => value.push_str(" world!")
    }
}

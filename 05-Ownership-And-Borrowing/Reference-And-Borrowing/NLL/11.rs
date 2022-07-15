// Solution

fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("I code errors  {}", r1) 
}


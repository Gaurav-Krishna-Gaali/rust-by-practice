
// Solution
struct Unit;
trait SomeTrait {
    // ...Some behaviors defined here.
}

impl SomeTrait for Unit {  }
fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
} 

fn do_something_with_unit(u: Unit) {   } // Take in Unit struct as parameter

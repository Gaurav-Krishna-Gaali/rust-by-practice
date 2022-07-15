// Solution
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a { // Add if let statement for the Bar variant
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}

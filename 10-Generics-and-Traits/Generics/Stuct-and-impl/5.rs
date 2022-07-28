// Solution

// Make Val generic
struct Val<T> {
    // Use generic type T instead of a concrete type
    val: T, 
}

impl<T> Val<T> { 
    fn value(&self) -> &T { 
        //  function returns generic type T , so change it to &T
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}
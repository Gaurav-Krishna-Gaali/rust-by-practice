// Solution
struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Give struct S as parameter because reg_fn takes type S which takes type A
    reg_fn(S(A)); 
    // Give generic struct SGen as parameter and SGen with value of struct A
    gen_spec_t(SGen(A)); 
    // Give generic struct SGen as parameter and SGen with a random i32 integer
    gen_spec_i32(SGen(114)); 
    // Give SGen with char value as parameter because we explicilty told the generic function that we are expecting a character
    generic::<char>(SGen('c')); 
    // Give generic struct SGen as parameter and SGen should have a char value 
    generic(SGen('c')); 

    println!("Success!");

}
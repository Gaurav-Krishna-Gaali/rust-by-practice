// Solution
struct Array<T, const N: usize> {
    data : [T; N]
}

fn main() {
    let arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            // Change to integers
            data: [1, 2, 3], 
        },
        Array {
            // Add another integer to make the length 3
        }
            data: [1, 2, 10] 
    ];

    println!("Success!");
}
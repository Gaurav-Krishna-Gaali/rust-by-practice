// Solution

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
} 

fn show_message(msg: Message) {
    match msg {
        Message::Move{x, y} => { // Add values as a struct
            assert_eq!(x, 1); // Change a to x
            assert_eq!(y, 3); // Change b to y so that they both match the value keys
        },
        Message::ChangeColor(r, g, b) => { // Add r
            assert_eq!(g, 255); // Add 255
            assert_eq!(b, 0); // Add 0
        }
        _ => println!("no data in these variants")
    }
}


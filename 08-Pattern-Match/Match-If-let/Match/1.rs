
// Solution

// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        // Add South and North variants with the or operator
        Direction::South | Direction::North => { 
            println!("South or North");
        },
        _ => println!("West"),
    };
}


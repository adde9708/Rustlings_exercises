// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // Define a few types of messages
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

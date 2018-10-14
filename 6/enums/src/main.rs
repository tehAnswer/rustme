enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u32, u32, u32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => { println!("I quit.") },
            Message::Move { x, y } => {
                println!("I move to ({},{})", x, y);
            },
            Message::Write(ref text) => {
                println!("Write: '{}'", text);
            },
            Message::ChangeColor(ref u1, u2, u3) => {
                println!("Changing color to ({}, {}, {})...", u1, u2, u3)
            }
        }
    }
}
fn main() {
    let msg1 = Message::Move { x: 0, y :0 };
    let msg2 = Message::Write(String::from("xd"));
    let msg3 = Message::ChangeColor(11, 100, 200);
    let msg4 = Message::Quit;
    msg1.call();
    msg2.call();
    msg3.call();
    msg4.call();
}

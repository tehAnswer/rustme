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
            Message::Write(text) => {
                println!("Write: '{}'", text);
            },
            Message::ChangeColor(u1, u2, u3) => {
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

    let first_msg = Some(msg1);
    let last_msg = Some(msg4);

    if let Some(Message::Move { x: 1, y: 1}) = first_msg {
        println!("You started by moving to (1,1)");
    } else {
        println!("God. Is. A. Crab.");
    }

    if let Some(Message::Quit) = last_msg {
        println!("/resign");
    } else {
        println!("Not yet.");
    }


}

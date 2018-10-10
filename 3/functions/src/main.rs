fn main() {
    println!("Hello, world!");
    another_function(1, 2);
    let x = 5;
    let y = {
        let x = 3;
        // This line shouldn't end with a semicolon because
        // then it becomes a sentence, which doesn't yield
        // any value.
        x + 1
    };
    println!("{} plus one is {}", 511, plus_one(511));
    println!("x:{} y:{}", x, y);
}


fn plus_one(x: u32) -> u32 {
    x + 1
}

fn another_function(x: u32, y: u32) {
    println!("Bye {}", x + y);
}

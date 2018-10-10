fn main() {
    let array_one = [1];
    let array_two = array_one;

    println!("{} {}", array_one[0], array_two[0]);

    // Fails because the variable has been moved.
    // let vector_one = Vec::new();
    // vector_one.push(1);
    // let vector_two = vector_one;
    // println!("{} {}", vector_one[0], vector_two[0]);

    let greet_one = String::from("What's good?");
    takes_ownership(greet_one);
    // println!(greet_one)

    let greet_two = 2;
    makes_copy_int(greet_two);
    println!("{}", greet_two);

    let greet_three = "3";
    makes_copy_str(greet_three);
    println!("{}", greet_three);

    let greet_four = String::from("4");
    let greet_four = takes_and_gives_back(greet_four);
    println!("{}", greet_four);
}

fn makes_copy_int(i : u32) {
    println!("made a copy of {}", i);
}

fn makes_copy_str(s : &str) {
    println!("made a copy of {}", s);
}

fn takes_ownership(s : String) {
    println!("took ownership of {}, calling `drop`", s);
}

fn takes_and_gives_back(s: String) -> (String) {
    s
}

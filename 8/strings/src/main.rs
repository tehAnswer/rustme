fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5;
    // NOTE: s4 has been moved, it can't be used anymore
    println!("s6 is {}", s6);

    let s7 = "Puta";
    let s8 = "Bida";
    let s9 = format!("{} {}", s7, s8);
    println!("s7 is {}", s7);
    println!("s8 is {}", s8);
    println!("s9 is {}", s9);

    let s10 = "tic";
    let s11 = "tac";
    let s12 = add(&s10, &s11)
    println!("s12: {}", s12);
}

fn add(s1: &str, s2: &str) -> String {
    // NOTE: This doesn't work: binary operation
    // `+` with `&str`
    // s1 + "-" + s2
}
